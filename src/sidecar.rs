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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewStatus {
    NotRequired,
    Queued,
    InProgress,
    Resolved,
    Unresolved,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_sidecar_has_stable_schema_version() {
        let sidecar = VerificationSidecar::empty();
        let json = serde_json::to_string_pretty(&sidecar).expect("serialize sidecar");

        assert!(json.contains(SIDECAR_SCHEMA_VERSION));
        assert!(sidecar.references.is_empty());
    }
}
