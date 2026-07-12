//! Backend-neutral page, block, and token layout IR.
//!
//! This module intentionally contains no PDF parser and no CSL mapping. Parser
//! adapters must produce this bounded contract before scholarly semantics are
//! attempted.

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const LAYOUT_SCHEMA_VERSION: &str = "citeweft.layout-document.v1";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayoutDocument {
    pub schema_version: String,
    pub provenance: LayoutProvenance,
    pub pages: Vec<LayoutPage>,
    pub reading_order: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<LayoutDiagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayoutProvenance {
    pub backend: String,
    pub version: String,
    pub configuration: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayoutPage {
    pub number: u32,
    pub width: f32,
    pub height: f32,
    pub blocks: Vec<LayoutBlock>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayoutBlock {
    pub id: String,
    #[serde(rename = "box")]
    pub box_: LayoutBox,
    pub column: Option<u16>,
    pub confidence: Option<f32>,
    pub tokens: Vec<LayoutToken>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayoutToken {
    pub id: String,
    pub text: String,
    #[serde(rename = "box")]
    pub box_: LayoutBox,
    pub source_id: String,
    pub style: LayoutStyle,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayoutStyle {
    pub font_family: Option<String>,
    pub font_size: Option<f32>,
    pub bold: bool,
    pub italic: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LayoutBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayoutDiagnostic {
    pub code: LayoutDiagnosticCode,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LayoutDiagnosticCode {
    OcrRequired,
    ReadingOrderAmbiguous,
    UnsupportedInput,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutLimits {
    pub max_input_bytes: usize,
    pub max_pages: usize,
    pub max_tokens: usize,
}

impl Default for LayoutLimits {
    fn default() -> Self {
        Self {
            max_input_bytes: 25 * 1024 * 1024,
            max_pages: 1_000,
            max_tokens: 1_000_000,
        }
    }
}

#[derive(Debug, Error)]
pub enum LayoutError {
    #[error("layout input exceeds {limit} bytes")]
    InputTooLarge { limit: usize },
    #[error("layout input exceeds {limit} pages")]
    PageLimitExceeded { limit: usize },
    #[error("layout input exceeds {limit} tokens")]
    TokenLimitExceeded { limit: usize },
    #[error("layout input is not valid UTF-8")]
    InvalidUtf8,
    #[error("fixture layout adapter does not accept PDF input")]
    UnsupportedInput,
    #[error("layout page {page} has invalid geometry")]
    InvalidGeometry { page: u32 },
}

pub trait LayoutExtractor {
    fn extract(&self, input: &[u8], limits: LayoutLimits) -> Result<LayoutDocument, LayoutError>;
}

/// Deterministic contract adapter for self-authored text fixtures.
///
/// It is deliberately not a PDF parser. Its purpose is to exercise the IR,
/// limits, provenance, and reading-order contract before a parser dependency is
/// selected through benchmark and license evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureTextLayoutExtractor {
    pub backend_version: String,
}

impl Default for FixtureTextLayoutExtractor {
    fn default() -> Self {
        Self {
            backend_version: "fixture-text-v1".to_string(),
        }
    }
}

impl LayoutExtractor for FixtureTextLayoutExtractor {
    fn extract(&self, input: &[u8], limits: LayoutLimits) -> Result<LayoutDocument, LayoutError> {
        if input.len() > limits.max_input_bytes {
            return Err(LayoutError::InputTooLarge {
                limit: limits.max_input_bytes,
            });
        }
        if input.starts_with(b"%PDF-") {
            return Err(LayoutError::UnsupportedInput);
        }
        let text = std::str::from_utf8(input).map_err(|_| LayoutError::InvalidUtf8)?;
        let page_texts: Vec<&str> = text.split('\u{000c}').collect();
        if page_texts.len() > limits.max_pages {
            return Err(LayoutError::PageLimitExceeded {
                limit: limits.max_pages,
            });
        }

        let mut pages = Vec::with_capacity(page_texts.len());
        let mut diagnostics = Vec::new();
        let mut token_count = 0;
        for (page_index, page_text) in page_texts.iter().enumerate() {
            let mut blocks = Vec::new();
            for (line_index, line) in page_text.lines().enumerate() {
                let words: Vec<&str> = line.split_whitespace().collect();
                if words.is_empty() {
                    continue;
                }
                token_count += words.len();
                if token_count > limits.max_tokens {
                    return Err(LayoutError::TokenLimitExceeded {
                        limit: limits.max_tokens,
                    });
                }
                let tokens = words
                    .iter()
                    .enumerate()
                    .map(|(word_index, word)| LayoutToken {
                        id: format!("p{page_index}-b{line_index}-t{word_index}"),
                        text: (*word).to_string(),
                        box_: LayoutBox {
                            x: (word_index as f32) * 12.0,
                            y: (line_index as f32) * 14.0,
                            width: (*word).len() as f32 * 7.0,
                            height: 10.0,
                        },
                        source_id: format!("page:{page_index}:line:{line_index}:word:{word_index}"),
                        style: LayoutStyle {
                            font_family: None,
                            font_size: Some(10.0),
                            bold: false,
                            italic: false,
                        },
                    })
                    .collect();
                blocks.push(LayoutBlock {
                    id: format!("p{page_index}-b{line_index}"),
                    box_: LayoutBox {
                        x: 0.0,
                        y: (line_index as f32) * 14.0,
                        width: 500.0,
                        height: 10.0,
                    },
                    column: Some(0),
                    confidence: Some(1.0),
                    tokens,
                });
            }
            pages.push(LayoutPage {
                number: page_index as u32 + 1,
                width: 612.0,
                height: 792.0,
                blocks,
            });
        }
        if token_count == 0 {
            diagnostics.push(LayoutDiagnostic {
                code: LayoutDiagnosticCode::OcrRequired,
                message: "no text layer was recovered; OCR handoff is required".to_string(),
            });
        }
        let reading_order = deterministic_reading_order(&mut pages, &mut diagnostics);
        let document = LayoutDocument {
            schema_version: LAYOUT_SCHEMA_VERSION.to_string(),
            provenance: LayoutProvenance {
                backend: "fixture-text".to_string(),
                version: self.backend_version.clone(),
                configuration: "bounded-single-column".to_string(),
            },
            pages,
            reading_order,
            diagnostics,
        };
        validate_geometry(&document)?;
        Ok(document)
    }
}

fn deterministic_reading_order(
    pages: &mut [LayoutPage],
    diagnostics: &mut Vec<LayoutDiagnostic>,
) -> Vec<String> {
    let mut order = Vec::new();
    for page in pages {
        page.blocks.sort_by(|left, right| {
            left.column
                .unwrap_or(u16::MAX)
                .cmp(&right.column.unwrap_or(u16::MAX))
                .then_with(|| left.box_.y.total_cmp(&right.box_.y))
                .then_with(|| left.box_.x.total_cmp(&right.box_.x))
                .then_with(|| left.id.cmp(&right.id))
        });
        for block in &page.blocks {
            order.push(block.id.clone());
        }
        for left in page.blocks.iter() {
            for right in page
                .blocks
                .iter()
                .filter(|candidate| candidate.id > left.id)
            {
                if left.box_.y < right.box_.y + right.box_.height
                    && right.box_.y < left.box_.y + left.box_.height
                    && left.box_.x < right.box_.x + right.box_.width
                    && right.box_.x < left.box_.x + left.box_.width
                {
                    diagnostics.push(LayoutDiagnostic {
                        code: LayoutDiagnosticCode::ReadingOrderAmbiguous,
                        message: format!("overlapping blocks `{}` and `{}`", left.id, right.id),
                    });
                }
            }
        }
    }
    order
}

fn validate_geometry(document: &LayoutDocument) -> Result<(), LayoutError> {
    for page in &document.pages {
        if !page.width.is_finite()
            || !page.height.is_finite()
            || page.width <= 0.0
            || page.height <= 0.0
        {
            return Err(LayoutError::InvalidGeometry { page: page.number });
        }
        for block in &page.blocks {
            if !within_page(block.box_, page.width, page.height) {
                return Err(LayoutError::InvalidGeometry { page: page.number });
            }
            for token in &block.tokens {
                if !within_page(token.box_, page.width, page.height) {
                    return Err(LayoutError::InvalidGeometry { page: page.number });
                }
            }
        }
    }
    Ok(())
}

fn within_page(box_: LayoutBox, width: f32, height: f32) -> bool {
    box_.x.is_finite()
        && box_.y.is_finite()
        && box_.width.is_finite()
        && box_.height.is_finite()
        && box_.x >= 0.0
        && box_.y >= 0.0
        && box_.width >= 0.0
        && box_.height >= 0.0
        && box_.x + box_.width <= width
        && box_.y + box_.height <= height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_adapter_has_stable_order_and_traceable_boxes() {
        let document = FixtureTextLayoutExtractor::default()
            .extract(b"alpha beta\ngamma", LayoutLimits::default())
            .expect("extract fixture");
        assert_eq!(document.schema_version, LAYOUT_SCHEMA_VERSION);
        assert_eq!(document.reading_order, ["p0-b0", "p0-b1"]);
        assert_eq!(
            document.pages[0].blocks[0].tokens[0].source_id,
            "page:0:line:0:word:0"
        );
        assert!(document.diagnostics.is_empty());
    }

    #[test]
    fn empty_text_requires_ocr_without_synthetic_tokens() {
        let document = FixtureTextLayoutExtractor::default()
            .extract(b"", LayoutLimits::default())
            .expect("empty fixture");
        assert!(document.pages.iter().all(|page| page.blocks.is_empty()));
        assert_eq!(
            document.diagnostics[0].code,
            LayoutDiagnosticCode::OcrRequired
        );
    }

    #[test]
    fn limits_reject_oversized_input_before_parsing() {
        let error = FixtureTextLayoutExtractor::default()
            .extract(
                b"12345",
                LayoutLimits {
                    max_input_bytes: 4,
                    ..Default::default()
                },
            )
            .expect_err("input limit");
        assert!(matches!(error, LayoutError::InputTooLarge { limit: 4 }));
    }

    #[test]
    fn pdf_input_is_rejected_by_fixture_adapter() {
        let error = FixtureTextLayoutExtractor::default()
            .extract(b"%PDF-1.7", LayoutLimits::default())
            .expect_err("fixture adapter must reject PDF input");
        assert!(matches!(error, LayoutError::UnsupportedInput));
    }

    #[test]
    fn overlapping_blocks_report_reading_order_ambiguity() {
        let mut pages = vec![LayoutPage {
            number: 1,
            width: 100.0,
            height: 100.0,
            blocks: vec![
                LayoutBlock {
                    id: "left".to_string(),
                    box_: LayoutBox {
                        x: 0.0,
                        y: 0.0,
                        width: 60.0,
                        height: 20.0,
                    },
                    column: Some(0),
                    confidence: None,
                    tokens: Vec::new(),
                },
                LayoutBlock {
                    id: "right".to_string(),
                    box_: LayoutBox {
                        x: 40.0,
                        y: 0.0,
                        width: 60.0,
                        height: 20.0,
                    },
                    column: Some(1),
                    confidence: None,
                    tokens: Vec::new(),
                },
            ],
        }];
        let mut diagnostics = Vec::new();
        deterministic_reading_order(&mut pages, &mut diagnostics);
        assert!(
            diagnostics.iter().any(|diagnostic| {
                diagnostic.code == LayoutDiagnosticCode::ReadingOrderAmbiguous
            })
        );
    }
}
