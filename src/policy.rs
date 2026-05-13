use reqwest::Url;
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
    issues.extend(provider_backed_url_archive_issues(document, sidecar));

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

    for item in &document.items {
        if let Some(raw_url) = canonical_url_text(item)
            && parse_absolute_url(raw_url).is_none()
        {
            issues.push(PolicyIssue {
                severity: PolicyIssueSeverity::Warning,
                reference_id: Some(item.id.clone()),
                code: "policy.url.invalid".to_string(),
                message: "Canonical CSL URL is not a valid absolute URL and cannot support deterministic integrity checks."
                    .to_string(),
            });
        }
    }

    issues
}

fn candidate_recency_findings(
    reference_id: &str,
    candidate: &ProviderCandidate,
    policy: &SourcerightPolicy,
) -> Vec<PolicyIssue> {
    let signal_text = collect_recency_signal_text(&candidate.data);
    let mut findings = Vec::new();

    if signal_text.contains("retract") {
        findings.push(PolicyIssue {
            severity: PolicyIssueSeverity::Error,
            reference_id: Some(reference_id.to_string()),
            code: "policy.recency.provider.retraction".to_string(),
            message: "Provider evidence suggests the record is retracted and should be reviewed."
                .to_string(),
        });
    }

    if signal_text.contains("expression of concern")
        || signal_text.contains("expressions of concern")
    {
        findings.push(PolicyIssue {
            severity: PolicyIssueSeverity::Warning,
            reference_id: Some(reference_id.to_string()),
            code: "policy.recency.provider.expression_of_concern".to_string(),
            message:
                "Provider evidence reports an expression of concern and should be treated conservatively."
                    .to_string(),
        });
    }

    if signal_text.contains("erratum") || signal_text.contains("correction") {
        findings.push(PolicyIssue {
            severity: PolicyIssueSeverity::Info,
            reference_id: Some(reference_id.to_string()),
            code: "policy.recency.provider.correction".to_string(),
            message:
                "Provider evidence reports a correction or erratum; review the linked record for context."
                    .to_string(),
        });
    }

    if signal_text.contains("preprint") {
        findings.push(PolicyIssue {
            severity: PolicyIssueSeverity::Info,
            reference_id: Some(reference_id.to_string()),
            code: "policy.recency.provider.preprint".to_string(),
            message:
                "Provider evidence identifies a preprint record rather than a final published version."
                    .to_string(),
        });
    }

    if signal_text.contains("supersed") {
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

fn collect_recency_signal_text(value: &serde_json::Value) -> String {
    let mut values = Vec::new();
    collect_recency_signal_text_recursive(value, &mut values);
    values.join(" ").to_ascii_lowercase()
}

fn collect_recency_signal_text_recursive(value: &serde_json::Value, values: &mut Vec<String>) {
    match value {
        serde_json::Value::Object(map) => {
            for (key, nested) in map {
                if is_recency_signal_key(key)
                    && let Some(text) = nested.as_str()
                {
                    values.push(text.to_string());
                }
                collect_recency_signal_text_recursive(nested, values);
            }
        }
        serde_json::Value::Array(items) => {
            for item in items {
                collect_recency_signal_text_recursive(item, values);
            }
        }
        _ => {}
    }
}

fn is_recency_signal_key(key: &str) -> bool {
    matches!(
        key,
        "status"
            | "publication_status"
            | "publicationStatus"
            | "publication-state"
            | "update_type"
            | "updateType"
            | "relation_type"
            | "relationType"
            | "record_type"
            | "recordType"
            | "type"
            | "subtype"
            | "category"
    )
}

pub fn provider_backed_url_archive_issues(
    document: &CslDocument,
    sidecar: &VerificationSidecar,
) -> Vec<PolicyIssue> {
    let mut issues = Vec::new();
    let mut seen = std::collections::BTreeSet::<(String, String, String)>::new();

    for item in &document.items {
        let Some(verification) = sidecar.references.get(&item.id) else {
            continue;
        };

        let canonical_url = canonical_url_text(item)
            .and_then(parse_absolute_url)
            .map(|url| url.to_string());

        for candidate in &verification.provider_candidates {
            let mut has_non_doi_landing_evidence = false;
            let mut has_valid_archive_evidence = false;
            let status_evidence = collect_url_status_evidence(&candidate.data);
            let landing_evidence = collect_url_evidence(
                &candidate.data,
                &["final_url", "landing_page_url", "resolved_url", "url"],
            );
            let archive_evidence = collect_url_evidence(
                &candidate.data,
                &[
                    "archive_url",
                    "memento_url",
                    "snapshot_url",
                    "wayback_url",
                    "archived_url",
                ],
            );

            let first_landing_url = landing_evidence
                .iter()
                .find_map(|evidence| evidence.parsed.as_ref().map(|url| url.to_string()));

            for evidence in landing_evidence {
                match evidence.parsed {
                    Some(url) => {
                        let normalized = url.to_string();
                        if is_doi_landing_url(&url) {
                            push_url_issue(
                                &mut issues,
                                &mut seen,
                                &item.id,
                                "policy.url.doi.landing_page.recorded",
                                PolicyIssueSeverity::Info,
                                format!(
                                    "Provider evidence records a DOI landing-page URL of {normalized}; keep the DOI resolver target in verification metadata rather than canonical CSL."
                                ),
                                normalized,
                            );
                        } else if let Some(canonical) = canonical_url.as_deref() {
                            if canonical != normalized {
                                push_url_issue(
                                    &mut issues,
                                    &mut seen,
                                    &item.id,
                                    "policy.url.redirect",
                                    PolicyIssueSeverity::Info,
                                    format!(
                                        "Provider evidence records a final URL of {normalized}, which differs from the canonical CSL URL; keep the redirect evidence in verification metadata."
                                    ),
                                    normalized,
                                );
                            }
                        } else {
                            push_url_issue(
                                &mut issues,
                                &mut seen,
                                &item.id,
                                "policy.url.landing_page.recorded",
                                PolicyIssueSeverity::Info,
                                format!(
                                    "Provider evidence records a landing-page URL of {normalized}; keep it in verification metadata rather than canonical CSL."
                                ),
                                normalized,
                            );
                        }

                        if !is_doi_landing_url(&url) {
                            has_non_doi_landing_evidence = true;
                        }
                    }
                    None => {
                        push_url_issue(
                            &mut issues,
                            &mut seen,
                            &item.id,
                            "policy.url.invalid",
                            PolicyIssueSeverity::Warning,
                            format!(
                                "Provider evidence field `{}` is not a valid absolute URL.",
                                evidence.field
                            ),
                            evidence.raw,
                        );
                    }
                }
            }

            for evidence in archive_evidence {
                match evidence.parsed {
                    Some(url) => {
                        let normalized = url.to_string();
                        if is_doi_landing_url(&url) {
                            push_url_issue(
                                &mut issues,
                                &mut seen,
                                &item.id,
                                "policy.url.archive.invalid",
                                PolicyIssueSeverity::Warning,
                                format!(
                                    "Archive evidence field `{}` points at a DOI landing URL instead of a distinct archive snapshot.",
                                    evidence.field
                                ),
                                normalized,
                            );
                        } else if canonical_url
                            .as_deref()
                            .is_some_and(|canonical| canonical == normalized)
                            || first_landing_url
                                .as_deref()
                                .is_some_and(|landing| landing == normalized)
                        {
                            push_url_issue(
                                &mut issues,
                                &mut seen,
                                &item.id,
                                "policy.url.archive.mismatch",
                                PolicyIssueSeverity::Warning,
                                format!(
                                    "Archive evidence field `{}` points at the live URL instead of a distinct archive snapshot.",
                                    evidence.field
                                ),
                                normalized,
                            );
                        } else {
                            has_valid_archive_evidence = true;
                            push_url_issue(
                                &mut issues,
                                &mut seen,
                                &item.id,
                                "policy.url.archive.recorded",
                                PolicyIssueSeverity::Info,
                                format!(
                                    "Provider evidence records an archive URL of {normalized}; keep the archive evidence in verification metadata."
                                ),
                                normalized,
                            );
                        }
                    }
                    None => {
                        push_url_issue(
                            &mut issues,
                            &mut seen,
                            &item.id,
                            "policy.url.archive.invalid",
                            PolicyIssueSeverity::Warning,
                            format!(
                                "Provider evidence archive field `{}` is not a valid absolute URL.",
                                evidence.field
                            ),
                            evidence.raw,
                        );
                    }
                }
            }

            for evidence in status_evidence {
                push_url_issue(
                    &mut issues,
                    &mut seen,
                    &item.id,
                    evidence.state.code(),
                    evidence.state.severity(),
                    format!(
                        "Provider evidence marks a URL check as {} ({}); keep the status evidence in verification metadata.",
                        evidence.state.label(),
                        evidence.detail
                    ),
                    evidence.detail,
                );
            }

            if has_non_doi_landing_evidence && !has_valid_archive_evidence {
                push_url_issue(
                    &mut issues,
                    &mut seen,
                    &item.id,
                    "policy.url.archive.missing",
                    PolicyIssueSeverity::Warning,
                    "Provider evidence records a landing-page URL but no distinct archive URL; retain archive evidence in verification metadata when available.".to_string(),
                    candidate.provider.clone(),
                );
            }
        }
    }

    issues
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UrlCheckState {
    Broken,
    Offline,
    Unchecked,
}

impl UrlCheckState {
    fn code(self) -> &'static str {
        match self {
            Self::Broken => "policy.url.broken",
            Self::Offline => "policy.url.offline",
            Self::Unchecked => "policy.url.unchecked",
        }
    }

    fn severity(self) -> PolicyIssueSeverity {
        match self {
            Self::Broken => PolicyIssueSeverity::Error,
            Self::Offline => PolicyIssueSeverity::Warning,
            Self::Unchecked => PolicyIssueSeverity::Info,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Broken => "broken",
            Self::Offline => "offline",
            Self::Unchecked => "unchecked",
        }
    }
}

#[derive(Debug, Clone)]
struct UrlStatusEvidence {
    state: UrlCheckState,
    detail: String,
}

fn push_url_issue(
    issues: &mut Vec<PolicyIssue>,
    seen: &mut std::collections::BTreeSet<(String, String, String)>,
    reference_id: &str,
    code: &str,
    severity: PolicyIssueSeverity,
    message: String,
    detail: String,
) {
    let key = (reference_id.to_string(), code.to_string(), detail);
    if seen.insert(key) {
        issues.push(PolicyIssue {
            severity,
            reference_id: Some(reference_id.to_string()),
            code: code.to_string(),
            message,
        });
    }
}

fn collect_url_evidence(value: &serde_json::Value, keys: &[&str]) -> Vec<UrlEvidence> {
    let mut evidence = Vec::new();
    let mut seen = std::collections::BTreeSet::<(String, String)>::new();
    collect_url_evidence_recursive(value, keys, &mut seen, &mut evidence);
    evidence
}

fn collect_url_evidence_recursive(
    value: &serde_json::Value,
    keys: &[&str],
    seen: &mut std::collections::BTreeSet<(String, String)>,
    evidence: &mut Vec<UrlEvidence>,
) {
    match value {
        serde_json::Value::Object(map) => {
            for key in keys {
                if let Some(raw) = map.get(*key).and_then(serde_json::Value::as_str) {
                    let raw = raw.trim();
                    let key = (*key).to_string();
                    let detail = raw.to_string();
                    if seen.insert((key.clone(), detail.clone())) {
                        evidence.push(UrlEvidence {
                            field: key,
                            raw: detail.clone(),
                            parsed: parse_absolute_url(raw),
                        });
                    }
                }
            }

            for nested in map.values() {
                collect_url_evidence_recursive(nested, keys, seen, evidence);
            }
        }
        serde_json::Value::Array(items) => {
            for item in items {
                collect_url_evidence_recursive(item, keys, seen, evidence);
            }
        }
        _ => {}
    }
}

fn collect_url_status_evidence(value: &serde_json::Value) -> Vec<UrlStatusEvidence> {
    let mut evidence = Vec::new();
    let mut seen = std::collections::BTreeSet::<(String, String)>::new();
    collect_url_status_evidence_recursive(value, &mut seen, &mut evidence);
    evidence
}

fn collect_url_status_evidence_recursive(
    value: &serde_json::Value,
    seen: &mut std::collections::BTreeSet<(String, String)>,
    evidence: &mut Vec<UrlStatusEvidence>,
) {
    match value {
        serde_json::Value::Object(map) => {
            if let Some(status) = classify_url_status(map) {
                let key = (status.state.code().to_string(), status.detail.clone());
                if seen.insert(key) {
                    evidence.push(status);
                }
            }

            for nested in map.values() {
                collect_url_status_evidence_recursive(nested, seen, evidence);
            }
        }
        serde_json::Value::Array(items) => {
            for item in items {
                collect_url_status_evidence_recursive(item, seen, evidence);
            }
        }
        _ => {}
    }
}

fn classify_url_status(
    map: &serde_json::Map<String, serde_json::Value>,
) -> Option<UrlStatusEvidence> {
    if let Some(text) = string_field(
        map,
        &[
            "url_status",
            "check_status",
            "reachability_status",
            "status",
        ],
    ) && let Some(state) = state_from_text(&text)
    {
        return Some(UrlStatusEvidence {
            state,
            detail: text,
        });
    }

    if let Some(text) = string_field(
        map,
        &["error", "error_code", "error_type", "failure_reason"],
    ) && let Some(state) = state_from_text(&text)
    {
        return Some(UrlStatusEvidence {
            state,
            detail: text,
        });
    }

    if let Some(status) = integer_field(map, &["http_status", "http_status_code", "status_code"]) {
        let state = match status {
            404 | 410 | 451 => Some(UrlCheckState::Broken),
            408 | 429 | 500..=599 => Some(UrlCheckState::Offline),
            400..=499 => Some(UrlCheckState::Broken),
            _ => None,
        };

        if let Some(state) = state {
            return Some(UrlStatusEvidence {
                state,
                detail: status.to_string(),
            });
        }
    }

    if bool_field(map, "unchecked").is_some_and(|value| value)
        || bool_field(map, "checked").is_some_and(|value| !value)
    {
        return Some(UrlStatusEvidence {
            state: UrlCheckState::Unchecked,
            detail: "unchecked".to_string(),
        });
    }

    if bool_field(map, "reachable").is_some_and(|value| !value) {
        return Some(UrlStatusEvidence {
            state: UrlCheckState::Offline,
            detail: "reachable=false".to_string(),
        });
    }

    None
}

fn string_field(map: &serde_json::Map<String, serde_json::Value>, keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|key| {
        map.get(*key)
            .and_then(serde_json::Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_ascii_lowercase)
    })
}

fn integer_field(map: &serde_json::Map<String, serde_json::Value>, keys: &[&str]) -> Option<u16> {
    keys.iter().find_map(|key| {
        map.get(*key)
            .and_then(serde_json::Value::as_u64)
            .and_then(|value| u16::try_from(value).ok())
    })
}

fn bool_field(map: &serde_json::Map<String, serde_json::Value>, key: &str) -> Option<bool> {
    map.get(key).and_then(serde_json::Value::as_bool)
}

fn state_from_text(text: &str) -> Option<UrlCheckState> {
    let normalized = text.trim().to_ascii_lowercase();
    if normalized.is_empty() {
        return None;
    }

    if matches!(
        normalized.as_str(),
        "unchecked" | "not_checked" | "not checked" | "pending" | "pending_check" | "skipped"
    ) {
        return Some(UrlCheckState::Unchecked);
    }

    if normalized.contains("timeout")
        || normalized.contains("timed out")
        || normalized.contains("offline")
        || normalized.contains("unavailable")
        || normalized.contains("unreachable")
        || normalized.contains("network error")
        || normalized.contains("dns")
    {
        return Some(UrlCheckState::Offline);
    }

    if normalized.contains("broken")
        || normalized.contains("failed")
        || normalized.contains("error")
        || normalized.contains("gone")
        || normalized.contains("dead")
        || normalized.contains("not found")
    {
        return Some(UrlCheckState::Broken);
    }

    None
}

fn canonical_url_text(item: &CslItem) -> Option<&str> {
    item.extra
        .get("URL")
        .or_else(|| item.extra.get("url"))
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
}

fn parse_absolute_url(value: &str) -> Option<Url> {
    Url::parse(value.trim()).ok()
}

fn is_doi_landing_url(url: &Url) -> bool {
    let Some(host) = url.host_str() else {
        return false;
    };

    let host = host.to_ascii_lowercase();
    host == "doi.org" || host == "dx.doi.org" || host.ends_with(".doi.org")
}

#[derive(Debug, Clone)]
struct UrlEvidence {
    field: String,
    raw: String,
    parsed: Option<Url>,
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
    fn policy_warns_for_invalid_canonical_url_without_network_checks() {
        let document = CslDocument {
            items: vec![CslItem {
                id: "broken-url".to_string(),
                item_type: "webpage".to_string(),
                title: Some("Broken URL".to_string()),
                doi: None,
                extra: [("URL".to_string(), json!("not a url"))]
                    .into_iter()
                    .collect(),
            }],
        };

        let report = evaluate_policy(&document, &SourcerightPolicy::journal_vancouver());

        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.code == "policy.url.invalid")
        );
    }

    #[test]
    fn provider_backed_url_status_evidence_is_classified_without_network_calls() {
        let document = CslDocument {
            items: vec![
                CslItem {
                    id: "broken-url".to_string(),
                    item_type: "webpage".to_string(),
                    title: Some("Broken URL".to_string()),
                    doi: None,
                    extra: [("URL".to_string(), json!("https://publisher.example/broken"))]
                        .into_iter()
                        .collect(),
                },
                CslItem {
                    id: "offline-url".to_string(),
                    item_type: "webpage".to_string(),
                    title: Some("Offline URL".to_string()),
                    doi: None,
                    extra: [(
                        "URL".to_string(),
                        json!("https://publisher.example/offline"),
                    )]
                    .into_iter()
                    .collect(),
                },
                CslItem {
                    id: "unchecked-url".to_string(),
                    item_type: "webpage".to_string(),
                    title: Some("Unchecked URL".to_string()),
                    doi: None,
                    extra: [(
                        "URL".to_string(),
                        json!("https://publisher.example/unchecked"),
                    )]
                    .into_iter()
                    .collect(),
                },
            ],
        };
        let mut sidecar = VerificationSidecar::empty();
        sidecar.references.insert(
            "broken-url".to_string(),
            ReferenceVerification {
                provider_candidates: vec![ProviderCandidate {
                    provider: "url-checker".to_string(),
                    confidence: 1.0,
                    retrieved_at: "2026-05-10T00:00:00Z".to_string(),
                    data: json!({
                        "url": "https://publisher.example/broken",
                        "url_status": "broken",
                        "http_status": 404
                    }),
                }],
                ..ReferenceVerification::default()
            },
        );
        sidecar.references.insert(
            "offline-url".to_string(),
            ReferenceVerification {
                provider_candidates: vec![ProviderCandidate {
                    provider: "url-checker".to_string(),
                    confidence: 1.0,
                    retrieved_at: "2026-05-10T00:00:00Z".to_string(),
                    data: json!({
                        "url": "https://publisher.example/offline",
                        "status": "timeout"
                    }),
                }],
                ..ReferenceVerification::default()
            },
        );
        sidecar.references.insert(
            "unchecked-url".to_string(),
            ReferenceVerification {
                provider_candidates: vec![ProviderCandidate {
                    provider: "url-checker".to_string(),
                    confidence: 1.0,
                    retrieved_at: "2026-05-10T00:00:00Z".to_string(),
                    data: json!({
                        "url": "https://publisher.example/unchecked",
                        "url_status": "unchecked"
                    }),
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
                .any(|issue| issue.code == "policy.url.broken")
        );
        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.code == "policy.url.offline")
        );
        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.code == "policy.url.unchecked")
        );
    }

    #[test]
    fn provider_backed_url_status_evidence_accepts_status_only_payloads() {
        let document = CslDocument {
            items: vec![CslItem {
                id: "status-only".to_string(),
                item_type: "webpage".to_string(),
                title: Some("Status only".to_string()),
                doi: None,
                extra: [("URL".to_string(), json!("https://publisher.example/status"))]
                    .into_iter()
                    .collect(),
            }],
        };
        let mut sidecar = VerificationSidecar::empty();
        sidecar.references.insert(
            "status-only".to_string(),
            ReferenceVerification {
                provider_candidates: vec![ProviderCandidate {
                    provider: "url-checker".to_string(),
                    confidence: 1.0,
                    retrieved_at: "2026-05-10T00:00:00Z".to_string(),
                    data: json!({
                        "url_status": "broken"
                    }),
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
                .any(|issue| issue.code == "policy.url.broken")
        );
    }

    #[test]
    fn provider_backed_url_archive_evidence_is_classified_without_mutating_csl() {
        let document = CslDocument {
            items: vec![CslItem {
                id: "doi-landing".to_string(),
                item_type: "article-journal".to_string(),
                title: Some("Landing page".to_string()),
                doi: Some("10.1000/example".to_string()),
                extra: [("URL".to_string(), json!("https://doi.org/10.1000/example"))]
                    .into_iter()
                    .collect(),
            }],
        };
        let mut sidecar = VerificationSidecar::empty();
        sidecar.references.insert(
            "doi-landing".to_string(),
            ReferenceVerification {
                provider_candidates: vec![ProviderCandidate {
                    provider: "doi-resolver".to_string(),
                    confidence: 1.0,
                    retrieved_at: "2026-05-10T00:00:00Z".to_string(),
                    data: json!({
                        "final_url": "https://publisher.example/article",
                        "archive_url": "https://web.archive.org/web/20260510000000/https://publisher.example/article"
                    }),
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
                .any(|issue| issue.code == "policy.url.redirect")
        );
        assert!(
            report
                .issues
                .iter()
                .any(|issue| issue.code == "policy.url.archive.recorded")
        );
        assert_eq!(
            document.items[0]
                .extra
                .get("URL")
                .and_then(serde_json::Value::as_str),
            Some("https://doi.org/10.1000/example")
        );
    }

    #[test]
    fn provider_backed_url_archive_evidence_flags_missing_archive_for_non_doi_landing() {
        let document = CslDocument {
            items: vec![CslItem {
                id: "landing-only".to_string(),
                item_type: "article-journal".to_string(),
                title: Some("Landing only".to_string()),
                doi: Some("10.1000/example".to_string()),
                extra: [(
                    "URL".to_string(),
                    json!("https://publisher.example/article"),
                )]
                .into_iter()
                .collect(),
            }],
        };
        let mut sidecar = VerificationSidecar::empty();
        sidecar.references.insert(
            "landing-only".to_string(),
            ReferenceVerification {
                provider_candidates: vec![ProviderCandidate {
                    provider: "crossref".to_string(),
                    confidence: 1.0,
                    retrieved_at: "2026-05-10T00:00:00Z".to_string(),
                    data: json!({
                        "final_url": "https://publisher.example/article"
                    }),
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
                .any(|issue| issue.code == "policy.url.archive.missing")
        );
    }

    #[test]
    fn provider_backed_url_archive_evidence_classifies_doi_landing_urls_without_redirect_noise() {
        let document = CslDocument {
            items: vec![CslItem {
                id: "doi-final".to_string(),
                item_type: "article-journal".to_string(),
                title: Some("DOI landing".to_string()),
                doi: Some("10.1000/example".to_string()),
                extra: Default::default(),
            }],
        };
        let mut sidecar = VerificationSidecar::empty();
        sidecar.references.insert(
            "doi-final".to_string(),
            ReferenceVerification {
                provider_candidates: vec![ProviderCandidate {
                    provider: "doi-resolver".to_string(),
                    confidence: 1.0,
                    retrieved_at: "2026-05-10T00:00:00Z".to_string(),
                    data: json!({
                        "final_url": "https://doi.org/10.1000/example"
                    }),
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
                .any(|issue| issue.code == "policy.url.doi.landing_page.recorded")
        );
        assert!(
            !report
                .issues
                .iter()
                .any(|issue| issue.code == "policy.url.redirect")
        );
    }

    #[test]
    fn provider_backed_url_archive_evidence_rejects_doi_urls_in_archive_fields() {
        let document = CslDocument {
            items: vec![CslItem {
                id: "archive-doi".to_string(),
                item_type: "article-journal".to_string(),
                title: Some("Archive DOI".to_string()),
                doi: Some("10.1000/example".to_string()),
                extra: [(
                    "URL".to_string(),
                    json!("https://publisher.example/article"),
                )]
                .into_iter()
                .collect(),
            }],
        };
        let mut sidecar = VerificationSidecar::empty();
        sidecar.references.insert(
            "archive-doi".to_string(),
            ReferenceVerification {
                provider_candidates: vec![ProviderCandidate {
                    provider: "crossref".to_string(),
                    confidence: 1.0,
                    retrieved_at: "2026-05-10T00:00:00Z".to_string(),
                    data: json!({
                        "final_url": "https://publisher.example/article",
                        "archive_url": "https://doi.org/10.1000/example"
                    }),
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
                .any(|issue| issue.code == "policy.url.archive.invalid")
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

    #[test]
    fn provider_backed_recency_evidence_ignores_unscoped_title_words() {
        let document = CslDocument {
            items: vec![CslItem {
                id: "history-2024".to_string(),
                item_type: "article-journal".to_string(),
                title: Some("Retraction policy history".to_string()),
                doi: Some("10.1234/history".to_string()),
                extra: Default::default(),
            }],
        };
        let mut sidecar = VerificationSidecar::empty();
        sidecar.references.insert(
            "history-2024".to_string(),
            ReferenceVerification {
                provider_candidates: vec![ProviderCandidate {
                    provider: "crossref".to_string(),
                    confidence: 0.9,
                    retrieved_at: "2026-05-10T00:00:00Z".to_string(),
                    data: json!({
                        "title": "Retraction policy history",
                        "publication_year": 2024
                    }),
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
            !report
                .issues
                .iter()
                .any(|issue| issue.code == "policy.recency.provider.retraction")
        );
    }
}
