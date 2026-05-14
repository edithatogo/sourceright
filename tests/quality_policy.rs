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
    assert!(ci.contains("typos --config typos.toml"));
    assert!(ci.contains("npm run typecheck"));
}

#[test]
fn release_drafter_is_configured_for_labelled_changelog_sections() {
    let workflow = fs::read_to_string(".github/workflows/release-drafter.yml")
        .expect("read release drafter workflow");
    let config =
        fs::read_to_string(".github/release-drafter.yml").expect("read release drafter config");

    assert!(workflow.contains("release-drafter/release-drafter@"));
    assert!(workflow.contains("# v7"));
    assert!(workflow.contains("tags:"));
    assert!(workflow.contains("\"v*.*.*\""));
    assert!(config.contains("Security"));
    assert!(config.contains("CI/CD And Release"));
    assert!(config.contains("Plugins And Providers"));
    assert!(config.contains("version-resolver"));
}

#[test]
fn renovate_groups_dependency_updates_by_ecosystem() {
    let renovate = fs::read_to_string("renovate.json").expect("read renovate config");
    assert!(renovate.contains("rust crate maintenance"));
    assert!(renovate.contains("github actions and mcp release automation"));
    assert!(renovate.contains("docs-site node modules"));
    assert!(renovate.contains("major dependency updates"));
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
