use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

#[test]
fn track_75_journal_platform_publication_hardening_is_completed() {
    let metadata = read("conductor/tracks/75-journal-platform-publication-hardening/metadata.json");
    assert!(
        metadata.contains("\"status\": \"completed\""),
        "track 75 should be completed"
    );

    for artifact in [
        "requirements-evidence.md",
        "ojs-compatibility-matrix.md",
        "ojs-package-lint-2026-05-18.md",
        "ojs-fixture-smoke-2026-06-09.md",
        "submission-drafts.md",
        "review.md",
    ] {
        let path = format!("conductor/tracks/75-journal-platform-publication-hardening/{artifact}");
        let content = read(&path);
        assert!(!content.is_empty(), "{path} should not be empty");
    }

    let review = read("conductor/tracks/75-journal-platform-publication-hardening/review.md");
    assert!(review.contains("hardened local package"));
    assert!(review.contains("No PKP Plugin Gallery submission"));

    let release_status = read("docs/src/release-status.md");
    assert!(release_status.contains("ojs-fixture-smoke-2026-06-09.md"));
    assert!(release_status.contains("| OJS/PKP | prepared |"));
    assert!(release_status.contains("No PKP Plugin Gallery acceptance is claimed"));

    let inventory = read("conductor/submission-requirements.json");
    assert!(inventory.contains("\"id\": \"ojs-pkp\""));
    assert!(inventory.contains("\"current_state\": \"hardened-local-package\""));

    let ledger = read("conductor/evidence-ledger.json");
    assert!(ledger.contains("75-journal-platform-publication-hardening"));
    assert!(ledger.contains("\"evidence_level\": \"hardened_local_package\""));
}
