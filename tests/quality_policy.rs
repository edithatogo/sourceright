use std::fs;

#[test]
fn rust_toolchain_and_editor_checks_are_strict() {
    let toolchain = fs::read_to_string("rust-toolchain.toml").expect("read rust-toolchain.toml");
    assert!(toolchain.contains("channel = \"1.94\""));
    assert!(toolchain.contains("\"clippy\""));
    assert!(toolchain.contains("\"rustfmt\""));

    let settings = fs::read_to_string(".vscode/settings.json").expect("read VS Code settings");
    assert!(settings.contains("\"rust-analyzer.check.command\": \"clippy\""));
    assert!(settings.contains("\"warnings\""));
}

#[test]
fn ci_runs_rust_quality_tools_beyond_fmt_clippy_and_tests() {
    let ci = fs::read_to_string(".github/workflows/ci.yml").expect("read CI workflow");
    assert!(ci.contains("cargo machete"));
    assert!(ci.contains("taplo lint Cargo.toml"));
    assert!(ci.contains("cargo clippy --all-targets -- -D warnings"));
    assert!(ci.contains("cargo fmt --all --check"));
}

#[test]
fn release_dry_run_checks_public_api_compatibility() {
    let workflow =
        fs::read_to_string(".github/workflows/release-dry-run.yml").expect("read release dry run");
    assert!(workflow.contains("cargo-semver-checks"));
    assert!(workflow.contains("continue-on-error: true"));
    assert!(workflow.contains("cargo semver-checks check-release --release-type minor"));
}

#[test]
fn crate_roots_forbid_unsafe_code() {
    for path in [
        "src/lib.rs",
        "src/main.rs",
        "src/bin/bench.rs",
        "src/bin/citation-sync.rs",
    ] {
        let source = fs::read_to_string(path).expect("read crate root");
        assert!(
            source.contains("#![forbid(unsafe_code)]"),
            "{path} must forbid unsafe code"
        );
    }
}
