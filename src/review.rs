use serde::{Deserialize, Serialize};

use crate::sidecar::{
    ReviewDecision, ReviewQueueEntry, ReviewStatus, ReviewStatusTransitionError,
    VerificationSidecar,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviewPartition {
    pub id: String,
    pub entries: Vec<ReviewQueueEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewDecisionImport {
    pub reference_id: String,
    pub decision: String,
    pub reviewer: String,
    pub decided_at: String,
    pub status: ReviewStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewImportReport {
    pub applied: usize,
    pub errors: Vec<ReviewImportError>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewImportError {
    pub reference_id: String,
    pub message: String,
}

pub fn partition_review_queue(
    sidecar: &VerificationSidecar,
    max_entries: usize,
) -> Vec<ReviewPartition> {
    let max_entries = max_entries.max(1);
    sidecar
        .review_queue_entries()
        .chunks(max_entries)
        .enumerate()
        .map(|(index, entries)| ReviewPartition {
            id: format!("review-partition-{:03}", index + 1),
            entries: entries.to_vec(),
        })
        .collect()
}

pub fn apply_review_decisions(
    sidecar: &mut VerificationSidecar,
    decisions: &[ReviewDecisionImport],
) -> ReviewImportReport {
    let mut applied = 0;
    let mut errors = Vec::new();

    for decision in decisions {
        let Some(verification) = sidecar.references.get_mut(&decision.reference_id) else {
            errors.push(ReviewImportError {
                reference_id: decision.reference_id.clone(),
                message: "No verification sidecar entry exists for this reference id.".to_string(),
            });
            continue;
        };

        let review_decision = ReviewDecision {
            decision: decision.decision.clone(),
            reviewer: decision.reviewer.clone(),
            decided_at: decision.decided_at.clone(),
            notes: decision.notes.clone(),
        };

        match verification.record_review_decision(review_decision, decision.status.clone()) {
            Ok(()) => applied += 1,
            Err(error) => errors.push(import_error(&decision.reference_id, error)),
        }
    }

    ReviewImportReport { applied, errors }
}

fn import_error(reference_id: &str, error: ReviewStatusTransitionError) -> ReviewImportError {
    ReviewImportError {
        reference_id: reference_id.to_string(),
        message: format!(
            "Invalid review status transition from {:?} to {:?}.",
            error.from, error.to
        ),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::sidecar::{ReferenceVerification, VerificationSidecar};

    #[test]
    fn review_queue_partitions_are_stable_and_bounded() {
        let mut sidecar = VerificationSidecar::empty();
        for id in ["a", "b", "c"] {
            sidecar.references.insert(
                id.to_string(),
                ReferenceVerification {
                    review_status: ReviewStatus::Queued,
                    ..ReferenceVerification::default()
                },
            );
        }

        let partitions = partition_review_queue(&sidecar, 2);

        assert_eq!(partitions.len(), 2);
        assert_eq!(partitions[0].id, "review-partition-001");
        assert_eq!(partitions[0].entries.len(), 2);
        assert_eq!(partitions[1].entries.len(), 1);
    }

    #[test]
    fn review_decision_import_records_decisions_and_status() {
        let mut sidecar = VerificationSidecar {
            schema_version: crate::sidecar::SIDECAR_SCHEMA_VERSION.to_string(),
            references: BTreeMap::from([(
                "smith-2024".to_string(),
                ReferenceVerification {
                    review_status: ReviewStatus::Queued,
                    ..ReferenceVerification::default()
                },
            )]),
        };

        let report = apply_review_decisions(
            &mut sidecar,
            &[ReviewDecisionImport {
                reference_id: "smith-2024".to_string(),
                decision: "accepted provider DOI".to_string(),
                reviewer: "agent:reviewer-1".to_string(),
                decided_at: "2026-05-10T00:00:00Z".to_string(),
                status: ReviewStatus::Resolved,
                notes: Some("Matches publisher landing page.".to_string()),
            }],
        );

        assert_eq!(report.applied, 1);
        assert!(report.errors.is_empty());
        assert_eq!(
            sidecar.references["smith-2024"].review_status,
            ReviewStatus::Resolved
        );
        assert_eq!(
            sidecar.references["smith-2024"].review_decisions[0].decision,
            "accepted provider DOI"
        );
    }

    #[test]
    fn review_decision_import_reports_unknown_references() {
        let mut sidecar = VerificationSidecar::empty();

        let report = apply_review_decisions(
            &mut sidecar,
            &[ReviewDecisionImport {
                reference_id: "missing".to_string(),
                decision: "unresolved".to_string(),
                reviewer: "agent:reviewer-1".to_string(),
                decided_at: "2026-05-10T00:00:00Z".to_string(),
                status: ReviewStatus::Unresolved,
                notes: None,
            }],
        );

        assert_eq!(report.applied, 0);
        assert_eq!(report.errors[0].reference_id, "missing");
    }
}
