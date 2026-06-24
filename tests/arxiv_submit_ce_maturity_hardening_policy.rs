use std::collections::HashSet;
use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn parse_json(path: &str) -> serde_json::Value {
    serde_json::from_str(&read(path)).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn arxiv_submit_ce_contract_drift_matches_pinned_snapshot() {
    let snapshot = parse_json(
        "conductor/tracks/79-arxiv-submit-ce-maturity-hardening/submit-ce-contract-snapshot.json",
    );
    let primary = parse_json("fixtures/journal/arxiv-submit-ce-submission.json");
    let variants = parse_json("fixtures/journal/arxiv-submit-ce-variants.json");
    let fixture_schema = read("schemas/sourceright.arxiv-submission-fixture.schema.json");
    let journal_schema = read("schemas/sourceright.journal-screening.schema.json");
    let manifest = read("plugins/manifests/journal.arxiv-submit-ce.toml");
    let journal_docs = read("docs/src/journal-integrations.md");
    let security =
        read("conductor/tracks/79-arxiv-submit-ce-maturity-hardening/security-boundaries.md");

    assert_eq!(
        primary["upstream_target"]["repository"],
        snapshot["upstream_repository"]
    );
    assert_eq!(
        primary["upstream_target"]["platform_generation"],
        snapshot["platform_generation"]
    );
    assert_eq!(
        primary["upstream_target"]["integration_boundary"],
        snapshot["integration_boundary"]
    );
    assert_eq!(
        primary["submission"]["platform"],
        snapshot["submission_platform_enum"]
    );
    assert_eq!(
        primary["expected_screening_report"]["schema_version"],
        snapshot["journal_screening_schema_version"]
    );

    for field in snapshot["required_submission_fields"]
        .as_array()
        .expect("submission fields")
    {
        let key = field.as_str().expect("field name");
        assert!(
            primary["submission"].get(key).is_some(),
            "primary fixture missing submission field {key}"
        );
    }

    for field in snapshot["required_source_bundle_fields"]
        .as_array()
        .expect("source bundle fields")
    {
        let key = field.as_str().expect("field name");
        assert!(
            primary["source_bundle"].get(key).is_some(),
            "primary fixture missing source_bundle field {key}"
        );
    }

    let variant_cases = variants["variants"]
        .as_array()
        .expect("variants array")
        .iter()
        .filter_map(|case| case["case"].as_str())
        .collect::<HashSet<_>>();
    for required in snapshot["variant_cases"].as_array().expect("variant cases") {
        let case = required.as_str().expect("variant case");
        assert!(
            variant_cases.contains(case),
            "variant suite missing case {case}"
        );
    }

    let statuses = variants["variants"]
        .as_array()
        .expect("variants array")
        .iter()
        .filter_map(|case| case["expected_status"].as_str())
        .collect::<HashSet<_>>();
    for expected in snapshot["expected_statuses"]
        .as_array()
        .expect("expected statuses")
    {
        let status = expected.as_str().expect("status");
        assert!(
            statuses.contains(status),
            "variant suite missing expected status {status}"
        );
    }

    assert!(fixture_schema.contains("arxiv_submit_ce"));
    assert!(journal_schema.contains("arxiv_submit_ce"));
    for label in snapshot["platform_labels"]
        .as_array()
        .expect("platform labels")
    {
        let label = label.as_str().expect("label");
        assert!(
            journal_docs.contains(label),
            "journal docs missing platform label {label}"
        );
        assert!(
            manifest.contains("submit-ce"),
            "manifest should reference submit-ce"
        );
    }

    assert!(security.contains("No paper submission"));
    assert!(security.contains("never silently overwritten"));
    assert!(snapshot["no_writeback"].as_bool().unwrap_or(false));
    assert!(
        snapshot["no_live_credentials_default_ci"]
            .as_bool()
            .unwrap_or(false)
    );
}

#[test]
fn track_79_arxiv_submit_ce_maturity_hardening_is_completed() {
    let metadata = read("conductor/tracks/79-arxiv-submit-ce-maturity-hardening/metadata.json");
    assert!(
        metadata.contains("\"status\": \"completed\""),
        "track 79 should be completed"
    );

    for artifact in [
        "requirements-evidence.md",
        "submit-ce-contract-snapshot.json",
        "schema-drift-check-2026-06-09.md",
        "security-boundaries.md",
        "evidence-packet.md",
        "review.md",
    ] {
        let path = format!("conductor/tracks/79-arxiv-submit-ce-maturity-hardening/{artifact}");
        let content = read(&path);
        assert!(!content.is_empty(), "{path} should not be empty");
    }

    let review = read("conductor/tracks/79-arxiv-submit-ce-maturity-hardening/review.md");
    assert!(review.contains("hardened local package"));
    assert!(review.contains("No upstream submission"));

    let journal_docs = read("docs/src/journal-integrations.md");
    assert!(journal_docs.contains("schema-drift-check-2026-06-09.md"));

    let inventory = read("conductor/submission-requirements.json");
    assert!(inventory.contains("\"id\": \"arxiv-submit-ce\""));
    assert!(inventory.contains("\"current_state\": \"hardened-local-package\""));

    let ledger = read("conductor/evidence-ledger.json");
    assert!(ledger.contains("79-arxiv-submit-ce-maturity-hardening"));
    assert!(ledger.contains("\"evidence_level\": \"hardened_local_package\""));
}
