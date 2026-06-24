use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).expect("fixture should be present")
}

#[test]
fn coverage_threshold_is_enforced_in_ci_and_hooks() {
    let ci = read(".github/workflows/coverage.yml");
    let hook = read(".githooks/pre-commit");
    let pre_commit = read(".pre-commit-config.yaml");
    let verify = read("scripts/verify.ps1");
    let contributing = read("CONTRIBUTING.md");
    let readme = read("README.md");

    assert!(ci.contains("--branch"));
    assert!(ci.contains("--fail-under-lines 85"));
    assert!(hook.contains("CoverageMinimum 85"));
    assert!(pre_commit.contains("CoverageMinimum 85"));
    assert!(verify.contains("CoverageMinimum = 85"));
    assert!(verify.contains("cargo llvm-cov"));
    assert!(verify.contains("--fail-under-lines $CoverageMinimum"));
    assert!(contributing.contains("85 percent floor"));
    assert!(readme.contains("Coverage stays gated above 85 percent line coverage"));
}
