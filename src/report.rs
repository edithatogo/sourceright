use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::csl::CslDocument;
use crate::policy::{
    PolicyIssue, PolicyIssueSeverity, SourcerightPolicy, provider_backed_recency_issues,
};
use crate::sidecar::{ReviewStatus, VerificationSidecar};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferenceReport {
    pub total_references: usize,
    pub verified_references: usize,
    pub review_queue_count: usize,
    pub unresolved_count: usize,
    pub conflict_count: usize,
    pub ai_risk_issue_count: usize,
    pub error_count: usize,
    pub warning_count: usize,
    pub info_count: usize,
    pub issues: Vec<ReferenceReportIssue>,
}

impl ReferenceReport {
    pub fn from_documents(csl: &CslDocument, sidecar: &VerificationSidecar) -> Self {
        let mut issues = Vec::new();

        for diagnostic in csl.validate() {
            issues.push(ReferenceReportIssue {
                severity: ReferenceReportSeverity::Error,
                category: category_for_issue_code(&diagnostic.code),
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
                    category: ReferenceReportCategory::CslIntegrity,
                    reference_id: Some(item.id.clone()),
                    code: "report.duplicate_id".to_string(),
                    message: "Duplicate CSL item id; downstream verification and exports need stable unique ids".to_string(),
                    ai_risk_signal: true,
                });
            }

            if item.doi.as_deref().unwrap_or_default().trim().is_empty() {
                issues.push(ReferenceReportIssue {
                    severity: ReferenceReportSeverity::Warning,
                    category: ReferenceReportCategory::Identifier,
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
                            category: ReferenceReportCategory::VerificationCoverage,
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
                            category: ReferenceReportCategory::ProviderConflict,
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
                            category: ReferenceReportCategory::ManualReview,
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
                    category: ReferenceReportCategory::VerificationCoverage,
                    reference_id: Some(item.id.clone()),
                    code: "report.missing_sidecar_entry".to_string(),
                    message: "Reference has no matching verification sidecar entry".to_string(),
                    ai_risk_signal: true,
                }),
            }
        }

        for issue in
            provider_backed_recency_issues(csl, sidecar, &SourcerightPolicy::journal_vancouver())
        {
            issues.push(reference_report_issue_from_policy_issue(issue));
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

        let ai_risk_issue_count = issues.iter().filter(|issue| issue.ai_risk_signal).count();
        let error_count = issues
            .iter()
            .filter(|issue| issue.severity == ReferenceReportSeverity::Error)
            .count();
        let warning_count = issues
            .iter()
            .filter(|issue| issue.severity == ReferenceReportSeverity::Warning)
            .count();
        let info_count = issues
            .iter()
            .filter(|issue| issue.severity == ReferenceReportSeverity::Info)
            .count();

        Self {
            total_references: csl.items.len(),
            verified_references,
            review_queue_count,
            unresolved_count,
            conflict_count,
            ai_risk_issue_count,
            error_count,
            warning_count,
            info_count,
            issues,
        }
    }

    pub fn to_markdown(&self) -> String {
        let mut markdown = String::new();
        markdown.push_str("# Sourceright Reference Report\n\n");
        markdown.push_str("This report identifies reference integrity risks, including patterns commonly seen in AI-assisted citation errors. It is an audit report, not an automatic correction record.\n\n");
        markdown.push_str("## Summary\n\n");
        markdown.push_str(&format!(
            "- Total references: {}\n- References with provider candidates: {}\n- Manual review queue: {}\n- Unresolved reviews: {}\n- Provider conflicts: {}\n- Issues: {}\n- AI-risk issue signals: {}\n- Severity totals: {} error, {} warning, {} info\n\n",
            self.total_references,
            self.verified_references,
            self.review_queue_count,
            self.unresolved_count,
            self.conflict_count,
            self.issues.len(),
            self.ai_risk_issue_count,
            self.error_count,
            self.warning_count,
            self.info_count
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
                    "- `{}` `{}` `{}` `{}`: {}{}\n",
                    issue.severity.as_str(),
                    issue.category.as_str(),
                    reference,
                    issue.code,
                    issue.message,
                    ai_signal
                ));
            }
        }

        markdown
    }

    pub fn to_json_output(&self) -> ReferenceReportJsonOutput {
        ReferenceReportJsonOutput {
            schema_version: REFERENCE_REPORT_SCHEMA_VERSION.to_string(),
            report_type: "reference_integrity".to_string(),
            summary: ReferenceReportSummary {
                total_references: self.total_references,
                verified_references: self.verified_references,
                review_queue_count: self.review_queue_count,
                unresolved_count: self.unresolved_count,
                conflict_count: self.conflict_count,
                ai_risk_issue_count: self.ai_risk_issue_count,
                error_count: self.error_count,
                warning_count: self.warning_count,
                info_count: self.info_count,
            },
            issues: self.issues.clone(),
        }
    }

    pub fn to_mcp_resource(&self) -> ReferenceReportResource {
        ReferenceReportResource {
            uri: "sourceright://reports/reference-integrity".to_string(),
            mime_type: "application/json".to_string(),
            data: self.to_json_output(),
        }
    }
}

