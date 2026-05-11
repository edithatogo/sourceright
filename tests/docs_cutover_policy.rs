use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).expect("fixture should be present")
}

#[test]
fn docs_cutover_treats_docs_src_as_archival_source_and_not_public_fallback() {
    let docs_cutover = read("docs/src/docs-cutover.md");
    let publishing = read("docs/src/publishing.md");
    let contributing = read("docs/src/contributing.md");
    let site_cutover = read("docs-site/src/content/docs/guides/docs-cutover.md");
    let site_publishing = read("docs-site/src/content/docs/guides/publishing.md");

    assert!(docs_cutover.contains("archival Markdown source"));
    assert!(!docs_cutover.contains("fallback and archival"));
    assert!(publishing.contains("archival Markdown source"));
    assert!(!publishing.contains("mdBook remains a fallback"));
    assert!(contributing.contains("Astro docs site"));
    assert!(site_cutover.contains("archival Markdown docs"));
    assert!(!site_cutover.contains("fallback Markdown docs"));
    assert!(site_publishing.contains("archival Markdown source"));
}
