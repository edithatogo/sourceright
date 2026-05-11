use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).expect("fixture should be present")
}

#[test]
fn operations_status_page_links_the_release_and_coverage_status_artifacts() {
    let docs_page = read("docs/src/operations-status.md");
    let docs_site_page = read("docs-site/src/content/docs/guides/operations-status.md");
    let readme = read("README.md");

    assert!(docs_page.contains("release-status.md"));
    assert!(docs_page.contains("coverage-status.md"));
    assert!(docs_site_page.contains("release-status.md"));
    assert!(docs_site_page.contains("coverage-status.md"));
    assert!(readme.contains("coverage-status.md"));
}