pub const REFERENCE_REPORT_SCHEMA_VERSION: &str = "sourceright.reference_report.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferenceReportJsonOutput {
    pub schema_version: String,
    pub report_type: String,
    pub summary: ReferenceReportSummary,
    pub issues: Vec<ReferenceReportIssue>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferenceReportSummary {
    pub total_references: usize,
    pub verified_references: usize,
    pub review_queue_count: usize,
    pub unresolved_count: usize,
    pub conflict_count: usize,
    pub ai_risk_issue_count: usize,
    pub error_count: usize,
    pub warning_count: usize,
    pub info_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferenceReportResource {
    pub uri: String,
    pub mime_type: String,
    pub data: ReferenceReportJsonOutput,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferenceReportIssue {
    pub severity: ReferenceReportSeverity,
    pub category: ReferenceReportCategory,
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

impl ReferenceReportSeverity {
    fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferenceReportCategory {
    CslIntegrity,
    Identifier,
    VerificationCoverage,
    ProviderConflict,
    ManualReview,
    RecencyEvidence,
    SidecarBoundary,
}

impl ReferenceReportCategory {
    fn as_str(self) -> &'static str {
        match self {
            Self::CslIntegrity => "csl_integrity",
            Self::Identifier => "identifier",
            Self::VerificationCoverage => "verification_coverage",
            Self::ProviderConflict => "provider_conflict",
            Self::ManualReview => "manual_review",
            Self::RecencyEvidence => "recency_evidence",
            Self::SidecarBoundary => "sidecar_boundary",
        }
    }
}

fn category_for_issue_code(code: &str) -> ReferenceReportCategory {
    if code.starts_with("policy.recency.") {
        return ReferenceReportCategory::RecencyEvidence;
    }
    match code {
        "csl.sidecar_field" => ReferenceReportCategory::SidecarBoundary,
        "csl.id.empty" | "csl.type.empty" | "csl.title.empty" => {
            ReferenceReportCategory::CslIntegrity
        }
        _ => ReferenceReportCategory::CslIntegrity,
    }
}

fn reference_id_from_path(csl: &CslDocument, path: &str) -> Option<String> {
    let index = path
        .strip_prefix("$[")
        .and_then(|rest| rest.split(']').next())
        .and_then(|index| index.parse::<usize>().ok())?;
    csl.items.get(index).map(|item| item.id.clone())
}

fn reference_report_issue_from_policy_issue(issue: PolicyIssue) -> ReferenceReportIssue {
    let severity = match issue.severity {
        PolicyIssueSeverity::Info => ReferenceReportSeverity::Info,
        PolicyIssueSeverity::Warning => ReferenceReportSeverity::Warning,
        PolicyIssueSeverity::Error => ReferenceReportSeverity::Error,
    };

    ReferenceReportIssue {
        severity,
        category: category_for_issue_code(&issue.code),
        reference_id: issue.reference_id,
        code: issue.code,
        message: issue.message,
        ai_risk_signal: true,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde_json::json;

    use super::*;
    use crate::csl::CslItem;
    use crate::sidecar::{ProviderCandidate, ReferenceVerification, VerificationSidecar};

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
        assert_eq!(report.ai_risk_issue_count, 2);
        assert_eq!(report.error_count, 0);
        assert_eq!(report.warning_count, 2);
        assert_eq!(report.info_count, 0);
        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.code == "report.missing_doi"
                    && issue.category == ReferenceReportCategory::Identifier
                    && issue.ai_risk_signal)
        );
        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.code == "report.unverified_reference"
                    && issue.category == ReferenceReportCategory::VerificationCoverage
                    && issue.ai_risk_signal)
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
        assert!(report.to_markdown().contains("AI-risk issue signals: 1"));
        assert!(
            report
                .to_markdown()
                .contains("`warning` `provider_conflict`")
        );
    }

    #[test]
    fn report_surfaces_provider_backed_recency_evidence() {
        let csl = CslDocument {
            items: vec![CslItem {
                id: "retracted-2024".to_string(),
                item_type: "article-journal".to_string(),
                title: Some("Retracted paper".to_string()),
                doi: Some("10.1234/retracted".to_string()),
                extra: BTreeMap::new(),
            }],
        };
        let mut sidecar = VerificationSidecar::empty();
        sidecar.references.insert(
            "retracted-2024".to_string(),
            ReferenceVerification {
                provider_candidates: vec![ProviderCandidate {
                    provider: "crossref".to_string(),
                    confidence: 0.93,
                    retrieved_at: "2026-05-10T00:00:00Z".to_string(),
                    data: json!({"status": "retracted", "publication_year": 2011}),
                }],
                ..ReferenceVerification::default()
            },
        );

        let report = ReferenceReport::from_documents(&csl, &sidecar);

        assert!(report.issues.iter().any(|issue| issue.category
            == ReferenceReportCategory::RecencyEvidence
            && issue.code == "policy.recency.provider.retraction"));
        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.code == "policy.recency.provider.publication_age")
        );
    }

    #[test]
    fn report_categorizes_csl_sidecar_boundary_violations() {
        let csl = CslDocument {
            items: vec![CslItem {
                id: "doe-2025".to_string(),
                item_type: "article-journal".to_string(),
                title: Some("Boundary check".to_string()),
                doi: Some("10.1234/example".to_string()),
                extra: BTreeMap::from([("confidence".to_string(), json!(0.9))]),
            }],
        };
        let sidecar = VerificationSidecar::empty();

        let report = ReferenceReport::from_documents(&csl, &sidecar);

        assert_eq!(report.error_count, 1);
        assert_eq!(report.warning_count, 1);
        assert_eq!(report.ai_risk_issue_count, 2);
        assert!(report.issues.iter().any(|issue| {
            issue.code == "csl.sidecar_field"
                && issue.category == ReferenceReportCategory::SidecarBoundary
                && issue.severity == ReferenceReportSeverity::Error
                && issue.reference_id.as_deref() == Some("doe-2025")
        }));
    }

    #[test]
    fn report_serializes_stable_json_output_model() {
        let csl = CslDocument {
            items: vec![CslItem {
                id: "smith-2024".to_string(),
                item_type: "article-journal".to_string(),
                title: Some("A plausible but unchecked paper".to_string()),
                doi: None,
                extra: BTreeMap::new(),
            }],
        };
        let sidecar = VerificationSidecar::empty();

        let report = ReferenceReport::from_documents(&csl, &sidecar);
        let output = report.to_json_output();
        let serialized = serde_json::to_value(&output).expect("serialize report json output");

        assert_eq!(
            serialized["schema_version"],
            "sourceright.reference_report.v1"
        );
        assert_eq!(serialized["report_type"], "reference_integrity");
        assert_eq!(serialized["summary"]["total_references"], 1);
        assert_eq!(serialized["summary"]["warning_count"], 2);
        assert_eq!(serialized["issues"][0]["severity"], "warning");
        assert_eq!(serialized["issues"][0]["category"], "identifier");
    }

    #[test]
    fn report_exposes_mcp_ready_json_resource_data() {
        let report = ReferenceReport {
            total_references: 0,
            verified_references: 0,
            review_queue_count: 0,
            unresolved_count: 0,
            conflict_count: 0,
            ai_risk_issue_count: 0,
            error_count: 0,
            warning_count: 0,
            info_count: 0,
            issues: Vec::new(),
        };

        let resource = report.to_mcp_resource();
        let serialized = serde_json::to_value(&resource).expect("serialize report resource");

        assert_eq!(
            serialized["uri"],
            "sourceright://reports/reference-integrity"
        );
        assert_eq!(serialized["mime_type"], "application/json");
        assert_eq!(
            serialized["data"]["schema_version"],
            "sourceright.reference_report.v1"
        );
        assert_eq!(serialized["data"]["summary"]["total_references"], 0);
    }
}
