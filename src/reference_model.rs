//! Deterministic, source-grounded reference and citation baseline.
//!
//! This module deliberately does not construct CSL, query providers, or
//! assert bibliographic truth. It is a neutral evidence producer for later
//! adapters and review queues.

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const REFERENCE_MODEL_SCHEMA_VERSION: &str = "citeweft.reference-model.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceSpan {
    pub start: usize,
    pub end: usize,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReferenceCandidate {
    pub id: String,
    pub raw_text: String,
    pub span: SourceSpan,
    pub fields: ReferenceFields,
    pub confidence: f32,
    pub status: ExtractionStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ReferenceFields {
    pub authors: Option<FieldEvidence>,
    pub title: Option<FieldEvidence>,
    pub year: Option<FieldEvidence>,
    pub doi: Option<FieldEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldEvidence {
    pub value: String,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtractionStatus {
    Extracted,
    Review,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CitationCallout {
    pub id: String,
    pub surface: String,
    pub span: SourceSpan,
    pub reference_id: Option<String>,
    pub confidence: f32,
    pub status: ExtractionStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelDiagnostic {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferenceModelProvenance {
    pub backend: String,
    pub version: String,
    pub configuration: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReferenceModelReport {
    pub schema_version: String,
    pub provenance: ReferenceModelProvenance,
    pub references: Vec<ReferenceCandidate>,
    pub callouts: Vec<CitationCallout>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<ModelDiagnostic>,
}

#[derive(Debug, Error)]
pub enum ReferenceModelError {
    #[error("reference model input exceeds {limit} bytes")]
    InputTooLarge { limit: usize },
    #[error("reference model input is not valid UTF-8")]
    InvalidUtf8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeterministicReferenceModel {
    pub version: String,
}

pub const DEFAULT_REFERENCE_MODEL_MAX_INPUT_BYTES: usize = 25 * 1024 * 1024;

impl Default for DeterministicReferenceModel {
    fn default() -> Self {
        Self {
            version: "deterministic-baseline-v1".to_string(),
        }
    }
}

impl DeterministicReferenceModel {
    pub fn extract(&self, input: &[u8]) -> Result<ReferenceModelReport, ReferenceModelError> {
        if input.len() > DEFAULT_REFERENCE_MODEL_MAX_INPUT_BYTES {
            return Err(ReferenceModelError::InputTooLarge {
                limit: DEFAULT_REFERENCE_MODEL_MAX_INPUT_BYTES,
            });
        }
        let text = std::str::from_utf8(input).map_err(|_| ReferenceModelError::InvalidUtf8)?;
        let section = find_reference_section(text);
        let (references, section_start, mut diagnostics) = match section {
            Some(start) => (segment_entries(text, start), start, Vec::new()),
            None => (
                Vec::new(),
                text.len(),
                vec![ModelDiagnostic {
                    code: "reference_section_not_found".to_string(),
                    message: "no References or Bibliography heading was found".to_string(),
                }],
            ),
        };
        let callouts = extract_numeric_callouts(text, section_start, &references);
        if references.is_empty() && section.is_some() {
            diagnostics.push(ModelDiagnostic {
                code: "reference_entries_not_found".to_string(),
                message: "reference heading was found but no entries were segmented".to_string(),
            });
        }
        Ok(ReferenceModelReport {
            schema_version: REFERENCE_MODEL_SCHEMA_VERSION.to_string(),
            provenance: ReferenceModelProvenance {
                backend: "deterministic-reference-baseline".to_string(),
                version: self.version.clone(),
                configuration: "numbered-entries-numeric-callouts".to_string(),
            },
            references,
            callouts,
            diagnostics,
        })
    }
}

fn find_reference_section(text: &str) -> Option<usize> {
    line_ranges(text).find_map(|(start, end)| {
        let heading = text[start..end]
            .trim()
            .trim_end_matches(':')
            .to_ascii_lowercase();
        if matches!(
            heading.as_str(),
            "references" | "bibliography" | "works cited"
        ) {
            let newline_bytes = text[end..]
                .chars()
                .take_while(|character| *character == '\r' || *character == '\n')
                .map(char::len_utf8)
                .sum::<usize>();
            Some(end + newline_bytes)
        } else {
            None
        }
    })
}

fn segment_entries(text: &str, section_start: usize) -> Vec<ReferenceCandidate> {
    let mut entries: Vec<(usize, usize)> = Vec::new();
    for (line_start, line_end) in line_ranges(&text[section_start..]) {
        let start = section_start + line_start;
        let end = section_start + line_end;
        let line = &text[start..end];
        if line.trim().is_empty() {
            continue;
        }
        if entry_marker_length(line).is_some() || entries.is_empty() {
            entries.push((start, end));
        } else if let Some(entry) = entries.last_mut() {
            entry.1 = end;
        }
    }
    entries
        .into_iter()
        .enumerate()
        .map(|(index, (start, end))| parse_entry(&text[start..end], start, index + 1))
        .collect()
}

fn parse_entry(raw: &str, start: usize, ordinal: usize) -> ReferenceCandidate {
    let marker_length = entry_marker_length(raw).unwrap_or(0);
    let content_offset = raw[..marker_length].len();
    let content = raw[content_offset..].trim();
    let content_start = start + content_offset + raw[content_offset..].len()
        - raw[content_offset..].trim_start().len();
    let span = SourceSpan {
        start,
        end: start + raw.len(),
        text: raw.to_string(),
    };
    let year = find_year(content, content_start);
    let doi = find_doi(content, content_start);
    let authors = year.as_ref().and_then(|evidence| {
        let prefix = content[..evidence.span.start - content_start]
            .trim()
            .trim_end_matches('(')
            .trim_end()
            .trim_end_matches('.')
            .trim();
        (!prefix.is_empty()).then(|| evidence_for(prefix, content_start, content))
    });
    let title = find_title(content, content_start, year.as_ref());
    let confidence = match (&title, &year) {
        (Some(_), Some(_)) => 0.9,
        (Some(_), None) | (None, Some(_)) => 0.65,
        (None, None) => 0.4,
    };
    ReferenceCandidate {
        id: format!("r{ordinal}"),
        raw_text: raw.to_string(),
        span,
        fields: ReferenceFields {
            authors,
            title,
            year,
            doi,
        },
        confidence,
        status: if confidence >= 0.8 {
            ExtractionStatus::Extracted
        } else {
            ExtractionStatus::Review
        },
    }
}

fn entry_marker_length(line: &str) -> Option<usize> {
    let trimmed_start = line.len() - line.trim_start().len();
    let bytes = line.as_bytes();
    let mut index = trimmed_start;
    if bytes.get(index) == Some(&b'[') {
        index += 1;
        let digits = index;
        while bytes.get(index).is_some_and(|byte| byte.is_ascii_digit()) {
            index += 1;
        }
        if index > digits && bytes.get(index) == Some(&b']') {
            return Some(index + 1);
        }
    } else {
        let digits = index;
        while bytes.get(index).is_some_and(|byte| byte.is_ascii_digit()) {
            index += 1;
        }
        if index > digits && matches!(bytes.get(index), Some(b'.' | b')')) {
            return Some(index + 1);
        }
    }
    None
}

fn find_year(content: &str, absolute_start: usize) -> Option<FieldEvidence> {
    for (index, window) in content.as_bytes().windows(4).enumerate() {
        if window.iter().all(|byte| byte.is_ascii_digit()) {
            let value = &content[index..index + 4];
            if (1600..=2100).contains(&value.parse::<u16>().ok()?) {
                return Some(evidence_for(value, absolute_start + index, content));
            }
        }
    }
    None
}

fn find_doi(content: &str, absolute_start: usize) -> Option<FieldEvidence> {
    let index = content.find("10.")?;
    let remainder = &content[index..];
    let end = remainder
        .char_indices()
        .find(|(_, character)| character.is_whitespace() || matches!(character, ']' | ';' | ','))
        .map(|(offset, _)| offset)
        .unwrap_or(remainder.len());
    let value = remainder[..end].trim_end_matches('.');
    (value.len() > 3).then(|| evidence_for(value, absolute_start + index, content))
}

fn find_title(
    content: &str,
    absolute_start: usize,
    year: Option<&FieldEvidence>,
) -> Option<FieldEvidence> {
    let start = year
        .map(|evidence| evidence.span.start + 4 - absolute_start)
        .unwrap_or(0);
    let remainder = content
        .get(start..)?
        .trim_start_matches([' ', '.', '(', ')']);
    let index = content.find(remainder)?;
    let value = remainder
        .split_once(". ")
        .map(|(title, _)| title)
        .unwrap_or(remainder)
        .trim_end_matches('.');
    (!value.is_empty()).then(|| evidence_for(value, absolute_start + index, content))
}

fn evidence_for(value: &str, absolute_start: usize, _source: &str) -> FieldEvidence {
    FieldEvidence {
        value: value.to_string(),
        span: SourceSpan {
            start: absolute_start,
            end: absolute_start + value.len(),
            text: value.to_string(),
        },
    }
}

fn extract_numeric_callouts(
    text: &str,
    section_start: usize,
    references: &[ReferenceCandidate],
) -> Vec<CitationCallout> {
    let body = &text[..section_start];
    let mut callouts = Vec::new();
    let bytes = body.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] != b'[' {
            index += 1;
            continue;
        }
        let digits_start = index + 1;
        let mut end = digits_start;
        while end < bytes.len() && bytes[end].is_ascii_digit() {
            end += 1;
        }
        if end == digits_start || end >= bytes.len() || bytes[end] != b']' {
            index += 1;
            continue;
        }
        let surface = &body[index..=end];
        let ordinal = body[digits_start..end].parse::<usize>().unwrap_or(0);
        let reference_id =
            (ordinal > 0 && ordinal <= references.len()).then(|| format!("r{ordinal}"));
        callouts.push(CitationCallout {
            id: format!("c{}", callouts.len() + 1),
            surface: surface.to_string(),
            span: SourceSpan {
                start: index,
                end: end + 1,
                text: surface.to_string(),
            },
            confidence: if reference_id.is_some() { 1.0 } else { 0.0 },
            status: if reference_id.is_some() {
                ExtractionStatus::Extracted
            } else {
                ExtractionStatus::Review
            },
            reference_id,
        });
        index = end + 1;
    }
    callouts
}

fn line_ranges(text: &str) -> impl Iterator<Item = (usize, usize)> + '_ {
    let mut start = 0;
    text.split_inclusive('\n').map(move |line| {
        let end = start + line.trim_end_matches('\n').trim_end_matches('\r').len();
        let range = (start, end);
        start += line.len();
        range
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = "A claim [1].\n\nReferences:\n[1] Ada Lovelace. (2026). A fixture reference. Fixture Journal. doi:10.1000/fixture.";

    #[test]
    fn deterministic_model_segments_fields_and_links_callout() {
        let report = DeterministicReferenceModel::default()
            .extract(FIXTURE.as_bytes())
            .expect("extract fixture");
        assert_eq!(report.schema_version, REFERENCE_MODEL_SCHEMA_VERSION);
        assert_eq!(report.references.len(), 1);
        assert_eq!(report.references[0].id, "r1");
        assert_eq!(
            report.references[0].fields.year.as_ref().unwrap().value,
            "2026"
        );
        assert_eq!(report.callouts[0].reference_id.as_deref(), Some("r1"));
        assert!(report.references[0].span.start < report.references[0].span.end);
    }

    #[test]
    fn unknown_callout_abstains_instead_of_forcing_a_link() {
        let report = DeterministicReferenceModel::default()
            .extract(b"A claim [9].\n\nReferences:\n[1] Incomplete entry.")
            .expect("extract fixture");
        assert_eq!(report.callouts[0].reference_id, None);
        assert_eq!(report.callouts[0].status, ExtractionStatus::Review);
    }

    #[test]
    fn invalid_utf8_is_rejected() {
        assert!(matches!(
            DeterministicReferenceModel::default().extract(&[0xff]),
            Err(ReferenceModelError::InvalidUtf8)
        ));
    }

    #[test]
    fn oversized_input_is_rejected_before_segmentation() {
        let input = vec![b'a'; DEFAULT_REFERENCE_MODEL_MAX_INPUT_BYTES + 1];
        assert!(matches!(
            DeterministicReferenceModel::default().extract(&input),
            Err(ReferenceModelError::InputTooLarge { .. })
        ));
    }
}
