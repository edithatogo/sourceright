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
            let reference_start = markdown_reference_start(&document.text);
            extract_text_like(&document.source, &document.text, reference_start)
        }
        IntakeSourceKind::Docx => unsupported(
            "intake.docx.requires_extractor",
            "DOCX extraction requires an adapter; plain text extracted from DOCX can be passed as text.",
        ),
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
    reference_entries(text)
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
    let reference_text = reference_start
        .map(|start| text.lines().skip(start).collect::<Vec<_>>().join("\n"))
        .unwrap_or_else(|| text.to_string());

    IntakeResult {
        references: extract_references_from_text(source, &reference_text),
        citations: extract_in_text_citations(source, text),
        diagnostics: Vec::new(),
    }
}

fn reference_entries(text: &str) -> Vec<(usize, String)> {
    let mut entries = Vec::new();
    let mut current = String::new();
    let mut start_line = 1;

    for (line_index, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            push_entry(&mut entries, start_line, &mut current);
            start_line = line_index + 2;
            continue;
        }

        if starts_new_reference(trimmed) && !current.trim().is_empty() {
            push_entry(&mut entries, start_line, &mut current);
            start_line = line_index + 1;
        }

        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(trimmed);
    }

    push_entry(&mut entries, start_line, &mut current);
    entries
}

fn push_entry(entries: &mut Vec<(usize, String)>, start_line: usize, current: &mut String) {
    let entry = current.trim();
    if !entry.is_empty() && looks_like_reference(entry) {
        entries.push((start_line, strip_reference_marker(entry)));
    }
    current.clear();
}

fn starts_new_reference(line: &str) -> bool {
    let marker = line
        .split_once(' ')
        .map(|(marker, _)| marker)
        .unwrap_or(line)
        .trim_end_matches('.');
    marker.parse::<usize>().is_ok()
        || line.starts_with("- ")
        || line.starts_with("* ")
        || line.starts_with('[')
}

fn strip_reference_marker(entry: &str) -> String {
    let entry = entry.trim_start_matches("- ").trim_start_matches("* ");
    if let Some((number, rest)) = entry.split_once('.')
        && number.trim().parse::<usize>().is_ok()
    {
        return rest.trim().to_string();
    }
    entry.to_string()
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

fn markdown_reference_start(text: &str) -> Option<usize> {
    text.lines().position(|line| {
        let normalized = line.trim().trim_matches('#').trim().to_ascii_lowercase();
        matches!(
            normalized.as_str(),
            "references" | "reference list" | "bibliography"
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
}
