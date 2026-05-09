use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::csl::CslDocument;
use crate::sidecar::{ReviewStatus, VerificationSidecar};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferenceReport {
    pub total_references: usize,
    pub verified_references: usize,
    pub review_queue_count: usize,
    pub unresolved_count: usize,
    pub conflict_count: usize,
    pub issues: Vec<ReferenceReportIssue>,
}

impl ReferenceReport {
    pub fn from_documents(csl: &CslDocument, sidecar: &VerificationSidecar) -> Self {
        let mut issues = Vec::new();

        for diagnostic in csl.validate() {
            issues.push(ReferenceReportIssue {
                severity: ReferenceReportSeverity::Error,
                reference_id: reference_id_from_path(csl, &diagnostic.path),
                code: diagnostic.code,
                message: diagnostic.message,
                ai_risk_signal: true,
            });
        }

        let mut seen_ids = BTreeSet::new();
        for item in &csl.items {
            if !seen_ids.insert(item.id.clone()) {
                issues.push(ReferenceReportIssue {
                    severity: ReferenceReportSeverity::Error,
                    reference_id: Some(item.id.clone()),
                    code: "report.duplicate_id".to_string(),
                    message: "Duplicate CSL item id; downstream verification and exports need stable unique ids".to_string(),
                    ai_risk_signal: true,
                });
            }

            if item.doi.as_deref().unwrap_or_default().trim().is_empty() {
                issues.push(ReferenceReportIssue {
                    severity: ReferenceReportSeverity::Warning,
                    reference_id: Some(item.id.clone()),
                    code: "report.missing_doi".to_string(),
                    message: "Reference has no DOI in canonical CSL JSON".to_string(),
                    ai_risk_signal: true,
                });
            }

            match sidecar.references.get(&item.id) {
                Some(verification) => {
                    if verification.provider_candidates.is_empty()
                        && verification.review_status == ReviewStatus::NotRequired
                    {
                        issues.push(ReferenceReportIssue {
                            severity: ReferenceReportSeverity::Warning,
                            reference_id: Some(item.id.clone()),
                            code: "report.unverified_reference".to_string(),
                            message: "Reference has no provider candidate or manual review state"
                                .to_string(),
                            ai_risk_signal: true,
                        });
                    }

                    if !verification.conflicts.is_empty() {
                        issues.push(ReferenceReportIssue {
                            severity: ReferenceReportSeverity::Warning,
                            reference_id: Some(item.id.clone()),
                            code: "report.provider_conflict".to_string(),
                            message: "Reference has unresolved provider conflict metadata"
                                .to_string(),
                            ai_risk_signal: true,
                        });
                    }

                    if matches!(
                        verification.review_status,
                        ReviewStatus::Queued | ReviewStatus::InProgress | ReviewStatus::Unresolved
                    ) {
                        issues.push(ReferenceReportIssue {
                            severity: ReferenceReportSeverity::Info,
                            reference_id: Some(item.id.clone()),
                            code: "report.manual_review_needed".to_string(),
                            message: format!(
                                "Reference review status is {:?}",
                                verification.review_status
                            ),
                            ai_risk_signal: false,
                        });
                    }
                }
                None => issues.push(ReferenceReportIssue {
                    severity: ReferenceReportSeverity::Warning,
                    reference_id: Some(item.id.clone()),
                    code: "report.missing_sidecar_entry".to_string(),
                    message: "Reference has no matching verification sidecar entry".to_string(),
                    ai_risk_signal: true,
                }),
            }
        }

        let verified_references = csl
            .items
            .iter()
            .filter(|item| {
                sidecar
                    .references
                    .get(&item.id)
                    .is_some_and(|verification| !verification.provider_candidates.is_empty())
            })
            .count();

        let review_queue_count = sidecar
            .references
            .values()
            .filter(|verification| {
                matches!(
                    verification.review_status,
                    ReviewStatus::Queued | ReviewStatus::InProgress
                )
            })
            .count();

        let unresolved_count = sidecar
            .references
            .values()
            .filter(|verification| verification.review_status == ReviewStatus::Unresolved)
            .count();

        let conflict_count = sidecar
            .references
            .values()
            .map(|verification| verification.conflicts.len())
            .sum();

        Self {
            total_references: csl.items.len(),
            verified_references,
            review_queue_count,
            unresolved_count,
            conflict_count,
            issues,
        }
    }

