//! Explicit hybrid extraction routing and production policy contracts.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const ROUTING_SCHEMA_VERSION: &str = "citeweft.routing-trace.v1";
pub const DEFAULT_ROUTING_MAX_INPUT_BYTES: usize = 64 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BackendKind {
    Grobid,
    NativeFast,
    NativeAccurate,
    Ner,
    Experimental,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteMode {
    Manual,
    Auto,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalibrationScore {
    pub task: String,
    pub calibration_id: String,
    pub value: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackendAvailability {
    pub backend: BackendKind,
    pub available: bool,
    pub version: String,
    pub config_fingerprint: String,
    pub calibrated_score: Option<CalibrationScore>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutePolicy {
    pub max_input_bytes: usize,
    pub auto_task: String,
    pub minimum_calibrated_score_millis: u16,
    pub allow_experimental: bool,
}

impl Default for RoutePolicy {
    fn default() -> Self {
        Self {
            max_input_bytes: DEFAULT_ROUTING_MAX_INPUT_BYTES,
            auto_task: "reference-extraction".to_string(),
            minimum_calibrated_score_millis: 800,
            allow_experimental: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteRequest {
    pub mode: RouteMode,
    pub preferred_backend: Option<BackendKind>,
    pub input_bytes: usize,
    pub document_sha256: String,
    pub model_fingerprint: String,
    pub options_fingerprint: String,
    pub available_backends: Vec<BackendAvailability>,
    pub policy: RoutePolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttemptOutcome {
    Selected,
    Unavailable,
    Skipped,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteAttempt {
    pub backend: BackendKind,
    pub outcome: AttemptOutcome,
    pub reason: String,
    pub version: Option<String>,
    pub config_fingerprint: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteDecision {
    Selected,
    Abstained,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteTrace {
    pub schema_version: String,
    pub mode: RouteMode,
    pub policy: RoutePolicy,
    pub attempts: Vec<RouteAttempt>,
    pub accepted_backend: Option<BackendKind>,
    pub decision: RouteDecision,
    pub decision_reason: String,
    pub cache_key: String,
}

pub fn route(request: &RouteRequest) -> RouteTrace {
    let cache_key = cache_key(request);
    if request.input_bytes > request.policy.max_input_bytes {
        return abstained(
            request,
            cache_key,
            "input exceeds routing resource budget",
            vec![RouteAttempt {
                backend: request.preferred_backend.unwrap_or(BackendKind::NativeFast),
                outcome: AttemptOutcome::Rejected,
                reason: "resource_limit_input_bytes".to_string(),
                version: None,
                config_fingerprint: None,
            }],
        );
    }

    match request.mode {
        RouteMode::Manual => manual_route(request, cache_key),
        RouteMode::Auto => auto_route(request, cache_key),
    }
}

fn manual_route(request: &RouteRequest, cache_key: String) -> RouteTrace {
    let Some(preferred) = request.preferred_backend else {
        return abstained(
            request,
            cache_key,
            "manual mode requires a backend",
            Vec::new(),
        );
    };
    if preferred == BackendKind::Experimental && !request.policy.allow_experimental {
        return abstained(
            request,
            cache_key,
            "experimental backend is disabled by policy",
            vec![RouteAttempt {
                backend: preferred,
                outcome: AttemptOutcome::Rejected,
                reason: "experimental_disabled".to_string(),
                version: None,
                config_fingerprint: None,
            }],
        );
    }
    let Some(availability) = request
        .available_backends
        .iter()
        .find(|candidate| candidate.backend == preferred)
    else {
        return abstained(
            request,
            cache_key,
            "requested backend is not registered",
            vec![RouteAttempt {
                backend: preferred,
                outcome: AttemptOutcome::Unavailable,
                reason: "backend_not_registered".to_string(),
                version: None,
                config_fingerprint: None,
            }],
        );
    };
    if !availability.available {
        return abstained(
            request,
            cache_key,
            "requested backend is unavailable; manual mode does not fallback",
            vec![attempt(
                availability,
                AttemptOutcome::Unavailable,
                "backend_unavailable",
            )],
        );
    }
    selected(
        request,
        cache_key,
        vec![attempt(
            availability,
            AttemptOutcome::Selected,
            "manual_selection",
        )],
        preferred,
    )
}

fn auto_route(request: &RouteRequest, cache_key: String) -> RouteTrace {
    let priority = [
        BackendKind::NativeFast,
        BackendKind::NativeAccurate,
        BackendKind::Grobid,
        BackendKind::Ner,
        BackendKind::Experimental,
    ];
    let mut attempts = Vec::new();
    for backend in priority {
        let Some(availability) = request
            .available_backends
            .iter()
            .find(|candidate| candidate.backend == backend)
        else {
            continue;
        };
        if backend == BackendKind::Experimental && !request.policy.allow_experimental {
            attempts.push(attempt(
                availability,
                AttemptOutcome::Skipped,
                "experimental_disabled",
            ));
            continue;
        }
        if !availability.available {
            attempts.push(attempt(
                availability,
                AttemptOutcome::Unavailable,
                "backend_unavailable",
            ));
            continue;
        }
        if let Some(score) = &availability.calibrated_score {
            if score.task != request.policy.auto_task {
                attempts.push(attempt(
                    availability,
                    AttemptOutcome::Skipped,
                    "incomparable_task_score",
                ));
                continue;
            }
            if score.calibration_id.is_empty()
                || score.value * 1000.0 < f32::from(request.policy.minimum_calibrated_score_millis)
            {
                attempts.push(attempt(
                    availability,
                    AttemptOutcome::Skipped,
                    "below_calibrated_threshold",
                ));
                continue;
            }
            attempts.push(attempt(
                availability,
                AttemptOutcome::Selected,
                "calibrated_auto_selection",
            ));
            return selected(request, cache_key, attempts, backend);
        }
        if backend == BackendKind::Grobid {
            attempts.push(attempt(
                availability,
                AttemptOutcome::Selected,
                "explicit_fallback_no_native_score",
            ));
            return selected(request, cache_key, attempts, backend);
        }
        attempts.push(attempt(
            availability,
            AttemptOutcome::Skipped,
            "no_calibrated_score",
        ));
    }
    abstained(
        request,
        cache_key,
        "no backend met the auto policy",
        attempts,
    )
}

fn attempt(
    availability: &BackendAvailability,
    outcome: AttemptOutcome,
    reason: &str,
) -> RouteAttempt {
    RouteAttempt {
        backend: availability.backend,
        outcome,
        reason: reason.to_string(),
        version: Some(availability.version.clone()),
        config_fingerprint: Some(availability.config_fingerprint.clone()),
    }
}

fn selected(
    request: &RouteRequest,
    cache_key: String,
    attempts: Vec<RouteAttempt>,
    backend: BackendKind,
) -> RouteTrace {
    RouteTrace {
        schema_version: ROUTING_SCHEMA_VERSION.to_string(),
        mode: request.mode,
        policy: request.policy.clone(),
        attempts,
        accepted_backend: Some(backend),
        decision: RouteDecision::Selected,
        decision_reason: "backend selected; invocation remains the adapter's responsibility"
            .to_string(),
        cache_key,
    }
}

fn abstained(
    request: &RouteRequest,
    cache_key: String,
    reason: &str,
    attempts: Vec<RouteAttempt>,
) -> RouteTrace {
    RouteTrace {
        schema_version: ROUTING_SCHEMA_VERSION.to_string(),
        mode: request.mode,
        policy: request.policy.clone(),
        attempts,
        accepted_backend: None,
        decision: RouteDecision::Abstained,
        decision_reason: reason.to_string(),
        cache_key,
    }
}

pub fn cache_key(request: &RouteRequest) -> String {
    let mut digest = Sha256::new();
    digest.update(
        format!(
            "mode={:?}\0preferred={:?}\0document={}\0input_bytes={}\0model={}\0options={}\0max_input={}\0task={}\0threshold={}\0experimental={}\0",
            request.mode,
            request.preferred_backend,
            request.document_sha256,
            request.input_bytes,
            request.model_fingerprint,
            request.options_fingerprint,
            request.policy.max_input_bytes,
            request.policy.auto_task,
            request.policy.minimum_calibrated_score_millis,
            request.policy.allow_experimental,
        )
        .as_bytes(),
    );
    for backend in &request.available_backends {
        digest.update(
            format!(
                "backend={:?}\0available={}\0version={}\0config={}\0score={:?}\0",
                backend.backend,
                backend.available,
                backend.version,
                backend.config_fingerprint,
                backend.calibrated_score,
            )
            .as_bytes(),
        );
    }
    let digest = digest.finalize();
    format!(
        "sha256:{}",
        digest
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>()
    )
}

pub fn redact_diagnostic(input: &str) -> String {
    let mut output = input.to_string();
    for key in ["api_key", "token", "password", "authorization"] {
        let marker = format!("{key}=");
        while let Some(start) = output.to_ascii_lowercase().find(&marker) {
            let value_start = start + marker.len();
            if output[value_start..].starts_with("<redacted>") {
                break;
            }
            let value_end = output[value_start..]
                .find(|character: char| character.is_whitespace() || character == '&')
                .map(|offset| value_start + offset)
                .unwrap_or(output.len());
            output.replace_range(value_start..value_end, "<redacted>");
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    fn available(backend: BackendKind, score: Option<CalibrationScore>) -> BackendAvailability {
        BackendAvailability {
            backend,
            available: true,
            version: "fixture-v1".to_string(),
            config_fingerprint: "config-v1".to_string(),
            calibrated_score: score,
        }
    }

    fn request(mode: RouteMode) -> RouteRequest {
        RouteRequest {
            mode,
            preferred_backend: Some(BackendKind::NativeFast),
            input_bytes: 10,
            document_sha256: "doc".to_string(),
            model_fingerprint: "model".to_string(),
            options_fingerprint: "options".to_string(),
            available_backends: Vec::new(),
            policy: RoutePolicy::default(),
        }
    }

    #[test]
    fn manual_unavailable_does_not_fallback() {
        let mut request = request(RouteMode::Manual);
        request.available_backends.push(BackendAvailability {
            available: false,
            ..available(BackendKind::NativeFast, None)
        });
        let trace = route(&request);
        assert_eq!(trace.decision, RouteDecision::Abstained);
        assert_eq!(trace.accepted_backend, None);
        assert!(trace.decision_reason.contains("does not fallback"));
    }

    #[test]
    fn auto_selects_calibrated_backend_and_records_trace() {
        let mut request = request(RouteMode::Auto);
        request.preferred_backend = None;
        request.available_backends.push(available(
            BackendKind::NativeFast,
            Some(CalibrationScore {
                task: "reference-extraction".to_string(),
                calibration_id: "cal-v1".to_string(),
                value: 0.95,
            }),
        ));
        let trace = route(&request);
        assert_eq!(trace.accepted_backend, Some(BackendKind::NativeFast));
        assert_eq!(trace.attempts[0].outcome, AttemptOutcome::Selected);
    }

    #[test]
    fn redaction_removes_secret_values() {
        let output = redact_diagnostic("url=https://x.test?token=secret api_key=abc");
        assert!(!output.contains("secret"));
        assert!(!output.contains("abc"));
        assert!(output.contains("<redacted>"));
    }

    #[test]
    fn empty_secret_values_do_not_loop() {
        let output = redact_diagnostic("token=");
        assert!(output.contains("token=<redacted>"));
    }
}
