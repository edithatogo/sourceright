use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).expect("fixture should be present")
}

#[test]
fn dependency_policy_uses_cargo_deny_for_advisories_bans_and_sources_plus_cargo_tree_for_duplicates()
 {
    let release = read(".github/workflows/release.yml");
    let publish_crate = read(".github/workflows/publish-crate.yml");
    let release_dry_run = read(".github/workflows/release-dry-run.yml");
    let ci = read(".github/workflows/ci.yml");
    let release_check = read("scripts/release-check.ps1");
    let publishing = read("docs/src/publishing.md");

    for workflow in [&release, &publish_crate, &release_dry_run, &ci] {
        assert!(workflow.contains("cargo deny check advisories bans sources"));
        assert!(workflow.contains("cargo tree -d --locked"));
    }

    assert!(release_check.contains("cargo deny check advisories bans sources"));
    assert!(release_check.contains("cargo tree -d --locked"));
    assert!(publishing.contains("cargo deny check advisories bans sources"));
    assert!(publishing.contains("cargo tree -d --locked"));
}
