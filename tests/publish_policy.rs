use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path)
        .expect("fixture should be present")
        .replace("\r\n", "\n")
}

fn squash_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn tag_creation_triggers_release_publishing_workflows() {
    let release = read(".github/workflows/release.yml");
    let crate_publish = read(".github/workflows/publish-crate.yml");
    let mcp_publish = read(".github/workflows/publish-mcp-registry.yml");
    let runbook = squash_whitespace(&read("docs/src/release-runbook.md"));
    let publishing = squash_whitespace(&read("docs/src/publishing.md"));

    assert!(release.contains("push:\n    tags:"));
    assert!(release.contains("pattern: sourceright-*"));
    assert!(crate_publish.contains("push:\n    tags:"));
    assert!(mcp_publish.contains("workflow_run:"));
    assert!(mcp_publish.contains("workflows: [\"Release\"]"));
    assert!(runbook.contains("MCP registry workflow follows the release workflow completion"));
    assert!(publishing.contains("follows the release workflow completion"));
    assert!(release.contains("release-status.ps1"));
    assert!(crate_publish.contains("release-status.ps1"));
    assert!(mcp_publish.contains("release-status.ps1"));
}
