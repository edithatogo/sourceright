use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntakeDocument {
    pub source: String,
    pub kind: IntakeSourceKind,
    pub text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IntakeSourceKind {
    PastedText,
    PlainText,
    Markdown,
    Docx,
    PdfText,
    ScannedPdf,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntakeResult {
    pub references: Vec<ReferenceCandidate>,
    pub citations: Vec<InTextCitationCandidate>,
    pub diagnostics: Vec<IntakeDiagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferenceCandidate {
    pub id: String,
    pub text: String,
    pub source: String,
    pub span: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InTextCitationCandidate {
    pub text: String,
    pub source: String,
    pub span: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntakeDiagnostic {
    pub code: String,
    pub message: String,
}

pub fn extract_intake(document: &IntakeDocument) -> IntakeResult {
    match document.kind {
        IntakeSourceKind::PastedText | IntakeSourceKind::PlainText => {
            extract_text_like(&document.source, &document.text, None)
        }
        IntakeSourceKind::Markdown => {
            let reference_start = reference_section_start(&document.text);
            extract_text_like(&document.source, &document.text, reference_start)
        }
        IntakeSourceKind::Docx if !document.text.trim().is_empty() => {
            let reference_start = reference_section_start(&document.text);
            let mut result = extract_text_like(&document.source, &document.text, reference_start);
            result.diagnostics.push(IntakeDiagnostic {
                code: "intake.docx.adapter_text_used".to_string(),
                message: "DOCX adapter supplied extracted text; Sourceright preserved the source as DOCX provenance.".to_string(),
            });
            result
        }
        IntakeSourceKind::Docx => unsupported(
            "intake.docx.requires_extractor",
            "DOCX extraction requires an adapter; plain text extracted from DOCX can be passed as text.",
        ),
        IntakeSourceKind::PdfText if !document.text.trim().is_empty() => {
            let reference_start = reference_section_start(&document.text);
            let mut result = extract_text_like(&document.source, &document.text, reference_start);
            result.diagnostics.push(IntakeDiagnostic {
                code: "intake.pdf.text_layer_used".to_string(),
                message: "PDF text-layer adapter supplied extracted text; source spans are line-based in the extracted text.".to_string(),
            });
            result
        }
        IntakeSourceKind::PdfText => unsupported(
            "intake.pdf.requires_text_layer_extractor",
            "PDF text extraction requires an adapter; extracted text can be passed as plain text.",
        ),
        IntakeSourceKind::ScannedPdf => unsupported(
            "intake.ocr.required",
            "Scanned PDF intake requires an OCR adapter before reference segmentation.",
        ),
    }
}

pub fn extract_references_from_text(source: &str, text: &str) -> Vec<ReferenceCandidate> {
    extract_references_from_text_with_base_line(source, text, 1)
}

fn extract_references_from_text_with_base_line(
    source: &str,
    text: &str,
    base_line: usize,
) -> Vec<ReferenceCandidate> {
    reference_entries(text, base_line, false)
        .entries
        .into_iter()
        .enumerate()
        .map(|(index, (line_number, text))| ReferenceCandidate {
            id: format!("ref-{:04}", index + 1),
            text,
            source: source.to_string(),
            span: format!("line:{line_number}"),
        })
        .collect()
}

pub fn extract_in_text_citations(source: &str, text: &str) -> Vec<InTextCitationCandidate> {
    let mut citations = Vec::new();

    for (line_index, line) in text.lines().enumerate() {
        let mut cursor = 0;
        while let Some(start) = line[cursor..].find('(') {
            let absolute_start = cursor + start;
            let Some(end) = line[absolute_start..].find(')') else {
                break;
            };
            let absolute_end = absolute_start + end + 1;
            let candidate = &line[absolute_start..absolute_end];
            if looks_like_citation(candidate) {
                citations.push(InTextCitationCandidate {
                    text: candidate.to_string(),
                    source: source.to_string(),
                    span: format!(
                        "line:{}:{}-{}",
                        line_index + 1,
                        absolute_start + 1,
                        absolute_end
                    ),
                });
            }
            cursor = absolute_end;
        }
    }

    citations
}

fn extract_text_like(source: &str, text: &str, reference_start: Option<usize>) -> IntakeResult {
    let (reference_text, base_line) = reference_start
        .map(|start| {
            (
                text.lines().skip(start).collect::<Vec<_>>().join("\n"),
                start + 1,
            )
        })
        .unwrap_or_else(|| (text.to_string(), 1));
    let reference_entries =
        reference_entries(&reference_text, base_line, reference_start.is_some());

    IntakeResult {
        references: reference_entries
            .entries
            .into_iter()
            .enumerate()
            .map(|(index, (line_number, text))| ReferenceCandidate {
                id: format!("ref-{:04}", index + 1),
                text,
                source: source.to_string(),
                span: format!("line:{line_number}"),
            })
            .collect(),
        citations: extract_in_text_citations(source, text),
        diagnostics: reference_entries.diagnostics,
    }
}

struct ReferenceEntryParse {
    entries: Vec<(usize, String)>,
    diagnostics: Vec<IntakeDiagnostic>,
}

fn reference_entries(text: &str, base_line: usize, emit_diagnostics: bool) -> ReferenceEntryParse {
    let mut entries = Vec::new();
    let mut diagnostics = Vec::new();
    let mut current = String::new();
    let mut start_line = base_line;
    let mut current_started_with_marker = false;

    for (line_index, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            push_entry(
                &mut entries,
                &mut diagnostics,
                start_line,
                &mut current,
                current_started_with_marker,
                emit_diagnostics,
            );
            start_line = base_line + line_index + 1;
            current_started_with_marker = false;
            continue;
        }

        if let Some(stripped) = strip_reference_marker(trimmed) {
            if !current.trim().is_empty() {
                push_entry(
                    &mut entries,
                    &mut diagnostics,
                    start_line,
                    &mut current,
                    current_started_with_marker,
                    emit_diagnostics,
                );
            }
            start_line = base_line + line_index;
            current_started_with_marker = true;
            current.push_str(stripped);
            continue;
        }

        if !current.trim().is_empty()
            && !current_started_with_marker
            && looks_like_reference(current.trim())
            && looks_like_reference(trimmed)
        {
            push_entry(
                &mut entries,
                &mut diagnostics,
                start_line,
                &mut current,
                current_started_with_marker,
                emit_diagnostics,
            );
            start_line = base_line + line_index;
        }

        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(trimmed);
    }

    push_entry(
        &mut entries,
        &mut diagnostics,
        start_line,
        &mut current,
        current_started_with_marker,
        emit_diagnostics,
    );
    ReferenceEntryParse {
        entries,
        diagnostics,
    }
}

fn push_entry(
    entries: &mut Vec<(usize, String)>,
    diagnostics: &mut Vec<IntakeDiagnostic>,
    start_line: usize,
    current: &mut String,
    started_with_marker: bool,
    emit_diagnostics: bool,
) {
    let entry = current.trim();
    if !entry.is_empty() && looks_like_reference(entry) {
        entries.push((start_line, strip_reference_marker_owned(entry)));
    } else if !entry.is_empty() && started_with_marker && emit_diagnostics {
        diagnostics.push(IntakeDiagnostic {
            code: "intake.references.malformed_entry".to_string(),
            message: format!(
                "Bibliography entry starting on line {start_line} did not match reference heuristics and was left for manual review."
            ),
        });
    }
    current.clear();
}

fn strip_reference_marker_owned(entry: &str) -> String {
    reference_marker_body(entry).map_or_else(|| entry.to_string(), |body| body.to_string())
}

fn strip_reference_marker(entry: &str) -> Option<&str> {
    reference_marker_body(entry)
}

fn reference_marker_body(entry: &str) -> Option<&str> {
    let entry = entry.trim_start();
    if let Some(rest) = entry.strip_prefix("- ") {
        return Some(rest.trim_start());
    }
    if let Some(rest) = entry.strip_prefix("* ") {
        return Some(rest.trim_start());
    }
    if let Some(rest) = entry.strip_prefix("• ") {
        return Some(rest.trim_start());
    }
    if let Some(stripped) = strip_bracket_marker(entry) {
        return Some(stripped);
    }
    if let Some(rest) = strip_numeric_marker(entry) {
        return Some(rest);
    }
    None
}

fn strip_numeric_marker(entry: &str) -> Option<&str> {
    let digits = entry.chars().take_while(|ch| ch.is_ascii_digit()).count();
    if digits == 0 {
        return None;
    }

    let rest = &entry[digits..];
    let rest = rest
        .strip_prefix('.')
        .or_else(|| rest.strip_prefix(')'))
        .or_else(|| rest.strip_prefix(':'))?;
    Some(rest.trim_start())
}

fn strip_bracket_marker(entry: &str) -> Option<&str> {
    let rest = entry.strip_prefix('[')?;
    let (marker, rest) = rest.split_once(']')?;
    marker.trim().parse::<usize>().is_ok().then(|| {
        rest.trim_start_matches(|ch: char| {
            ch == '.' || ch == ')' || ch == ':' || ch.is_whitespace()
        })
    })
}

fn looks_like_reference(entry: &str) -> bool {
    let lower = entry.to_ascii_lowercase();
    lower.contains("doi")
        || lower.contains("http")
        || lower.contains("journal")
        || lower.contains("press")
        || lower.contains(" vol")
        || lower.contains(" pp")
        || entry.chars().filter(|ch| *ch == '.').count() >= 2
}

fn looks_like_citation(candidate: &str) -> bool {
    candidate.contains(',') && candidate.chars().any(|ch| ch.is_ascii_digit())
}

fn reference_section_start(text: &str) -> Option<usize> {
    text.lines().position(|line| {
        let normalized = line
            .trim()
            .trim_matches('#')
            .trim()
            .trim_end_matches(['.', ':'])
            .to_ascii_lowercase();
        matches!(
            normalized.as_str(),
            "references"
                | "reference list"
                | "bibliography"
                | "works cited"
                | "footnotes"
                | "endnotes"
        )
    })
}

fn unsupported(code: &str, message: &str) -> IntakeResult {
    IntakeResult {
        references: Vec::new(),
        citations: Vec::new(),
        diagnostics: vec![IntakeDiagnostic {
            code: code.to_string(),
            message: message.to_string(),
        }],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pasted_bibliography_is_segmented_with_original_text() {
        let references = extract_references_from_text(
            "paste",
            "1. Smith J. Trial paper. Journal. doi:10.1/example\n\n2. Doe J. Book title. Press.",
        );

        assert_eq!(references.len(), 2);
        assert_eq!(references[0].id, "ref-0001");
        assert_eq!(
            references[0].text,
            "Smith J. Trial paper. Journal. doi:10.1/example"
        );
        assert_eq!(references[0].span, "line:1");
    }

    #[test]
    fn markdown_detects_reference_section_and_body_citations() {
        let document = IntakeDocument {
            source: "manuscript.md".to_string(),
            kind: IntakeSourceKind::Markdown,
            text: "Intro text (Smith, 2024).\n\n## References\n\n- Smith J. Trial paper. Journal. doi:10.1/example".to_string(),
        };

        let result = extract_intake(&document);

        assert_eq!(result.references.len(), 1);
        assert_eq!(result.citations.len(), 1);
        assert_eq!(result.citations[0].text, "(Smith, 2024)");
        assert_eq!(result.references[0].span, "line:5");
    }

    #[test]
    fn docx_reference_section_preserves_original_line_spans() {
        let document = IntakeDocument {
            source: "submission.docx".to_string(),
            kind: IntakeSourceKind::Docx,
            text: "Intro text (Smith, 2024).\n\nReferences\n\n[1] Smith J. Trial paper. Journal. doi:10.1/example\n[2] Doe J. Book title. Press.".to_string(),
        };

        let result = extract_intake(&document);

        assert_eq!(result.references.len(), 2);
        assert_eq!(result.references[0].span, "line:5");
        assert_eq!(
            result.references[0].text,
            "Smith J. Trial paper. Journal. doi:10.1/example"
        );
        assert_eq!(result.citations[0].text, "(Smith, 2024)");
        assert_eq!(result.diagnostics[0].code, "intake.docx.adapter_text_used");
    }

    #[test]
    fn malformed_bibliography_entries_emit_diagnostics_without_changing_spans() {
        let document = IntakeDocument {
            source: "submission.pdf".to_string(),
            kind: IntakeSourceKind::PdfText,
            text: "Intro paragraph.\n\nReferences\n\n1. Smith J Trial paper without enough structure\n[2] Doe J. Proper article. Journal. doi:10.2/example".to_string(),
        };

        let result = extract_intake(&document);

        assert_eq!(result.references.len(), 1);
        assert_eq!(result.references[0].span, "line:6");
        assert_eq!(
            result.references[0].text,
            "Doe J. Proper article. Journal. doi:10.2/example"
        );
        assert_eq!(
            result.diagnostics[0].code,
            "intake.references.malformed_entry"
        );
        assert!(result.diagnostics[0].message.contains("line 5"));
        assert_eq!(result.diagnostics[1].code, "intake.pdf.text_layer_used");
    }

    #[test]
    fn binary_document_sources_return_capability_diagnostics() {
        let document = IntakeDocument {
            source: "scan.pdf".to_string(),
            kind: IntakeSourceKind::ScannedPdf,
            text: String::new(),
        };

        let result = extract_intake(&document);

        assert!(result.references.is_empty());
        assert_eq!(result.diagnostics[0].code, "intake.ocr.required");
    }

    #[test]
    fn docx_adapter_text_is_segmented_with_docx_provenance() {
        let document = IntakeDocument {
            source: "submission.docx".to_string(),
            kind: IntakeSourceKind::Docx,
            text: "[1] Smith J. Trial paper. Journal. doi:10.1/example".to_string(),
        };

        let result = extract_intake(&document);

        assert_eq!(result.references.len(), 1);
        assert_eq!(
            result.references[0].text,
            "Smith J. Trial paper. Journal. doi:10.1/example"
        );
        assert_eq!(result.references[0].source, "submission.docx");
        assert_eq!(result.diagnostics[0].code, "intake.docx.adapter_text_used");
    }

    #[test]
    fn pdf_text_layer_adapter_text_is_segmented_with_reference_spans() {
        let document = IntakeDocument {
            source: "submission.pdf".to_string(),
            kind: IntakeSourceKind::PdfText,
            text: "Background text (Doe, 2024).\n\nWorks Cited\n\n1. Doe J. Dataset paper. Journal. https://doi.org/10.2/data".to_string(),
        };

        let result = extract_intake(&document);

        assert_eq!(result.references.len(), 1);
        assert_eq!(result.references[0].span, "line:5");
        assert_eq!(result.citations[0].text, "(Doe, 2024)");
        assert_eq!(result.diagnostics[0].code, "intake.pdf.text_layer_used");
    }

    #[test]
    fn endnotes_section_is_treated_as_reference_material() {
        let document = IntakeDocument {
            source: "submission.docx".to_string(),
            kind: IntakeSourceKind::Docx,
            text:
                "Body paragraph.\n\nEndnotes\n\n1. Smith J. Trial paper. Journal. doi:10.1/example"
                    .to_string(),
        };

        let result = extract_intake(&document);

        assert_eq!(result.references.len(), 1);
        assert_eq!(result.references[0].span, "line:5");
        assert_eq!(
            result.references[0].text,
            "Smith J. Trial paper. Journal. doi:10.1/example"
        );
    }

    #[test]
    fn docx_table_references_preserve_row_cell_origin_provenance() {
        // Simulates table-like reference material (text-formatted rows) where
        // each row carries a citation-like structure. The extraction should
        // preserve row-level spans and original text without losing provenance.
        let document = IntakeDocument {
            source: "manuscript.docx".to_string(),
            kind: IntakeSourceKind::Docx,
            text: "Smith J. Trial paper. Journal. doi:10.1/example
Doe J. Book title. Press. https://doi.org/10.2/book
Lee K. Report. Agency. doi:10.3/report"
                .to_string(),
        };

        let result = extract_intake(&document);

        // All three rows should be extracted as references
        assert_eq!(result.references.len(), 3);
        assert_eq!(result.references[0].span, "line:1");
        assert_eq!(
            result.references[0].text,
            "Smith J. Trial paper. Journal. doi:10.1/example"
        );
        assert_eq!(result.references[1].span, "line:2");
        assert_eq!(
            result.references[1].text,
            "Doe J. Book title. Press. https://doi.org/10.2/book"
        );
        assert_eq!(result.references[2].span, "line:3");
        assert_eq!(
            result.references[2].text,
            "Lee K. Report. Agency. doi:10.3/report"
        );
        // DOCX provenance diagnostic should be present
        assert_eq!(result.diagnostics[0].code, "intake.docx.adapter_text_used");
    }

    #[test]
    fn wrapped_numbered_and_bulleted_reference_lines_stay_in_one_entry() {
        let document = IntakeDocument {
            source: "submission.pdf".to_string(),
            kind: IntakeSourceKind::PdfText,
            text: "Intro paragraph.\n\nReferences:\n\n1) Smith J. Long title that wraps\n   across hanging-indent continuation lines\n   and ends with doi:10.1/example\n• Doe J. Another article\n  Journal of Tests. 2024;12(3):45-50."
                .to_string(),
        };

        let result = extract_intake(&document);

        assert_eq!(result.references.len(), 2);
        assert_eq!(result.references[0].span, "line:5");
        assert_eq!(
            result.references[0].text,
            "Smith J. Long title that wraps across hanging-indent continuation lines and ends with doi:10.1/example"
        );
        assert_eq!(result.references[1].span, "line:8");
        assert_eq!(
            result.references[1].text,
            "Doe J. Another article Journal of Tests. 2024;12(3):45-50."
        );
    }
}
