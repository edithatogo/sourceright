use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

const SIDECAR_KEYS: &[&str] = &[
    "verification",
    "verification_status",
    "provider_matches",
    "provider_candidates",
    "confidence",
    "conflicts",
    "review_status",
    "review_decisions",
    "extraction",
    "provenance",
];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CslItem {
    pub id: String,
    #[serde(rename = "type")]
    pub item_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "DOI")]
    pub doi: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CslDocument {
    pub items: Vec<CslItem>,
}

impl CslDocument {
    pub fn empty() -> Self {
        Self { items: Vec::new() }
    }

    pub fn validate(&self) -> Vec<ValidationDiagnostic> {
        let mut diagnostics = Vec::new();

        for (index, item) in self.items.iter().enumerate() {
            let path = format!("$[{index}]");

            if item.id.trim().is_empty() {
                diagnostics.push(ValidationDiagnostic::new(
                    "csl.id.empty",
                    format!("{path}.id"),
                    "CSL item id must not be empty",
                ));
            }

            if item.item_type.trim().is_empty() {
                diagnostics.push(ValidationDiagnostic::new(
                    "csl.type.empty",
                    format!("{path}.type"),
                    "CSL item type must not be empty",
                ));
            }

            if item.title.as_deref().unwrap_or_default().trim().is_empty() {
                diagnostics.push(ValidationDiagnostic::new(
                    "csl.title.empty",
                    format!("{path}.title"),
                    "CSL item title must not be empty for the initial academic reference workflow",
                ));
            }

            for key in SIDECAR_KEYS {
                if item.extra.contains_key(*key) {
                    diagnostics.push(ValidationDiagnostic::new(
                        "csl.sidecar_field",
                        format!("{path}.{key}"),
                        "verification metadata belongs in references.verification.json, not CSL JSON",
                    ));
                }
            }
        }

        diagnostics
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationDiagnostic {
    pub code: String,
    pub path: String,
    pub message: String,
}

impl ValidationDiagnostic {
    fn new(code: impl Into<String>, path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            path: path.into(),
            message: message.into(),
        }
    }
}

pub fn validate_csl_json(input: &str) -> Result<Vec<ValidationDiagnostic>, serde_json::Error> {
    let document: CslDocument = serde_json::from_str(input)?;
    Ok(document.validate())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_article_record_serializes_as_csl_array() {
        let document = CslDocument {
            items: vec![CslItem {
                id: "smith-2024-trial".to_string(),
                item_type: "article-journal".to_string(),
                title: Some("A reference verification trial".to_string()),
                doi: Some("10.1234/example".to_string()),
                extra: BTreeMap::new(),
            }],
        };

        let json = serde_json::to_string_pretty(&document).expect("serialize CSL document");

        assert!(json.starts_with('['));
        assert!(json.contains(r#""type": "article-journal""#));
        assert!(json.contains(r#""DOI": "10.1234/example""#));
        assert!(document.validate().is_empty());
    }

    #[test]
    fn validation_rejects_empty_required_fields() {
        let document = CslDocument {
            items: vec![CslItem {
                id: " ".to_string(),
                item_type: "".to_string(),
                title: None,
                doi: None,
                extra: BTreeMap::new(),
            }],
        };

        let codes = document
            .validate()
            .into_iter()
            .map(|diagnostic| diagnostic.code)
            .collect::<Vec<_>>();

        assert_eq!(codes, ["csl.id.empty", "csl.type.empty", "csl.title.empty"]);
    }

    #[test]
    fn validation_rejects_sidecar_metadata_inside_csl() {
        let diagnostics = validate_csl_json(
            r#"[{"id":"doe-2025","type":"article-journal","title":"Example","confidence":0.9}]"#,
        )
        .expect("parse CSL JSON");

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code, "csl.sidecar_field");
        assert_eq!(diagnostics[0].path, "$[0].confidence");
    }
}
