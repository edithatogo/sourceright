use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::csl::{CslDocument, CslItem, normalize_doi, normalize_title};
use crate::sidecar::{ProviderCandidate, ReviewStatus, VerificationSidecar};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConflictResolutionReport {
    pub document: CslDocument,
    pub sidecar: VerificationSidecar,
    pub decisions: Vec<ConflictResolutionDecision>,
}

impl ConflictResolutionReport {
    pub fn to_markdown(&self) -> String {
        let mut markdown = String::from("# Sourceright Conflict Resolution Report\n\n");
        markdown.push_str(&format!("Decisions: {}\n\n", self.decisions.len()));
        if self.decisions.is_empty() {
            markdown.push_str("No provider conflicts or merge decisions were detected.\n");
            return markdown;
        }

        for decision in &self.decisions {
            markdown.push_str(&format!(
                "- `{}` `{}` `{}` from `{}` confidence `{:.2}`: {}\n",
                decision.reference_id,
                decision.field,
                decision.action.as_str(),
                decision.provider,
                decision.confidence,
                decision.reason
            ));
        }

        markdown
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConflictResolutionDecision {
    pub reference_id: String,
    pub field: String,
    pub action: ConflictResolutionAction,
    pub provider: String,
    pub confidence: f64,
    pub before: Option<String>,
    pub after: Option<String>,
    pub reason: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictResolutionAction {
    AppliedMissingCanonicalValue,
    PreservedCanonicalConflict,
    QueuedAmbiguousMerge,
}

impl ConflictResolutionAction {
    fn as_str(self) -> &'static str {
        match self {
            Self::AppliedMissingCanonicalValue => "applied_missing_canonical_value",
            Self::PreservedCanonicalConflict => "preserved_canonical_conflict",
            Self::QueuedAmbiguousMerge => "queued_ambiguous_merge",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConflictResolutionPolicy {
    pub auto_merge_missing_threshold: f64,
    pub review_threshold: f64,
}

impl Default for ConflictResolutionPolicy {
    fn default() -> Self {
        Self {
            auto_merge_missing_threshold: 0.95,
            review_threshold: 0.5,
        }
    }
}

pub fn resolve_conflicts(
    document: &CslDocument,
    sidecar: &VerificationSidecar,
) -> ConflictResolutionReport {
    resolve_conflicts_with_policy(document, sidecar, ConflictResolutionPolicy::default())
}

pub fn resolve_conflicts_with_policy(
    document: &CslDocument,
    sidecar: &VerificationSidecar,
    policy: ConflictResolutionPolicy,
) -> ConflictResolutionReport {
    let mut document = document.clone();
    let mut sidecar = sidecar.clone();
    let mut decisions = Vec::new();

    for item in &mut document.items {
        let reference_id = item.id.clone();
        let Some(verification) = sidecar.references.get_mut(&reference_id) else {
            continue;
        };

        for candidate in verification.provider_candidates.clone() {
            for field in ["DOI", "title", "container-title", "issued"] {
                let Some(candidate_value) = candidate_field_value(&candidate, field) else {
                    continue;
                };
                let before = item_field_value(item, field);

                match before.as_deref() {
                    None | Some("")
                        if candidate.confidence >= policy.auto_merge_missing_threshold =>
                    {
                        set_item_field_value(item, field, &candidate_value);
                        decisions.push(ConflictResolutionDecision {
                            reference_id: reference_id.clone(),
                            field: field.to_string(),
                            action: ConflictResolutionAction::AppliedMissingCanonicalValue,
                            provider: candidate.provider.clone(),
                            confidence: candidate.confidence,
                            before,
                            after: Some(candidate_value),
                            reason:
                                "High-confidence provider value filled a missing canonical field."
                                    .to_string(),
                        });
                    }
                    None | Some("") if candidate.confidence >= policy.review_threshold => {
                        queue_conflict(
                            verification,
                            &candidate,
                            field,
                            None,
                            &candidate_value,
                            "missing canonical value below auto-merge confidence",
                        );
                        decisions.push(ConflictResolutionDecision {
                            reference_id: reference_id.clone(),
                            field: field.to_string(),
                            action: ConflictResolutionAction::QueuedAmbiguousMerge,
                            provider: candidate.provider.clone(),
                            confidence: candidate.confidence,
                            before,
                            after: Some(candidate_value),
                            reason: "Provider value is plausible but below the automatic merge threshold.".to_string(),
                        });
                    }
                    Some(current)
                        if comparable_value(field, current)
                            != comparable_value(field, &candidate_value)
                            && candidate.confidence >= policy.review_threshold =>
                    {
                        queue_conflict(
                            verification,
                            &candidate,
                            field,
                            Some(current),
                            &candidate_value,
                            "provider disagrees with canonical value",
                        );
                        decisions.push(ConflictResolutionDecision {
                            reference_id: reference_id.clone(),
                            field: field.to_string(),
                            action: ConflictResolutionAction::PreservedCanonicalConflict,
                            provider: candidate.provider.clone(),
                            confidence: candidate.confidence,
                            before,
                            after: Some(candidate_value),
                            reason: "Canonical value was preserved and disagreement was recorded for review.".to_string(),
                        });
                    }
                    _ => {}
                }
            }
        }
    }

    ConflictResolutionReport {
        document,
        sidecar,
        decisions,
    }
}

fn queue_conflict(
    verification: &mut crate::sidecar::ReferenceVerification,
    candidate: &ProviderCandidate,
    field: &str,
    canonical: Option<&str>,
    candidate_value: &str,
    reason: &str,
) {
    let conflict = serde_json::json!({
        "field": field,
        "severity": "review",
        "provider": candidate.provider,
        "confidence": candidate.confidence,
        "canonical": canonical,
        "candidate": candidate_value,
        "reason": reason,
    });

    if !verification.conflicts.contains(&conflict) {
        verification.conflicts.push(conflict);
    }
    verification.review_status = ReviewStatus::Queued;
}

fn item_field_value(item: &CslItem, field: &str) -> Option<String> {
    match field {
        "DOI" => item.doi.clone(),
        "title" => item.title.clone(),
        key => item.extra.get(key).and_then(stringish_value),
    }
}

fn set_item_field_value(item: &mut CslItem, field: &str, value: &str) {
    match field {
        "DOI" => item.doi = Some(normalize_doi(value)),
        "title" => item.title = Some(normalize_title(value)),
        key => {
            item.extra
                .insert(key.to_string(), Value::String(normalize_title(value)));
        }
    }
}

fn candidate_field_value(candidate: &ProviderCandidate, field: &str) -> Option<String> {
    match field {
        "DOI" => candidate_doi(&candidate.data).map(|doi| normalize_doi(&doi)),
        "title" => candidate_title(&candidate.data).map(|title| normalize_title(&title)),
        "container-title" => {
            candidate_container_title(&candidate.data).map(|title| normalize_title(&title))
        }
        "issued" => candidate_issued(&candidate.data),
        _ => None,
    }
    .filter(|value| !value.trim().is_empty())
}

fn candidate_doi(data: &Value) -> Option<String> {
    string_at(data, &["DOI"])
        .or_else(|| string_at(data, &["doi"]))
        .or_else(|| string_at(data, &["data", "attributes", "doi"]))
        .or_else(|| string_at(data, &["attributes", "doi"]))
        .or_else(|| {
            data.get("articleids")?.as_array()?.iter().find_map(|id| {
                (string_at(id, &["idtype"])?.eq_ignore_ascii_case("doi"))
                    .then(|| string_at(id, &["value"]))
                    .flatten()
            })
        })
}

fn candidate_title(data: &Value) -> Option<String> {
    string_at(data, &["title"])
        .or_else(|| string_at(data, &["display_name"]))
        .or_else(|| string_at(data, &["sorttitle"]))
        .or_else(|| {
            data.get("title")?
                .as_array()?
                .first()
                .and_then(stringish_value)
        })
        .or_else(|| string_at(data, &["data", "attributes", "titles", "0", "title"]))
        .or_else(|| string_at(data, &["attributes", "titles", "0", "title"]))
}

fn candidate_container_title(data: &Value) -> Option<String> {
    string_at(data, &["container-title"])
        .or_else(|| {
            data.get("container-title")?
                .as_array()?
                .first()
                .and_then(stringish_value)
        })
        .or_else(|| string_at(data, &["journal"]))
        .or_else(|| string_at(data, &["fulljournalname"]))
        .or_else(|| string_at(data, &["primary_location", "source", "display_name"]))
}

fn candidate_issued(data: &Value) -> Option<String> {
    string_at(data, &["published-print", "date-time"])
        .or_else(|| string_at(data, &["published-online", "date-time"]))
        .or_else(|| string_at(data, &["publication_date"]))
        .or_else(|| string_at(data, &["publicationDate"]))
        .or_else(|| string_at(data, &["from-pub-date"]))
}

fn string_at(data: &Value, path: &[&str]) -> Option<String> {
    let mut current = data;
    for segment in path {
        current = if let Ok(index) = segment.parse::<usize>() {
            current.as_array()?.get(index)?
        } else {
            current.get(*segment)?
        };
    }
    stringish_value(current)
}

fn stringish_value(value: &Value) -> Option<String> {
    value.as_str().map(ToString::to_string)
}

fn comparable_value(field: &str, value: &str) -> String {
    match field {
        "DOI" => normalize_doi(value),
        _ => normalize_title(value).to_ascii_lowercase(),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde_json::json;

    use super::*;
    use crate::sidecar::{ReferenceVerification, VerificationSidecar};

    #[test]
    fn high_confidence_provider_value_fills_missing_doi() {
        let document = CslDocument {
            items: vec![CslItem {
                id: "smith-2024".to_string(),
                item_type: "article-journal".to_string(),
                title: Some("Trial".to_string()),
                doi: None,
                extra: BTreeMap::new(),
            }],
        };
        let mut sidecar = VerificationSidecar::empty();
        sidecar.references.insert(
            "smith-2024".to_string(),
            ReferenceVerification {
                provider_candidates: vec![ProviderCandidate {
                    provider: "crossref".to_string(),
                    confidence: 0.98,
                    retrieved_at: "2026-05-10T00:00:00Z".to_string(),
                    data: json!({"DOI": "https://doi.org/10.1000/ABC"}),
                }],
                ..ReferenceVerification::default()
            },
        );

        let report = resolve_conflicts(&document, &sidecar);

        assert_eq!(report.document.items[0].doi.as_deref(), Some("10.1000/abc"));
        assert_eq!(
            report.decisions[0].action,
            ConflictResolutionAction::AppliedMissingCanonicalValue
        );
        assert!(report.sidecar.references["smith-2024"].conflicts.is_empty());
    }

    #[test]
    fn disagreement_preserves_canonical_value_and_queues_review() {
        let document = CslDocument {
            items: vec![CslItem {
                id: "smith-2024".to_string(),
                item_type: "article-journal".to_string(),
                title: Some("Canonical title".to_string()),
                doi: Some("10.1000/canonical".to_string()),
                extra: BTreeMap::new(),
            }],
        };
        let mut sidecar = VerificationSidecar::empty();
        sidecar.references.insert(
            "smith-2024".to_string(),
            ReferenceVerification {
                provider_candidates: vec![ProviderCandidate {
                    provider: "openalex".to_string(),
                    confidence: 0.91,
                    retrieved_at: "2026-05-10T00:00:00Z".to_string(),
                    data: json!({"doi": "10.1000/provider", "display_name": "Provider title"}),
                }],
                ..ReferenceVerification::default()
            },
        );

        let report = resolve_conflicts(&document, &sidecar);
        let verification = &report.sidecar.references["smith-2024"];

        assert_eq!(
            report.document.items[0].doi.as_deref(),
            Some("10.1000/canonical")
        );
        assert_eq!(verification.review_status, ReviewStatus::Queued);
        assert_eq!(verification.conflicts.len(), 2);
        assert!(
            report
                .to_markdown()
                .contains("preserved_canonical_conflict")
        );
    }

    #[test]
    fn plausible_but_low_confidence_missing_value_is_queued_not_merged() {
        let document = CslDocument {
            items: vec![CslItem {
                id: "smith-2024".to_string(),
                item_type: "article-journal".to_string(),
                title: Some("Trial".to_string()),
                doi: None,
                extra: BTreeMap::new(),
            }],
        };
        let mut sidecar = VerificationSidecar::empty();
        sidecar.references.insert(
            "smith-2024".to_string(),
            ReferenceVerification {
                provider_candidates: vec![ProviderCandidate {
                    provider: "datacite".to_string(),
                    confidence: 0.7,
                    retrieved_at: "2026-05-10T00:00:00Z".to_string(),
                    data: json!({"data": {"attributes": {"doi": "10.5061/example"}}}),
                }],
                ..ReferenceVerification::default()
            },
        );

        let report = resolve_conflicts(&document, &sidecar);

        assert!(report.document.items[0].doi.is_none());
        assert_eq!(
            report.decisions[0].action,
            ConflictResolutionAction::QueuedAmbiguousMerge
        );
        assert_eq!(
            report.sidecar.references["smith-2024"].review_status,
            ReviewStatus::Queued
        );
    }
}
