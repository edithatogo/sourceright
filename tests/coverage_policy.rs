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

    assert!(ci.contains("--fail-under-lines 90"));
    assert!(hook.contains("CoverageMinimum 90"));
    assert!(pre_commit.contains("CoverageMinimum 90"));
    assert!(verify.contains("CoverageMinimum = 90"));
    assert!(verify.contains("cargo llvm-cov"));
    assert!(contributing.contains("90 percent floor"));
    assert!(readme.contains("Coverage stays gated above 90 percent"));
}
