use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).expect("fixture should be present")
}

#[test]
fn coverage_workflow_emits_a_named_status_artifact_on_the_supported_runner() {
    let coverage = read(".github/workflows/coverage.yml");
    let runbook = read("docs/src/coverage-reporting.md");
    let docs_page = read("docs/src/coverage-status.md");
    let docs_site_page = read("docs-site/src/content/docs/guides/coverage-status.md");

    assert!(coverage.contains("ubuntu-latest"));
    assert!(coverage.contains("coverage-status.md"));
    assert!(coverage.contains("Generate coverage status"));
    assert!(runbook.contains("ubuntu-latest"));
    assert!(runbook.contains("coverage-status.md"));
    assert!(docs_page.contains("single human-readable summary"));
    assert!(docs_site_page.contains("single human-readable summary"));
}
