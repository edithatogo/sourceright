//! Canonical CSL JSON data structures, validation, and normalization helpers.
//!
//! Sourceright keeps bibliographic records in CSL JSON and stores verification
//! metadata in the sidecar instead of mutating the canonical record set.

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
    /// A stable item identifier used to join CSL data to verification records.
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
    /// Canonical CSL items in document order.
    pub items: Vec<CslItem>,
}

impl CslDocument {
    /// Returns an empty CSL document.
    pub fn empty() -> Self {
        Self { items: Vec::new() }
    }

    /// Validates ids, titles, DOIs, item types, and sidecar boundary fields.
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
    /// Stable validation code.
    pub code: String,
    /// JSON path to the affected value.
    pub path: String,
    /// Human-readable diagnostic message.
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

/// Validates CSL JSON input and returns structured diagnostics.
pub fn validate_csl_json(input: &str) -> Result<Vec<ValidationDiagnostic>, serde_json::Error> {
    let document = parse_csl_json(input)?;
    Ok(document.validate())
}

/// Parses canonical CSL JSON into a [`CslDocument`].
///
/// ```
/// use sourceright::parse_csl_json;
///
/// let document = parse_csl_json(r#"[{"id":"smith-2024","type":"article-journal","title":"Example"}]"#).unwrap();
/// assert_eq!(document.items.len(), 1);
/// ```
pub fn parse_csl_json(input: &str) -> Result<CslDocument, serde_json::Error> {
    serde_json::from_str(input)
}

/// Formats a [`CslDocument`] as pretty-printed CSL JSON.
///
/// ```
/// use sourceright::{format_csl_json, parse_csl_json};
///
/// let document = parse_csl_json(r#"[{"id":"smith-2024","type":"article-journal","title":"Example"}]"#).unwrap();
/// let json = format_csl_json(&document).unwrap();
/// assert!(json.ends_with('\n'));
/// ```
pub fn format_csl_json(document: &CslDocument) -> Result<String, serde_json::Error> {
    let json = serde_json::to_string_pretty(document)?;
    Ok(format!("{json}\n"))
}

/// Migrates a CSL document into Sourceright's canonical form.
pub fn migrate_csl_document(document: &CslDocument) -> CslMigrationReport {
    let mut normalized = document.clone();
    let mut changes = Vec::new();

    for (index, item) in document.items.iter().enumerate() {
        let normalized_id = normalize_identifier(&item.id);
        if normalized_id != item.id {
            changes.push(CslMigrationChange::new(
                format!("$[{index}].id"),
                "csl.migration.id_normalized",
                "Normalized CSL item id whitespace",
            ));
        }

        let normalized_type = normalize_item_type(&item.item_type);
        if normalized_type != item.item_type {
            changes.push(CslMigrationChange::new(
                format!("$[{index}].type"),
                "csl.migration.type_normalized",
                "Normalized CSL item type spelling",
            ));
        }

        if let Some(title) = item.title.as_deref() {
            let normalized_title = normalize_title(title);
            if normalized_title != title {
                changes.push(CslMigrationChange::new(
                    format!("$[{index}].title"),
                    "csl.migration.title_normalized",
                    "Normalized CSL title whitespace",
                ));
            }
        }

        if let Some(doi) = item.doi.as_deref() {
            let normalized_doi = normalize_doi(doi);
            if !normalized_doi.is_empty() && normalized_doi != doi {
                changes.push(CslMigrationChange::new(
                    format!("$[{index}].DOI"),
                    "csl.migration.doi_normalized",
                    "Normalized DOI for provider matching",
                ));
            }
        }
    }

    normalized.normalize_in_place();
    let diagnostics = normalized.validate();

    CslMigrationReport {
        document: normalized,
        diagnostics,
        changes,
    }
}

/// Migrates CSL JSON input and returns the normalized document plus changes.
pub fn migrate_csl_json(input: &str) -> Result<CslMigrationReport, serde_json::Error> {
    let document = parse_csl_json(input)?;
    Ok(migrate_csl_document(&document))
}

/// Normalizes a CSL identifier by collapsing whitespace.
pub fn normalize_identifier(value: &str) -> String {
    collapse_whitespace(value).to_string()
}

/// Normalizes a CSL item type to canonical lowercase spelling.
pub fn normalize_item_type(value: &str) -> String {
    collapse_whitespace(value).to_ascii_lowercase()
}

/// Normalizes a CSL title by collapsing whitespace.
pub fn normalize_title(value: &str) -> String {
    collapse_whitespace(value).to_string()
}

/// Normalizes a DOI by trimming common prefixes and lowercasing the result.
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
    value.trim_start_matches('/').trim().to_ascii_lowercase()
}

