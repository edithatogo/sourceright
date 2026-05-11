use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).expect("fixture should be present")
}

#[test]
fn tag_creation_triggers_release_publishing_workflows() {
    let release = read(".github/workflows/release.yml");
    let crate_publish = read(".github/workflows/publish-crate.yml");
    let mcp_publish = read(".github/workflows/publish-mcp-registry.yml");
    let runbook = read("docs/src/release-runbook.md");
    let publishing = read("docs/src/publishing.md");

    assert!(release.contains("push:\n    tags:"));
    assert!(crate_publish.contains("push:\n    tags:"));
    assert!(mcp_publish.contains("workflow_run:"));
    assert!(mcp_publish.contains("workflows: [\"Release\"]"));
    assert!(runbook.contains("MCP registry workflow follows the release workflow completion"));
    assert!(publishing.contains("follows the release workflow completion"));
}
