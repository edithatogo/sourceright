use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

#[test]
fn track_73_mcp_directory_submission_hardening_is_completed() {
    let metadata = read("conductor/tracks/73-mcp-directory-submission-hardening/metadata.json");
    assert!(
        metadata.contains("\"status\": \"completed\""),
        "track 73 should be completed"
    );

    for artifact in [
        "requirements-evidence.md",
        "registry-metadata-validation.md",
        "smithery-mcpb-build-2026-06-09.md",
        "smithery-mcpb-publish-2026-06-10.md",
        "glama-metadata-verification.md",
        "submission-drafts.md",
        "review.md",
    ] {
        let path = format!("conductor/tracks/73-mcp-directory-submission-hardening/{artifact}");
        let content = read(&path);
        assert!(!content.is_empty(), "{path} should not be empty");
    }

    let review = read("conductor/tracks/73-mcp-directory-submission-hardening/review.md");
    assert!(review.contains("hardened local package"));
    assert!(review.contains("Glama publication"));
    assert!(review.contains("Smithery listing submitted"));

    let drafts =
        read("conductor/tracks/73-mcp-directory-submission-hardening/submission-drafts.md");
    assert!(drafts.contains("Rollback"));
    assert!(drafts.contains("Approval Gate"));

    let release_status = read("docs/src/release-status.md");
    assert!(release_status.contains("smithery-mcpb-publish-2026-06-10.md"));
    assert!(release_status.contains("glama-metadata-verification.md"));
    assert!(release_status.contains("| Smithery | submitted |"));
    assert!(release_status.contains("smithery.ai/servers/edithatogo/sourceright"));

    let live_evidence = read("conductor/submission-packets/live-evidence.json");
    assert!(live_evidence.contains("\"surface_id\": \"smithery\""));
    assert!(live_evidence.contains("smithery.ai/servers/edithatogo/sourceright"));

    let ledger = read("conductor/evidence-ledger.json");
    assert!(ledger.contains("\"evidence_level\": \"submitted\""));
    assert!(ledger.contains("73-mcp-directory-submission-hardening"));
}
