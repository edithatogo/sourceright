use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::csl::{CslItem, normalize_doi, normalize_title};
use crate::sidecar::ProviderCandidate;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AcademicProviderResult {
    pub provider: AcademicProvider,
    pub status: ProviderResultStatus,
    pub candidates: Vec<ProviderCandidate>,
    pub errors: Vec<ProviderErrorEvidence>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AcademicProvider {
    Crossref,
    DoiResolver,
    DataCite,
    OpenAlex,
    PubMed,
    Orcid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderResultStatus {
    Match,
    NoMatch,
    Ambiguous,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderErrorEvidence {
    pub code: String,
    pub message: String,
}

pub fn crossref_candidate_from_work(
    retrieved_at: &str,
    canonical: &CslItem,
    work: &Value,
) -> AcademicProviderResult {
    let Some(object) = work.as_object() else {
        return provider_error(
            AcademicProvider::Crossref,
            "provider.crossref.malformed_response",
            "Crossref work response was not a JSON object.",
        );
    };

    let doi = object
        .get("DOI")
        .and_then(Value::as_str)
        .map(normalize_doi)
        .unwrap_or_default();
    let title = object
        .get("title")
        .and_then(Value::as_array)
        .and_then(|titles| titles.first())
        .and_then(Value::as_str)
        .map(normalize_title)
        .unwrap_or_default();

    if doi.is_empty() && title.is_empty() {
        return AcademicProviderResult {
            provider: AcademicProvider::Crossref,
            status: ProviderResultStatus::NoMatch,
            candidates: Vec::new(),
            errors: Vec::new(),
        };
    }

    let confidence = confidence_for_candidate(canonical, &doi, &title);
    let candidate = ProviderCandidate {
        provider: "crossref".to_string(),
        confidence,
        retrieved_at: retrieved_at.to_string(),
        data: work.clone(),
    };

    AcademicProviderResult {
        provider: AcademicProvider::Crossref,
        status: ProviderResultStatus::Match,
        candidates: vec![candidate],
        errors: Vec::new(),
    }
}

pub fn doi_resolution_evidence(
    doi: &str,
    retrieved_at: &str,
    final_url: Option<&str>,
    error: Option<&str>,
) -> AcademicProviderResult {
    if let Some(error) = error {
        return AcademicProviderResult {
            provider: AcademicProvider::DoiResolver,
            status: ProviderResultStatus::Error,
            candidates: Vec::new(),
            errors: vec![ProviderErrorEvidence {
                code: "provider.doi_resolution.failed".to_string(),
                message: error.to_string(),
            }],
        };
    }

    let data = serde_json::json!({
        "DOI": normalize_doi(doi),
        "retrieved_at": retrieved_at,
        "final_url": final_url,
    });

    AcademicProviderResult {
        provider: AcademicProvider::DoiResolver,
        status: ProviderResultStatus::Match,
        candidates: vec![ProviderCandidate {
            provider: "doi-resolver".to_string(),
            confidence: 1.0,
            retrieved_at: retrieved_at.to_string(),
            data,
        }],
        errors: Vec::new(),
    }
}

pub fn provider_error(
    provider: AcademicProvider,
    code: &str,
    message: &str,
) -> AcademicProviderResult {
    AcademicProviderResult {
        provider,
        status: ProviderResultStatus::Error,
        candidates: Vec::new(),
        errors: vec![ProviderErrorEvidence {
            code: code.to_string(),
            message: message.to_string(),
        }],
    }
}

fn confidence_for_candidate(canonical: &CslItem, doi: &str, title: &str) -> f64 {
    let canonical_doi = canonical
        .doi
        .as_deref()
        .map(normalize_doi)
        .unwrap_or_default();
    if !doi.is_empty() && doi == canonical_doi {
        return 1.0;
    }

    let canonical_title = canonical
        .title
        .as_deref()
        .map(normalize_title)
        .unwrap_or_default();
    if !title.is_empty() && title == canonical_title {
        return 0.85;
    }

    0.5
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde_json::json;

    use super::*;

    #[test]
    fn crossref_doi_lookup_records_candidate_and_confidence_inputs() {
        let canonical = CslItem {
            id: "smith-2024".to_string(),
            item_type: "article-journal".to_string(),
            title: Some("Trial paper".to_string()),
            doi: Some("10.1000/example".to_string()),
            extra: BTreeMap::new(),
        };

        let result = crossref_candidate_from_work(
            "2026-05-09T00:00:00Z",
            &canonical,
            &json!({"DOI": "https://doi.org/10.1000/EXAMPLE", "title": ["Trial paper"]}),
        );

        assert_eq!(result.status, ProviderResultStatus::Match);
        assert_eq!(result.candidates[0].provider, "crossref");
        assert_eq!(result.candidates[0].confidence, 1.0);
        assert_eq!(result.candidates[0].retrieved_at, "2026-05-09T00:00:00Z");
    }

    #[test]
    fn doi_resolution_records_reachability_without_bibliographic_overwrite() {
        let result = doi_resolution_evidence(
            "10.1000/example",
            "2026-05-09T00:00:00Z",
            Some("https://publisher.example/article"),
            None,
        );

        assert_eq!(result.provider, AcademicProvider::DoiResolver);
        assert_eq!(
            result.candidates[0].data["final_url"],
            "https://publisher.example/article"
        );
        assert!(result.errors.is_empty());
    }

    #[test]
    fn malformed_provider_response_is_error_evidence() {
        let canonical = CslItem {
            id: "smith-2024".to_string(),
            item_type: "article-journal".to_string(),
            title: Some("Trial paper".to_string()),
            doi: None,
            extra: BTreeMap::new(),
        };

        let result = crossref_candidate_from_work(
            "2026-05-09T00:00:00Z",
            &canonical,
            &Value::String("not an object".to_string()),
        );

        assert_eq!(result.status, ProviderResultStatus::Error);
        assert_eq!(
            result.errors[0].code,
            "provider.crossref.malformed_response"
        );
        assert!(result.candidates.is_empty());
    }
}
