use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

#[test]
fn track_77_vscode_open_vsx_publication_hardening_is_completed() {
    let metadata = read("conductor/tracks/77-vscode-open-vsx-publication-hardening/metadata.json");
    assert!(
        metadata.contains("\"status\": \"completed\""),
        "track 77 should be completed"
    );

    for artifact in [
        "requirements-evidence.md",
        "vsix-build-2026-05-18.md",
        "vsix-smoke-2026-06-09.md",
        "marketplace-metadata-draft.md",
        "submission-drafts.md",
        "review.md",
    ] {
        let path = format!("conductor/tracks/77-vscode-open-vsx-publication-hardening/{artifact}");
        let content = read(&path);
        assert!(!content.is_empty(), "{path} should not be empty");
    }

    let smoke =
        read("conductor/tracks/77-vscode-open-vsx-publication-hardening/vsix-smoke-2026-06-09.md");
    assert!(smoke.contains("install_smoke"));
    assert!(smoke.contains("passed"));
    assert!(smoke.contains("does not claim an accepted"));
    assert!(smoke.contains("public listing"));

    let review = read("conductor/tracks/77-vscode-open-vsx-publication-hardening/review.md");
    assert!(review.contains("hardened local package"));
    assert!(review.contains("No external marketplace submission"));

    let release_status = read("docs/src/release-status.md");
    assert!(release_status.contains("vsix-smoke-2026-06-09.md"));
    assert!(release_status.contains("| VS Code Marketplace / Open VSX | prepared |"));
    assert!(release_status.contains("no Marketplace/Open VSX listing claimed"));

    let inventory = read("conductor/submission-requirements.json");
    assert!(inventory.contains("\"id\": \"vscode-open-vsx\""));
    assert!(inventory.contains("\"current_state\": \"hardened-local-package\""));

    let ledger = read("conductor/evidence-ledger.json");
    assert!(ledger.contains("77-vscode-open-vsx-publication-hardening"));
    assert!(ledger.contains("\"evidence_level\": \"hardened_local_package\""));
}
