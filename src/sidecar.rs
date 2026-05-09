use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const SIDECAR_SCHEMA_VERSION: &str = "sourceright.verification.v1";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerificationSidecar {
    pub schema_version: String,
    pub references: BTreeMap<String, ReferenceVerification>,
}

impl VerificationSidecar {
    pub fn empty() -> Self {
        Self {
            schema_version: SIDECAR_SCHEMA_VERSION.to_string(),
            references: BTreeMap::new(),
        }
    }

    pub fn review_queue_entries(&self) -> Vec<ReviewQueueEntry> {
        self.references
            .iter()
            .filter(|(_, verification)| verification.requires_review_queue())
            .map(|(id, verification)| ReviewQueueEntry::from_reference(id, verification))
            .collect()
    }

    pub fn to_review_queue_jsonl(&self) -> serde_json::Result<String> {
        let mut jsonl = String::new();

        for entry in self.review_queue_entries() {
            jsonl.push_str(&serde_json::to_string(&entry)?);
            jsonl.push('\n');
        }

        Ok(jsonl)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviewQueueEntry {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extraction: Option<ExtractionProvenance>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub provider_candidates: Vec<ProviderCandidate>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conflicts: Vec<Value>,
    pub review_status: ReviewStatus,
}

impl ReviewQueueEntry {
    fn from_reference(id: &str, verification: &ReferenceVerification) -> Self {
        Self {
            id: id.to_string(),
            extraction: verification.extraction.clone(),
            provider_candidates: verification.provider_candidates.clone(),
            conflicts: verification.conflicts.clone(),
            review_status: verification.review_status.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReferenceVerification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extraction: Option<ExtractionProvenance>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub provider_candidates: Vec<ProviderCandidate>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conflicts: Vec<Value>,
    pub review_status: ReviewStatus,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub review_decisions: Vec<ReviewDecision>,
}

impl Default for ReferenceVerification {
    fn default() -> Self {
        Self {
            extraction: None,
            provider_candidates: Vec::new(),
            conflicts: Vec::new(),
            review_status: ReviewStatus::NotRequired,
            review_decisions: Vec::new(),
        }
    }
}

impl ReferenceVerification {
    pub fn requires_review_queue(&self) -> bool {
        self.review_status.requires_review_queue()
    }

    pub fn has_provider_evidence(&self) -> bool {
        self.provider_candidates
            .iter()
            .any(ProviderCandidate::has_supported_shape)
    }

    pub fn has_conflicts(&self) -> bool {
        !self.conflicts.is_empty()
    }

    pub fn latest_review_decision(&self) -> Option<&ReviewDecision> {
        self.review_decisions.last()
    }

    pub fn transition_review_status(
        &mut self,
        next: ReviewStatus,
    ) -> Result<(), ReviewStatusTransitionError> {
        if self.review_status.can_transition_to(&next) {
            self.review_status = next;
            Ok(())
        } else {
            Err(ReviewStatusTransitionError {
                from: self.review_status.clone(),
                to: next,
            })
        }
    }

    pub fn record_review_decision(
        &mut self,
        decision: ReviewDecision,
        next_status: ReviewStatus,
    ) -> Result<(), ReviewStatusTransitionError> {
        self.transition_review_status(next_status)?;
        self.review_decisions.push(decision);
        Ok(())
    }

    pub fn invariant_issues(&self) -> Vec<SidecarInvariantIssue> {
        let mut issues = Vec::new();

        for candidate in &self.provider_candidates {
            if !candidate.has_supported_shape() {
                issues.push(SidecarInvariantIssue::new(
                    "sidecar.provider_candidate.invalid",
                    "Provider candidates must name a provider, include a retrieval timestamp, carry data, and use confidence from 0.0 to 1.0.",
                ));
            }
        }

        for conflict in &self.conflicts {
            if !conflict_has_supported_shape(conflict) {
                issues.push(SidecarInvariantIssue::new(
                    "sidecar.conflict.invalid",
                    "Conflicts must identify a field, severity, and provider or source.",
                ));
            }
        }

        for decision in &self.review_decisions {
            if !decision.has_supported_shape() {
                issues.push(SidecarInvariantIssue::new(
                    "sidecar.review_decision.invalid",
                    "Review decisions must include decision, reviewer, and decided_at values.",
                ));
            }
        }

        if self.review_status.is_terminal()
            && self.has_conflicts()
            && self.latest_review_decision().is_none()
        {
            issues.push(SidecarInvariantIssue::new(
                "sidecar.review_decision.missing",
                "Terminal review status on a conflicted record must retain review decision evidence.",
            ));
        }

        issues
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtractionProvenance {
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProviderCandidate {
    pub provider: String,
    pub confidence: f64,
    pub retrieved_at: String,
    pub data: Value,
}

impl ProviderCandidate {
    pub fn has_supported_shape(&self) -> bool {
        !self.provider.trim().is_empty()
            && !self.retrieved_at.trim().is_empty()
            && self.confidence.is_finite()
            && (0.0..=1.0).contains(&self.confidence)
            && !self.data.is_null()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewStatus {
    NotRequired,
    Queued,
    InProgress,
    Resolved,
    Unresolved,
}

impl ReviewStatus {
    pub fn requires_review_queue(&self) -> bool {
        matches!(self, Self::Queued | Self::InProgress | Self::Unresolved)
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Resolved | Self::Unresolved)
    }

    pub fn can_transition_to(&self, next: &Self) -> bool {
        use ReviewStatus::*;

        self == next
            || matches!(
                (self, next),
                (NotRequired, Queued)
                    | (NotRequired, Resolved)
                    | (NotRequired, Unresolved)
                    | (Queued, InProgress)
                    | (Queued, Resolved)
                    | (Queued, Unresolved)
                    | (InProgress, Queued)
                    | (InProgress, Resolved)
                    | (InProgress, Unresolved)
                    | (Resolved, Queued)
                    | (Resolved, Unresolved)
                    | (Unresolved, Queued)
                    | (Unresolved, InProgress)
                    | (Unresolved, Resolved)
            )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReviewDecision {
    pub decision: String,
    pub reviewer: String,
    pub decided_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl ReviewDecision {
    pub fn has_supported_shape(&self) -> bool {
        !self.decision.trim().is_empty()
            && !self.reviewer.trim().is_empty()
            && !self.decided_at.trim().is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SidecarInvariantIssue {
    pub code: &'static str,
    pub message: &'static str,
}

impl SidecarInvariantIssue {
    fn new(code: &'static str, message: &'static str) -> Self {
        Self { code, message }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewStatusTransitionError {
    pub from: ReviewStatus,
    pub to: ReviewStatus,
}

fn conflict_has_supported_shape(conflict: &Value) -> bool {
    let Some(object) = conflict.as_object() else {
        return false;
    };

    has_non_empty_string(object.get("field"))
        && has_non_empty_string(object.get("severity"))
        && (has_non_empty_string(object.get("provider"))
            || has_non_empty_string(object.get("source")))
}

fn has_non_empty_string(value: Option<&Value>) -> bool {
    value
        .and_then(Value::as_str)
        .is_some_and(|text| !text.trim().is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn empty_sidecar_has_stable_schema_version() {
        let sidecar = VerificationSidecar::empty();
        let json = serde_json::to_string_pretty(&sidecar).expect("serialize sidecar");

        assert!(json.contains(SIDECAR_SCHEMA_VERSION));
        assert!(sidecar.references.is_empty());
    }

    #[test]
    fn provider_candidate_shape_requires_identity_bounded_confidence_and_data() {
        let valid = ProviderCandidate {
            provider: "crossref".to_string(),
            confidence: 0.91,
            retrieved_at: "2026-05-09T00:00:00Z".to_string(),
            data: json!({"DOI": "10.0000/example"}),
        };

        let invalid = ProviderCandidate {
            provider: " ".to_string(),
            confidence: 1.2,
            retrieved_at: "".to_string(),
            data: Value::Null,
        };

        assert!(valid.has_supported_shape());
        assert!(!invalid.has_supported_shape());
    }

    #[test]
    fn invariant_issues_flag_unactionable_conflicts() {
        let verification = ReferenceVerification {
            conflicts: vec![
                json!({
                    "field": "issued",
                    "severity": "review",
                    "provider": "crossref"
                }),
                json!({
                    "field": "title",
                    "severity": "review"
                }),
            ],
            ..ReferenceVerification::default()
        };

        let issues = verification.invariant_issues();

        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].code, "sidecar.conflict.invalid");
    }

    #[test]
    fn review_decision_helper_moves_queued_records_to_resolved() {
        let mut verification = ReferenceVerification {
            review_status: ReviewStatus::Queued,
            conflicts: vec![json!({
                "field": "issued",
                "severity": "review",
                "source": "manual_import"
            })],
            ..ReferenceVerification::default()
        };

        verification
            .record_review_decision(
                ReviewDecision {
                    decision: "accepted canonical year".to_string(),
                    reviewer: "agent:worker-b".to_string(),
                    decided_at: "2026-05-09T00:00:00Z".to_string(),
                    notes: Some("Provider candidate had stale metadata.".to_string()),
                },
                ReviewStatus::Resolved,
            )
            .expect("valid review transition");

        assert_eq!(verification.review_status, ReviewStatus::Resolved);
        assert_eq!(
            verification
                .latest_review_decision()
                .expect("decision recorded")
                .decision,
            "accepted canonical year"
        );
        assert!(verification.invariant_issues().is_empty());
    }

    #[test]
    fn invalid_review_status_transition_is_rejected() {
        let mut verification = ReferenceVerification {
            review_status: ReviewStatus::Resolved,
            ..ReferenceVerification::default()
        };

        let error = verification
            .transition_review_status(ReviewStatus::InProgress)
            .expect_err("resolved records must be explicitly reopened first");

        assert_eq!(error.from, ReviewStatus::Resolved);
        assert_eq!(error.to, ReviewStatus::InProgress);
        assert_eq!(verification.review_status, ReviewStatus::Resolved);
    }

    #[test]
    fn unresolved_and_active_records_remain_queue_eligible() {
        assert!(ReviewStatus::Queued.requires_review_queue());
        assert!(ReviewStatus::InProgress.requires_review_queue());
        assert!(ReviewStatus::Unresolved.requires_review_queue());
        assert!(!ReviewStatus::NotRequired.requires_review_queue());
        assert!(!ReviewStatus::Resolved.requires_review_queue());
    }

    #[test]
    fn review_queue_jsonl_is_derived_and_sorted_by_reference_id() {
        let mut sidecar = VerificationSidecar::empty();
        sidecar.references.insert(
            "zeta-queued".to_string(),
            ReferenceVerification {
                extraction: Some(ExtractionProvenance {
                    source: "input.docx".to_string(),
                    original_text: Some("Zeta queued reference".to_string()),
                    span: Some("paragraph:2".to_string()),
                }),
                provider_candidates: vec![ProviderCandidate {
                    provider: "crossref".to_string(),
                    confidence: 0.42,
                    retrieved_at: "2026-05-09T00:00:00Z".to_string(),
                    data: json!({"DOI": "10.0000/zeta"}),
                }],
                conflicts: vec![json!({
                    "field": "issued",
                    "severity": "review",
                    "provider": "crossref"
                })],
                review_status: ReviewStatus::Queued,
                ..ReferenceVerification::default()
            },
        );
        sidecar.references.insert(
            "alpha-unresolved".to_string(),
            ReferenceVerification {
                review_status: ReviewStatus::Unresolved,
                ..ReferenceVerification::default()
            },
        );
        sidecar.references.insert(
            "middle-resolved".to_string(),
            ReferenceVerification {
                review_status: ReviewStatus::Resolved,
                ..ReferenceVerification::default()
            },
        );

        let jsonl = sidecar
            .to_review_queue_jsonl()
            .expect("serialize review queue jsonl");
        let lines = jsonl.lines().collect::<Vec<_>>();

        assert_eq!(lines.len(), 2);
        assert_eq!(
            lines[0],
            r#"{"id":"alpha-unresolved","review_status":"unresolved"}"#
        );
        assert_eq!(
            lines[1],
            r#"{"id":"zeta-queued","extraction":{"source":"input.docx","original_text":"Zeta queued reference","span":"paragraph:2"},"provider_candidates":[{"provider":"crossref","confidence":0.42,"retrieved_at":"2026-05-09T00:00:00Z","data":{"DOI":"10.0000/zeta"}}],"conflicts":[{"field":"issued","provider":"crossref","severity":"review"}],"review_status":"queued"}"#
        );
        assert!(jsonl.ends_with('\n'));
    }
}
