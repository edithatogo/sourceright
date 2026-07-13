use sourceright::{
    AttemptOutcome, BackendAvailability, BackendKind, CalibrationScore, RouteDecision, RouteMode,
    RoutePolicy, RouteRequest, redact_diagnostic, route,
};

fn available(backend: BackendKind, score: Option<CalibrationScore>) -> BackendAvailability {
    BackendAvailability {
        backend,
        available: true,
        version: "fixture-v1".to_string(),
        config_fingerprint: "config-v1".to_string(),
        calibrated_score: score,
    }
}

#[test]
fn routing_fixture_selects_only_calibrated_backend() {
    let request = RouteRequest {
        mode: RouteMode::Auto,
        preferred_backend: None,
        input_bytes: 10,
        document_sha256: "fixture-document".to_string(),
        model_fingerprint: "fixture-model-v1".to_string(),
        options_fingerprint: "fixture-options-v1".to_string(),
        available_backends: vec![available(
            BackendKind::NativeFast,
            Some(CalibrationScore {
                task: "reference-extraction".to_string(),
                calibration_id: "cal-v1".to_string(),
                value: 0.95,
            }),
        )],
        policy: RoutePolicy::default(),
    };
    let trace = route(&request);
    assert_eq!(trace.decision, RouteDecision::Selected);
    assert_eq!(trace.accepted_backend, Some(BackendKind::NativeFast));
    assert_eq!(trace.attempts[0].outcome, AttemptOutcome::Selected);
}

#[test]
fn oversized_input_abstains_before_backend_selection() {
    let request = RouteRequest {
        mode: RouteMode::Auto,
        preferred_backend: None,
        input_bytes: RoutePolicy::default().max_input_bytes + 1,
        document_sha256: "doc".to_string(),
        model_fingerprint: "model".to_string(),
        options_fingerprint: "options".to_string(),
        available_backends: vec![available(BackendKind::Grobid, None)],
        policy: RoutePolicy::default(),
    };
    let trace = route(&request);
    assert_eq!(trace.decision, RouteDecision::Abstained);
    assert_eq!(trace.attempts[0].outcome, AttemptOutcome::Rejected);
}

#[test]
fn cache_key_changes_when_model_fingerprint_changes() {
    let base = RouteRequest {
        mode: RouteMode::Manual,
        preferred_backend: Some(BackendKind::Grobid),
        input_bytes: 10,
        document_sha256: "doc".to_string(),
        model_fingerprint: "model-a".to_string(),
        options_fingerprint: "options".to_string(),
        available_backends: Vec::new(),
        policy: RoutePolicy::default(),
    };
    let mut changed = base.clone();
    changed.model_fingerprint = "model-b".to_string();
    assert_ne!(route(&base).cache_key, route(&changed).cache_key);
}

#[test]
fn cache_key_changes_when_route_mode_changes() {
    let mut manual = RouteRequest {
        mode: RouteMode::Manual,
        preferred_backend: Some(BackendKind::Grobid),
        input_bytes: 10,
        document_sha256: "doc".to_string(),
        model_fingerprint: "model".to_string(),
        options_fingerprint: "options".to_string(),
        available_backends: Vec::new(),
        policy: RoutePolicy::default(),
    };
    let manual_key = route(&manual).cache_key;
    manual.mode = RouteMode::Auto;
    assert_ne!(manual_key, route(&manual).cache_key);
}

#[test]
fn manual_experimental_backend_respects_policy() {
    let request = RouteRequest {
        mode: RouteMode::Manual,
        preferred_backend: Some(BackendKind::Experimental),
        input_bytes: 10,
        document_sha256: "doc".to_string(),
        model_fingerprint: "model".to_string(),
        options_fingerprint: "options".to_string(),
        available_backends: vec![available(BackendKind::Experimental, None)],
        policy: RoutePolicy::default(),
    };
    let trace = route(&request);
    assert_eq!(trace.decision, RouteDecision::Abstained);
    assert_eq!(trace.attempts[0].reason, "experimental_disabled");
}

#[test]
fn diagnostics_redact_secret_values() {
    let redacted = redact_diagnostic("token=secret api_key=abc");
    assert!(!redacted.contains("secret"));
    assert!(!redacted.contains("abc"));
}

#[test]
fn incomparable_calibration_is_skipped_without_numeric_mixing() {
    let request = RouteRequest {
        mode: RouteMode::Auto,
        preferred_backend: None,
        input_bytes: 10,
        document_sha256: "doc".to_string(),
        model_fingerprint: "model".to_string(),
        options_fingerprint: "options".to_string(),
        available_backends: vec![available(
            BackendKind::NativeFast,
            Some(CalibrationScore {
                task: "entity-recognition".to_string(),
                calibration_id: "ner-v1".to_string(),
                value: 0.99,
            }),
        )],
        policy: RoutePolicy::default(),
    };
    let trace = route(&request);
    assert_eq!(trace.decision, RouteDecision::Abstained);
    assert_eq!(trace.attempts[0].reason, "incomparable_task_score");
}
