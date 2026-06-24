use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn parse_json(path: &str) -> serde_json::Value {
    serde_json::from_str(&read(path)).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn arxiv_submission_core_migration_mapping_matches_pinned_snapshot() {
    let snapshot = parse_json(
        "conductor/tracks/80-arxiv-submission-core-maturity-hardening/submission-core-contract-snapshot.json",
    );
    let primary = parse_json("fixtures/journal/arxiv-submission-core-submission.json");
    let variants = parse_json("fixtures/journal/arxiv-submission-core-variants.json");
    let fixture_schema = read("schemas/sourceright.arxiv-submission-fixture.schema.json");
    let journal_schema = read("schemas/sourceright.journal-screening.schema.json");
    let manifest = read("plugins/manifests/journal.arxiv-submission-core.toml");
    let journal_docs = read("docs/src/journal-integrations.md");
    let security =
        read("conductor/tracks/80-arxiv-submission-core-maturity-hardening/security-boundaries.md");

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

    let events = primary["submission_events"]
        .as_array()
        .expect("primary submission_events");
    assert!(
        !events.is_empty(),
        "primary fixture should include submission_events"
    );
    for field in snapshot["required_primary_event_fields"]
        .as_array()
        .expect("event fields")
    {
        let key = field.as_str().expect("field name");
        assert!(
            events[0].get(key).is_some(),
            "primary event missing field {key}"
        );
    }

    let event_cases = variants["submission_events"]
        .as_array()
        .expect("submission_events array")
        .iter()
        .filter_map(|case| case["case"].as_str())
        .collect::<HashSet<_>>();
    for required in snapshot["event_cases"].as_array().expect("event cases") {
        let case = required.as_str().expect("event case");
        assert!(
            event_cases.contains(case),
            "event suite missing case {case}"
        );
    }

    let mapping = variants["submission_events"]
        .as_array()
        .expect("submission_events array")
        .iter()
        .filter_map(|case| Some((case["case"].as_str()?, case["expected_status"].as_str()?)))
        .collect::<HashMap<_, _>>();

    let statuses = mapping.values().copied().collect::<HashSet<_>>();
    for expected in snapshot["expected_statuses"]
        .as_array()
        .expect("expected statuses")
    {
        let status = expected.as_str().expect("status");
        assert!(
            statuses.contains(status),
            "event suite missing expected status {status}"
        );
    }

    assert_eq!(
        mapping.get("unknown-event").copied(),
        Some(
            snapshot["unknown_event_degrades_to"]
                .as_str()
                .expect("unknown mapping")
        )
    );
    assert_eq!(
        mapping.get("malformed").copied(),
        Some(
            snapshot["malformed_event_degrades_to"]
                .as_str()
                .expect("malformed mapping")
        )
    );

    assert!(fixture_schema.contains("arxiv_submission_core"));
    assert!(journal_schema.contains("arxiv_submission_core"));
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
            manifest.contains("submission-core"),
            "manifest should reference submission-core"
        );
    }

    assert!(security.contains("No mutation of submission-core"));
    assert!(security.contains("never silently overwritten"));
    assert!(snapshot["no_writeback"].as_bool().unwrap_or(false));
    assert!(
        snapshot["no_live_credentials_default_ci"]
            .as_bool()
            .unwrap_or(false)
    );
}

#[test]
fn track_80_arxiv_submission_core_maturity_hardening_is_completed() {
    let metadata =
        read("conductor/tracks/80-arxiv-submission-core-maturity-hardening/metadata.json");
    assert!(
        metadata.contains("\"status\": \"completed\""),
        "track 80 should be completed"
    );

    for artifact in [
        "requirements-evidence.md",
        "submission-core-contract-snapshot.json",
        "migration-mapping-check-2026-06-09.md",
        "security-boundaries.md",
        "evidence-packet.md",
        "review.md",
    ] {
        let path =
            format!("conductor/tracks/80-arxiv-submission-core-maturity-hardening/{artifact}");
        let content = read(&path);
        assert!(!content.is_empty(), "{path} should not be empty");
    }

    let review = read("conductor/tracks/80-arxiv-submission-core-maturity-hardening/review.md");
    assert!(review.contains("hardened local package"));
    assert!(review.contains("No upstream submission"));

    let journal_docs = read("docs/src/journal-integrations.md");
    assert!(journal_docs.contains("migration-mapping-check-2026-06-09.md"));

    let inventory = read("conductor/submission-requirements.json");
    assert!(inventory.contains("\"id\": \"arxiv-submission-core\""));
    assert!(inventory.contains("\"current_state\": \"hardened-local-package\""));

    let ledger = read("conductor/evidence-ledger.json");
    assert!(ledger.contains("80-arxiv-submission-core-maturity-hardening"));
    assert!(ledger.contains("\"evidence_level\": \"hardened_local_package\""));
}
