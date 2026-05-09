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

const SUPPORTED_ITEM_TYPES: &[&str] = &[
    "article",
    "article-journal",
    "article-magazine",
    "article-newspaper",
    "bill",
    "book",
    "broadcast",
    "chapter",
    "dataset",
    "entry",
    "entry-dictionary",
    "entry-encyclopedia",
    "figure",
    "graphic",
    "interview",
    "legal_case",
    "legislation",
    "manuscript",
    "map",
    "motion_picture",
    "musical_score",
    "pamphlet",
    "paper-conference",
    "patent",
    "personal_communication",
    "post",
    "post-weblog",
    "regulation",
    "report",
    "review",
    "review-book",
    "software",
    "song",
    "speech",
    "thesis",
    "treaty",
    "webpage",
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

impl CslItem {
    pub fn normalize_in_place(&mut self) {
        self.id = normalize_identifier(&self.id);
        self.item_type = normalize_item_type(&self.item_type);
        self.title = self
            .title
            .as_deref()
            .map(normalize_title)
            .filter(|title| !title.is_empty());
        self.doi = self
            .doi
            .as_deref()
            .map(normalize_doi)
            .filter(|doi| !doi.is_empty());
    }

    pub fn normalized(&self) -> Self {
        let mut item = self.clone();
        item.normalize_in_place();
        item
    }
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
        let mut seen_ids = BTreeMap::<String, usize>::new();
        for (index, item) in self.items.iter().enumerate() {
            let path = format!("$[{index}]");
            let normalized_id = normalize_identifier(&item.id);
            let normalized_type = normalize_item_type(&item.item_type);

            if normalized_id.is_empty() {
                diagnostics.push(ValidationDiagnostic::new(
                    "csl.id.empty",
                    format!("{path}.id"),
                    "CSL item id must not be empty",
                ));
            } else {
                if normalized_id != item.id {
                    diagnostics.push(ValidationDiagnostic::new(
                        "csl.id.not_canonical",
                        format!("{path}.id"),
                        "CSL item id must be trimmed and whitespace-normalized",
                    ));
                }

                if let Some(first_index) = seen_ids.insert(normalized_id, index) {
                    diagnostics.push(ValidationDiagnostic::new(
                        "csl.id.duplicate",
                        format!("{path}.id"),
                        format!("CSL item id duplicates normalized id from $[{first_index}].id"),
                    ));
                }
            }

            if normalized_type.is_empty() {
                diagnostics.push(ValidationDiagnostic::new(
                    "csl.type.empty",
                    format!("{path}.type"),
                    "CSL item type must not be empty",
                ));
            } else {
                if normalized_type != item.item_type {
                    diagnostics.push(ValidationDiagnostic::new(
                        "csl.type.not_canonical",
                        format!("{path}.type"),
                        "CSL item type must use canonical lowercase CSL spelling",
                    ));
                }

                if !SUPPORTED_ITEM_TYPES.contains(&normalized_type.as_str()) {
                    diagnostics.push(ValidationDiagnostic::new(
                        "csl.type.unsupported",
                        format!("{path}.type"),
                        "CSL item type is not in Sourceright's supported type set",
                    ));
                }
            }

            let normalized_title = item.title.as_deref().map(normalize_title);
            if normalized_title.as_deref().unwrap_or_default().is_empty() {
                diagnostics.push(ValidationDiagnostic::new(
                    "csl.title.empty",
                    format!("{path}.title"),
                    "CSL item title must not be empty for the initial academic reference workflow",
                ));
            } else if normalized_title.as_deref() != item.title.as_deref() {
                diagnostics.push(ValidationDiagnostic::new(
                    "csl.title.not_canonical",
                    format!("{path}.title"),
                    "CSL item title must be trimmed and whitespace-normalized",
                ));
            }

            if let Some(doi) = item.doi.as_deref() {
                let normalized_doi = normalize_doi(doi);
                if !normalized_doi.is_empty() && normalized_doi != doi {
                    diagnostics.push(ValidationDiagnostic::new(
                        "csl.doi.not_canonical",
                        format!("{path}.DOI"),
                        "CSL DOI must be normalized for provider matching",
                    ));
                }
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

    pub fn normalize_in_place(&mut self) {
        for item in &mut self.items {
            item.normalize_in_place();
        }
    }

    pub fn normalized(&self) -> Self {
        let mut document = self.clone();
        document.normalize_in_place();
        document
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

pub fn normalize_identifier(value: &str) -> String {
    collapse_whitespace(value).to_string()
}

pub fn normalize_item_type(value: &str) -> String {
    collapse_whitespace(value).to_ascii_lowercase()
}

pub fn normalize_title(value: &str) -> String {
    collapse_whitespace(value).to_string()
}

pub fn normalize_doi(value: &str) -> String {
    let value = collapse_whitespace(value);
    let lower = value.to_ascii_lowercase();
    let value = if lower.starts_with("https://doi.org/") {
        &value["https://doi.org/".len()..]
    } else if lower.starts_with("http://doi.org/") {
        &value["http://doi.org/".len()..]
    } else if lower.starts_with("doi:") {
        &value["doi:".len()..]
    } else {
        value.as_str()
    };
    value.trim_start_matches('/').to_ascii_lowercase()
}

fn collapse_whitespace(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
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

    #[test]
    fn normalizes_matching_fields_without_losing_csl_payload() {
        let document = CslDocument {
            items: vec![CslItem {
                id: "  Smith\t2024  Trial  ".to_string(),
                item_type: "ARTICLE-JOURNAL".to_string(),
                title: Some(" Trial\nwith   provider evidence ".to_string()),
                doi: Some("https://doi.org/10.5555/EXAMPLE ".to_string()),
                extra: BTreeMap::from([(
                    "container-title".to_string(),
                    Value::String("BMJ".to_string()),
                )]),
            }],
        };

        let normalized = document.normalized();
        let item = &normalized.items[0];

        assert_eq!(item.id, "Smith 2024 Trial");
        assert_eq!(item.item_type, "article-journal");
        assert_eq!(item.title.as_deref(), Some("Trial with provider evidence"));
        assert_eq!(item.doi.as_deref(), Some("10.5555/example"));
        assert_eq!(
            item.extra.get("container-title"),
            Some(&Value::String("BMJ".to_string()))
        );
    }

    #[test]
    fn validation_reports_duplicate_ids_after_normalization() {
        let diagnostics = validate_csl_json(
            r#"[
                {"id":"smith 2024 trial","type":"article-journal","title":"Example"},
                {"id":"smith   2024   trial","type":"article-journal","title":"Example two"}
            ]"#,
        )
        .expect("parse CSL JSON");

        let codes = diagnostics
            .into_iter()
            .map(|diagnostic| diagnostic.code)
            .collect::<Vec<_>>();

        assert_eq!(codes, ["csl.id.not_canonical", "csl.id.duplicate"]);
    }

    #[test]
    fn validation_reports_noncanonical_type_title_and_doi() {
        let diagnostics = validate_csl_json(
            r#"[{"id":"doe-2025","type":"ARTICLE-JOURNAL","title":"  Example   Title ","DOI":"doi:10.1000/ABC"}]"#,
        )
        .expect("parse CSL JSON");

        let codes = diagnostics
            .into_iter()
            .map(|diagnostic| diagnostic.code)
            .collect::<Vec<_>>();

        assert_eq!(
            codes,
            [
                "csl.type.not_canonical",
                "csl.title.not_canonical",
                "csl.doi.not_canonical"
            ]
        );
    }

    #[test]
    fn validation_reports_unsupported_types() {
        let diagnostics =
            validate_csl_json(r#"[{"id":"x","type":"custom-type","title":"Example"}]"#)
                .expect("parse CSL JSON");

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code, "csl.type.unsupported");
    }
}