fn collapse_whitespace(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CslMigrationReport {
    /// Normalized CSL document.
    pub document: CslDocument,
    /// Validation diagnostics produced during migration.
    pub diagnostics: Vec<ValidationDiagnostic>,
    /// Changes applied during migration.
    pub changes: Vec<CslMigrationChange>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CslMigrationChange {
    /// JSON path of the migrated value.
    pub path: String,
    /// Stable migration code.
    pub code: String,
    /// Human-readable change description.
    pub message: String,
}

impl CslMigrationChange {
    fn new(path: impl Into<String>, code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            code: code.into(),
            message: message.into(),
        }
    }
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

    #[test]
    fn formats_csl_json_with_stable_newline_terminated_output() {
        let document = CslDocument {
            items: vec![CslItem {
                id: "smith-2024-trial".to_string(),
                item_type: "article-journal".to_string(),
                title: Some("A reference verification trial".to_string()),
                doi: Some("10.1234/example".to_string()),
                extra: BTreeMap::from([
                    (
                        "issued".to_string(),
                        serde_json::json!({"date-parts": [[2024]]}),
                    ),
                    (
                        "container-title".to_string(),
                        Value::String("Example Journal".to_string()),
                    ),
                ]),
            }],
        };

        let json = format_csl_json(&document).expect("format CSL JSON");

        assert!(json.ends_with('\n'));
        assert_eq!(
            json,
            concat!(
                "[\n",
                "  {\n",
                "    \"id\": \"smith-2024-trial\",\n",
                "    \"type\": \"article-journal\",\n",
                "    \"title\": \"A reference verification trial\",\n",
                "    \"DOI\": \"10.1234/example\",\n",
                "    \"container-title\": \"Example Journal\",\n",
                "    \"issued\": {\n",
                "      \"date-parts\": [\n",
                "        [\n",
                "          2024\n",
                "        ]\n",
                "      ]\n",
                "    }\n",
                "  }\n",
                "]\n"
            )
        );
    }

    #[test]
    fn parses_and_reformats_csl_json_deterministically() {
        let input = r#"[{"issued":{"date-parts":[[2024]]},"container-title":"Example Journal","DOI":"10.1234/example","title":"A reference verification trial","type":"article-journal","id":"smith-2024-trial"}]"#;

        let document = parse_csl_json(input).expect("parse CSL JSON");
        let first = format_csl_json(&document).expect("format CSL JSON");
        let reparsed = parse_csl_json(&first).expect("reparse formatted CSL JSON");
        let second = format_csl_json(&reparsed).expect("reformat CSL JSON");

        assert_eq!(first, second);
    }

    #[test]
    fn migration_normalizes_legacy_records_and_reports_changes() {
        let report = migrate_csl_json(
            r#"[{"id":" smith   2024 ","type":"ARTICLE-JOURNAL","title":"  Example   Title ","DOI":"https://doi.org/10.1000/ABC"}]"#,
        )
        .expect("migrate CSL JSON");

        let item = &report.document.items[0];
        assert_eq!(item.id, "smith 2024");
        assert_eq!(item.item_type, "article-journal");
        assert_eq!(item.title.as_deref(), Some("Example Title"));
        assert_eq!(item.doi.as_deref(), Some("10.1000/abc"));
        assert!(report.diagnostics.is_empty());
        assert_eq!(
            report
                .changes
                .iter()
                .map(|change| change.code.as_str())
                .collect::<Vec<_>>(),
            [
                "csl.migration.id_normalized",
                "csl.migration.type_normalized",
                "csl.migration.title_normalized",
                "csl.migration.doi_normalized",
            ]
        );
    }
}
