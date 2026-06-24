use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).expect("fixture should be present")
}

#[test]
fn release_status_artifacts_are_emitted_by_all_tag_triggered_publish_workflows() {
    let release = read(".github/workflows/release.yml");
    let publish_crate = read(".github/workflows/publish-crate.yml");
    let publish_mcp = read(".github/workflows/publish-mcp-registry.yml");
    let runbook = read("docs/src/release-runbook.md");
    let docs_page = read("docs/src/release-status.md");
    let docs_site_page = read("docs-site/src/content/docs/release-status.md");

    assert!(release.contains("release-status.md"));
    assert!(publish_crate.contains("release-status.md"));
    assert!(publish_mcp.contains("release-status.md"));
    assert!(runbook.contains("release-status.md"));
    assert!(runbook.contains("scripts/verify-release-surface-refresh.ps1"));
    assert!(docs_page.contains("single human-readable summary"));
    assert!(docs_site_page.contains("single human-readable summary"));
}
