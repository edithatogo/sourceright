use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).expect("policy fixture should be present")
}

#[test]
fn dependency_updates_cover_runtime_and_workflow_surfaces() {
    let dependabot = read(".github/dependabot.yml");

    assert!(dependabot.contains("package-ecosystem: cargo"));
    assert!(dependabot.contains("package-ecosystem: github-actions"));
    assert!(dependabot.contains("package-ecosystem: docker"));
    assert_eq!(dependabot.matches("interval: weekly").count(), 3);
}

#[test]
fn security_workflow_keeps_independent_scan_lanes() {
    let security = read(".github/workflows/security.yml");

    for control in [
        "github/codeql-action/init@",
        "cargo install cargo-audit --locked",
        "actions/dependency-review-action@",
        "ossf/scorecard-action@",
        "github/codeql-action/upload-sarif@",
        "security-events: write",
    ] {
        assert!(
            security.contains(control),
            "missing security control: {control}"
        );
    }
}

#[test]
fn release_paths_publish_checksums_attestations_and_dependency_sbom() {
    for path in [
        ".github/workflows/release.yml",
        ".github/workflows/publish-crate.yml",
    ] {
        let workflow = read(path);
        assert!(workflow.contains("cargo metadata --locked --format-version 1"));
        assert!(workflow.contains("actions/attest-build-provenance@"));
        assert!(workflow.contains("attestations: write"));
    }

    let release = read(".github/workflows/release.yml");
    assert!(release.contains("sha256"));
    assert!(release.contains("sourceright-cargo-sbom.json"));
}
