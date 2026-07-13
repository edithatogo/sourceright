use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

#[test]
fn track_76_ai_client_extension_publication_hardening_is_completed() {
    let metadata =
        read("conductor/tracks/76-ai-client-extension-publication-hardening/metadata.json");
    assert!(
        metadata.contains("\"status\": \"completed\""),
        "track 76 should be completed"
    );

    for artifact in [
        "requirements-evidence.md",
        "package-decisions-2026-05-18.md",
        "mcp-client-smoke-2026-06-09.md",
        "submission-drafts.md",
        "review.md",
    ] {
        let path =
            format!("conductor/tracks/76-ai-client-extension-publication-hardening/{artifact}");
        let content = read(&path);
        assert!(!content.is_empty(), "{path} should not be empty");
    }

    let decisions = read(
        "conductor/tracks/76-ai-client-extension-publication-hardening/package-decisions-2026-05-18.md",
    );
    assert!(decisions.contains("No Codex app plugin"));
    assert!(decisions.contains("GitHub Copilot extension claim is made"));
    assert!(decisions.contains("Gemini CLI Extensions"));
    assert!(decisions.contains("Qwen CLI Extensions"));

    let release_status = read("docs/src/release-status.md");
    assert!(release_status.contains("mcp-client-smoke-2026-06-09.md"));
    assert!(release_status.contains("Claude Desktop client config"));
    assert!(release_status.contains("Gemini CLI extensions"));
    assert!(release_status.contains("Qwen CLI extensions"));
    assert!(release_status.contains("No Claude, Codex, or Copilot marketplace package exists"));

    let inventory = read("conductor/submission-requirements.json");
    assert!(inventory.contains("\"id\": \"claude-cowork\""));
    assert!(inventory.contains("\"hardened_local_package\": false"));
    assert!(inventory.contains("\"gates\": {"));

    let ledger = read("conductor/evidence-ledger.json");
    assert!(ledger.contains("76-ai-client-extension-publication-hardening"));
    assert!(ledger.contains("\"evidence_level\": \"hardened_local_package\""));
}
