use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).expect("fixture should be present")
}

#[test]
fn release_artifacts_have_attestations_and_visible_status_badges() {
    let release = read(".github/workflows/release.yml");
    let publish_crate = read(".github/workflows/publish-crate.yml");
    let readme = read("README.md");
    let runbook = read("docs/src/release-runbook.md");

    assert!(release.contains("attestations: write"));
    assert!(release.contains("actions/attest-build-provenance@"));
    assert!(release.contains("# v3"));
    assert!(publish_crate.contains("attestations: write"));
    assert!(publish_crate.contains("actions/attest-build-provenance@"));
    assert!(publish_crate.contains("# v3"));
    assert!(readme.contains("actions/workflows/release.yml/badge.svg"));
    assert!(readme.contains("actions/workflows/publish-crate.yml/badge.svg"));
    assert!(runbook.contains("attestation"));
}