    pub fn to_markdown(&self) -> String {
        let mut markdown = String::new();
        markdown.push_str("# Sourceright Reference Report\n\n");
        markdown.push_str("This report identifies reference integrity risks, including patterns commonly seen in AI-assisted citation errors. It is an audit report, not an automatic correction record.\n\n");
        markdown.push_str("## Summary\n\n");
        markdown.push_str(&format!(
            "- Total references: {}\n- References with provider candidates: {}\n- Manual review queue: {}\n- Unresolved reviews: {}\n- Provider conflicts: {}\n- Issues: {}\n\n",
            self.total_references,
            self.verified_references,
            self.review_queue_count,
            self.unresolved_count,
            self.conflict_count,
            self.issues.len()
        ));

        markdown.push_str("## Issues\n\n");
        if self.issues.is_empty() {
            markdown.push_str("No reference integrity issues detected by the current checks.\n");
        } else {
            for issue in &self.issues {
                let reference = issue.reference_id.as_deref().unwrap_or("document");
                let ai_signal = if issue.ai_risk_signal {
                    " AI-risk signal."
                } else {
                    ""
                };
                markdown.push_str(&format!(
                    "- `{:?}` `{}` `{}`: {}{}\n",
                    issue.severity, reference, issue.code, issue.message, ai_signal
                ));
            }
        }

        markdown
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferenceReportIssue {
    pub severity: ReferenceReportSeverity,
    pub reference_id: Option<String>,
    pub code: String,
    pub message: String,
    pub ai_risk_signal: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferenceReportSeverity {
    Info,
    Warning,
    Error,
}

fn reference_id_from_path(csl: &CslDocument, path: &str) -> Option<String> {
    let index = path
        .strip_prefix("$[")
        .and_then(|rest| rest.split(']').next())
        .and_then(|index| index.parse::<usize>().ok())?;
    csl.items.get(index).map(|item| item.id.clone())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde_json::json;

    use super::*;
    use crate::csl::CslItem;
    use crate::sidecar::{ProviderCandidate, ReferenceVerification};

    #[test]
    fn report_flags_unverified_missing_doi_references_as_ai_risk_signals() {
        let csl = CslDocument {
            items: vec![CslItem {
                id: "smith-2024".to_string(),
                item_type: "article-journal".to_string(),
                title: Some("A plausible but unchecked paper".to_string()),
                doi: None,
                extra: BTreeMap::new(),
            }],
        };
        let mut sidecar = VerificationSidecar::empty();
        sidecar
            .references
            .insert("smith-2024".to_string(), ReferenceVerification::default());

        let report = ReferenceReport::from_documents(&csl, &sidecar);

        assert_eq!(report.total_references, 1);
        assert_eq!(report.verified_references, 0);
        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.code == "report.missing_doi" && issue.ai_risk_signal)
        );
        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.code == "report.unverified_reference" && issue.ai_risk_signal)
        );
    }

    #[test]
    fn report_counts_provider_candidates_and_conflicts() {
        let csl = CslDocument {
            items: vec![CslItem {
                id: "doe-2025".to_string(),
                item_type: "article-journal".to_string(),
                title: Some("Verified record".to_string()),
                doi: Some("10.1234/example".to_string()),
                extra: BTreeMap::new(),
            }],
        };
        let mut sidecar = VerificationSidecar::empty();
        sidecar.references.insert(
            "doe-2025".to_string(),
            ReferenceVerification {
                provider_candidates: vec![ProviderCandidate {
                    provider: "crossref".to_string(),
                    confidence: 0.98,
                    retrieved_at: "2026-05-09T00:00:00Z".to_string(),
                    data: json!({"DOI": "10.1234/example"}),
                }],
                conflicts: vec![json!({"field": "title"})],
                ..ReferenceVerification::default()
            },
        );

        let report = ReferenceReport::from_documents(&csl, &sidecar);

        assert_eq!(report.verified_references, 1);
        assert_eq!(report.conflict_count, 1);
        assert!(report.to_markdown().contains("Provider conflicts: 1"));
    }
}
