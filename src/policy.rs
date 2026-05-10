use serde::{Deserialize, Serialize};

use crate::csl::{CslDocument, CslItem};
use crate::sidecar::{ProviderCandidate, VerificationSidecar};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourcerightPolicy {
    pub schema_version: String,
    pub policy_id: String,
    pub reference_order: ReferenceOrderPolicy,
    pub doi_policy: DoiPolicy,
    pub recency: RecencyPolicy,
}

impl SourcerightPolicy {
    pub fn journal_vancouver() -> Self {
        Self {
            schema_version: "sourceright.policy.v1".to_string(),
            policy_id: "journal-vancouver".to_string(),
            reference_order: ReferenceOrderPolicy::Appearance,
            doi_policy: DoiPolicy::RequiredIfAvailable,
            recency: RecencyPolicy {
                publication_age_warning_years: Some(10),
                guideline_age_warning_years: Some(5),
                current_year: 2026,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferenceOrderPolicy {
    Appearance,
    Alphabetical,
    Unspecified,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DoiPolicy {
    RequiredIfAvailable,
    Optional,
    NotRequired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecencyPolicy {
    pub publication_age_warning_years: Option<u16>,
    pub guideline_age_warning_years: Option<u16>,
    pub current_year: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyReport {
    pub schema_version: String,
    pub policy_id: String,
    pub issues: Vec<PolicyIssue>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyIssue {
    pub severity: PolicyIssueSeverity,
    pub reference_id: Option<String>,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyIssueSeverity {
    Info,
    Warning,
    Error,
}

pub fn evaluate_policy(document: &CslDocument, policy: &SourcerightPolicy) -> PolicyReport {
    let issues = base_policy_issues(document, policy);
    PolicyReport {
        schema_version: "sourceright.policy_report.v1".to_string(),
        policy_id: policy.policy_id.clone(),
        issues,
    }
}

pub fn evaluate_policy_with_verification(
    document: &CslDocument,
    sidecar: &VerificationSidecar,
    policy: &SourcerightPolicy,
) -> PolicyReport {
    let mut issues = base_policy_issues(document, policy);
    issues.extend(provider_backed_recency_issues(document, sidecar, policy));

    PolicyReport {
        schema_version: "sourceright.policy_report.v1".to_string(),
        policy_id: policy.policy_id.clone(),
        issues,
    }
}

pub fn provider_backed_recency_issues(
    document: &CslDocument,
    sidecar: &VerificationSidecar,
    policy: &SourcerightPolicy,
) -> Vec<PolicyIssue> {
    let mut issues = Vec::new();
    let mut seen = std::collections::BTreeSet::<(String, String)>::new();

    for item in &document.items {
        let Some(verification) = sidecar.references.get(&item.id) else {
            continue;
        };
        for candidate in &verification.provider_candidates {
            for finding in candidate_recency_findings(&item.id, candidate, policy) {
                let key = (
                    finding.reference_id.clone().unwrap_or_default(),
                    finding.code.clone(),
                );
                if seen.insert(key) {
                    issues.push(finding);
                }
            }
        }
    }

    issues
}

fn base_policy_issues(document: &CslDocument, policy: &SourcerightPolicy) -> Vec<PolicyIssue> {
    let mut issues = Vec::new();

    if policy.schema_version != "sourceright.policy.v1" {
        issues.push(PolicyIssue {
            severity: PolicyIssueSeverity::Error,
            reference_id: None,
            code: "policy.schema_version.unsupported".to_string(),
            message: "Policy schema_version is not supported by this Sourceright build."
                .to_string(),
        });
    }

    if policy.doi_policy == DoiPolicy::RequiredIfAvailable {
        for item in &document.items {
            if doi_like_item(item) && item.doi.as_deref().unwrap_or_default().trim().is_empty() {
                issues.push(PolicyIssue {
                    severity: PolicyIssueSeverity::Warning,
                    reference_id: Some(item.id.clone()),
                    code: "policy.doi.missing".to_string(),
                    message: "Reference type usually supports DOI metadata but no DOI is present."
                        .to_string(),
                });
            }
        }
    }

    if let Some(max_age) = policy.recency.publication_age_warning_years {
        for item in &document.items {
            if let Some(year) = issued_year(item)
                && policy.recency.current_year.saturating_sub(year) > max_age
            {
                issues.push(PolicyIssue {
                    severity: PolicyIssueSeverity::Warning,
                    reference_id: Some(item.id.clone()),
                    code: "policy.recency.publication_age".to_string(),
                    message: format!(
                        "Reference is older than the configured publication-age warning threshold of {max_age} years."
                    ),
                });
            }
        }
    }

    if policy.reference_order == ReferenceOrderPolicy::Alphabetical
        && !is_alphabetical_by_title(&document.items)
    {
        issues.push(PolicyIssue {
            severity: PolicyIssueSeverity::Info,
            reference_id: None,
            code: "policy.order.not_alphabetical".to_string(),
            message: "Reference list is not alphabetized by title under the configured policy."
                .to_string(),
        });
    }

    issues
}

fn candidate_recency_findings(
    reference_id: &str,
    candidate: &ProviderCandidate,
    policy: &SourcerightPolicy,
) -> Vec<PolicyIssue> {
    let payload = candidate.data.to_string().to_ascii_lowercase();
    let mut findings = Vec::new();

    if payload.contains("retract") {
        findings.push(PolicyIssue {
            severity: PolicyIssueSeverity::Error,
            reference_id: Some(reference_id.to_string()),
            code: "policy.recency.provider.retraction".to_string(),
            message: "Provider evidence suggests the record is retracted and should be reviewed."
                .to_string(),
        });
    }

    if payload.contains("expression of concern") || payload.contains("expressions of concern") {
        findings.push(PolicyIssue {
            severity: PolicyIssueSeverity::Warning,
            reference_id: Some(reference_id.to_string()),
            code: "policy.recency.provider.expression_of_concern".to_string(),
            message:
                "Provider evidence reports an expression of concern and should be treated conservatively."
                    .to_string(),
        });
    }

    if payload.contains("erratum") || payload.contains("correction") {
        findings.push(PolicyIssue {
            severity: PolicyIssueSeverity::Info,
            reference_id: Some(reference_id.to_string()),
            code: "policy.recency.provider.correction".to_string(),
            message:
                "Provider evidence reports a correction or erratum; review the linked record for context."
                    .to_string(),
        });
    }

    if payload.contains("preprint") {
        findings.push(PolicyIssue {
            severity: PolicyIssueSeverity::Info,
            reference_id: Some(reference_id.to_string()),
            code: "policy.recency.provider.preprint".to_string(),
            message:
                "Provider evidence identifies a preprint record rather than a final published version."
                    .to_string(),
        });
    }

    if payload.contains("supersed") {
        findings.push(PolicyIssue {
            severity: PolicyIssueSeverity::Warning,
            reference_id: Some(reference_id.to_string()),
            code: "policy.recency.provider.superseded_guideline".to_string(),
            message: "Provider evidence suggests this guideline or record has been superseded."
                .to_string(),
        });
    }

    if let Some(year) = candidate_publication_year(&candidate.data)
        && policy.recency.current_year.saturating_sub(year)
            > policy.recency.publication_age_warning_years.unwrap_or(0)
    {
        findings.push(PolicyIssue {
            severity: PolicyIssueSeverity::Warning,
            reference_id: Some(reference_id.to_string()),
            code: "policy.recency.provider.publication_age".to_string(),
            message: format!(
                "Provider evidence indicates a publication year of {year}, which is older than the configured warning threshold."
            ),
        });
    }

    findings
}

fn candidate_publication_year(value: &serde_json::Value) -> Option<u16> {
    match value {
        serde_json::Value::Object(map) => {
            for key in ["publication_year", "publicationYear", "year"] {
                if let Some(year) = map.get(key).and_then(value_to_year) {
                    return Some(year);
                }
            }
            for nested_key in ["published", "issued", "publication", "date"] {
                if let Some(year) = map.get(nested_key).and_then(candidate_publication_year) {
                    return Some(year);
                }
            }
            for nested in map.values() {
                if let Some(year) = candidate_publication_year(nested) {
                    return Some(year);
                }
            }
            None
        }
        serde_json::Value::Array(items) => items
            .iter()
            .find_map(candidate_publication_year)
            .or_else(|| items.first().and_then(value_to_year)),
        _ => value_to_year(value),
    }
}

fn value_to_year(value: &serde_json::Value) -> Option<u16> {
    match value {
        serde_json::Value::Number(number) => {
            number.as_u64().and_then(|year| u16::try_from(year).ok())
        }
        serde_json::Value::String(text) => text
            .trim()
            .parse::<u16>()
            .ok()
            .filter(|year| (1000..=3000).contains(year)),
        serde_json::Value::Array(items) => items.iter().find_map(value_to_year),
        serde_json::Value::Object(map) => map.values().find_map(value_to_year),
        serde_json::Value::Bool(_) | serde_json::Value::Null => None,
    }
}

fn doi_like_item(item: &CslItem) -> bool {
    matches!(
        item.item_type.as_str(),
        "article" | "article-journal" | "paper-conference" | "dataset" | "software" | "report"
    )
}

fn issued_year(item: &CslItem) -> Option<u16> {
    item.extra
        .get("issued")
        .and_then(|issued| issued.get("date-parts"))
        .and_then(|date_parts| date_parts.as_array())
        .and_then(|parts| parts.first())
        .and_then(|first| first.as_array())
        .and_then(|parts| parts.first())
        .and_then(|year| year.as_u64())
        .and_then(|year| u16::try_from(year).ok())
}

fn is_alphabetical_by_title(items: &[CslItem]) -> bool {
    let titles = items
        .iter()
        .map(|item| {
            item.title
                .as_deref()
                .unwrap_or(&item.id)
                .to_ascii_lowercase()
        })
        .collect::<Vec<_>>();
    titles.windows(2).all(|pair| pair[0] <= pair[1])
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::sidecar::{ProviderCandidate, ReferenceVerification, VerificationSidecar};

    #[test]
    fn doi_policy_warns_for_doi_capable_reference_without_doi() {
        let document = CslDocument {
            items: vec![CslItem {
                id: "smith-2024".to_string(),
                item_type: "article-journal".to_string(),
                title: Some("Example".to_string()),
                doi: None,
                extra: Default::default(),
            }],
        };

        let report = evaluate_policy(&document, &SourcerightPolicy::journal_vancouver());

        assert_eq!(report.issues[0].code, "policy.doi.missing");
        assert_eq!(report.issues[0].reference_id.as_deref(), Some("smith-2024"));
    }

    #[test]
    fn recency_policy_warns_for_old_publication_year() {
        let document = CslDocument {
            items: vec![CslItem {
                id: "old-guideline".to_string(),
                item_type: "report".to_string(),
                title: Some("Old guideline".to_string()),
                doi: Some("10.1234/old".to_string()),
                extra: [("issued".to_string(), json!({"date-parts": [[2010]]}))]
                    .into_iter()
                    .collect(),
            }],
        };

        let report = evaluate_policy(&document, &SourcerightPolicy::journal_vancouver());

        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.code == "policy.recency.publication_age")
        );
    }

    #[test]
    fn provider_backed_recency_evidence_is_classified() {
        let document = CslDocument {
            items: vec![CslItem {
                id: "retracted-2024".to_string(),
                item_type: "article-journal".to_string(),
                title: Some("Retracted paper".to_string()),
                doi: Some("10.1234/retracted".to_string()),
                extra: Default::default(),
            }],
        };
        let mut sidecar = VerificationSidecar::empty();
        sidecar.references.insert(
            "retracted-2024".to_string(),
            ReferenceVerification {
                provider_candidates: vec![ProviderCandidate {
                    provider: "crossref".to_string(),
                    confidence: 0.9,
                    retrieved_at: "2026-05-10T00:00:00Z".to_string(),
                    data: json!({"status": "retracted", "publication_year": 2012}),
                }],
                ..ReferenceVerification::default()
            },
        );

        let report = evaluate_policy_with_verification(
            &document,
            &sidecar,
            &SourcerightPolicy::journal_vancouver(),
        );

        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.code == "policy.recency.provider.retraction")
        );
        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.code == "policy.recency.provider.publication_age")
        );
    }
}
