use std::fs;
use std::path::Path;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

#[test]
fn host_client_submission_tracks_have_packages() {
    assert!(Path::new("extensions/gemini-sourceright/gemini-extension.json").exists());
    assert!(Path::new("extensions/qwen-sourceright/qwen-extension.json").exists());
    assert!(Path::new("extensions/opencode-sourceright/opencode.example.json").exists());
    assert!(Path::new("packages/codex-sourceright-mcp/codex-mcp.json").exists());
    assert!(Path::new("packages/copilot-sourceright/vscode-mcp.json").exists());
    assert!(Path::new("mcp/llms-install.md").exists());
    assert!(Path::new("scripts/build-host-packages.ps1").exists());
    assert!(Path::new("scripts/publish-host-submissions.ps1").exists());

    let vscode_pkg = read("extensions/vscode-sourceright/package.json");
    assert!(vscode_pkg.contains("sourceright.init"));
    assert!(vscode_pkg.contains("sourceright.validateCsl"));
    assert!(vscode_pkg.contains("sourceright.journalScreen"));

    for (id, marker) in [
        (
            "83-vscode-open-vsx-submission-and-acceptance",
            "83 VS Code and Open VSX submission and acceptance",
        ),
        (
            "90-cline-mcp-marketplace-submission-and-acceptance",
            "90 Cline MCP Marketplace submission and acceptance",
        ),
    ] {
        let metadata = read(&format!("conductor/tracks/{id}/metadata.json"));
        assert!(
            metadata.contains("\"status\": \"completed\""),
            "{id} should be completed"
        );
        let tracks = read("conductor/tracks.md");
        assert!(tracks.contains(marker));
    }
}
