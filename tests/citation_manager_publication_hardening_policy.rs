use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

#[test]
fn track_74_citation_manager_publication_hardening_is_completed() {
    let metadata = read("conductor/tracks/74-citation-manager-publication-hardening/metadata.json");
    assert!(
        metadata.contains("\"status\": \"completed\""),
        "track 74 should be completed"
    );

    for artifact in [
        "requirements-evidence.md",
        "publication-decision-2026-05-18.md",
        "zotero-adapter-hardening-2026-06-09.md",
        "endnote-reparse-verification-2026-06-09.md",
        "submission-drafts.md",
        "review.md",
    ] {
        let path = format!("conductor/tracks/74-citation-manager-publication-hardening/{artifact}");
        let content = read(&path);
        assert!(!content.is_empty(), "{path} should not be empty");
    }

    let review = read("conductor/tracks/74-citation-manager-publication-hardening/review.md");
    assert!(review.contains("hardened local package"));
    assert!(review.contains("No external Zotero forum post"));

    let decision = read(
        "conductor/tracks/74-citation-manager-publication-hardening/publication-decision-2026-05-18.md",
    );
    assert!(decision.contains("CLI/Web API adapter"));
    assert!(decision.contains("ENW/RIS"));

    let release_status = read("docs/src/release-status.md");
    assert!(release_status.contains("zotero-adapter-hardening-2026-06-09.md"));
    assert!(release_status.contains("endnote-reparse-verification-2026-06-09.md"));
    assert!(release_status.contains("| Zotero | prepared |"));
    assert!(release_status.contains("| EndNote | prepared |"));
    assert!(release_status.contains("No Zotero `.xpi`"));

    let inventory = read("conductor/submission-requirements.json");
    assert!(inventory.contains("\"id\": \"zotero\""));
    assert!(inventory.contains("\"current_state\": \"hardened-local-adapter\""));
    assert!(inventory.contains("\"current_state\": \"hardened-local-file-handoff\""));

    let ledger = read("conductor/evidence-ledger.json");
    assert!(ledger.contains("74-citation-manager-publication-hardening"));
    assert!(ledger.contains("\"evidence_level\": \"hardened_local_package\""));
}
