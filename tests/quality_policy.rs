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

    let windows_gnu_verify =
        fs::read_to_string("scripts/verify-local-windows-gnu.ps1").expect("read GNU verifier");
    assert!(windows_gnu_verify.contains("stable-x86_64-pc-windows-gnu"));
    assert!(windows_gnu_verify.contains("C:\\tmp\\sourceright-target-local"));
    assert!(windows_gnu_verify.contains("clippy --locked --target-dir $TargetDir --all-targets"));
    assert!(windows_gnu_verify.contains("test --locked --target-dir $TargetDir"));
    assert!(windows_gnu_verify.contains("check --locked --target-dir $TargetDir"));
    assert!(windows_gnu_verify.contains(
        "run --locked --target-dir $TargetDir --bin sourceright -- plugins validate --json"
    ));
    assert!(
        windows_gnu_verify
            .contains("run --locked --target-dir $TargetDir --bin sourceright -- bench")
    );
    assert!(windows_gnu_verify.contains("plugins validate --json"));
    assert!(windows_gnu_verify.contains("report --json examples/workspace"));
    assert!(!windows_gnu_verify.contains("examples/workspace/.sourceright"));

    let security_docs =
        fs::read_to_string("docs/src/security-automation.md").expect("read security automation");
    let docs_site_security =
        fs::read_to_string("docs-site/src/content/docs/guides/security-automation.md")
            .expect("read docs-site security automation");
    assert!(security_docs.contains("scripts\\verify-local-windows-gnu.ps1"));
    assert!(security_docs.contains("stable-x86_64-pc-windows-gnu"));
    assert!(docs_site_security.contains("scripts\\verify-local-windows-gnu.ps1"));
    assert!(docs_site_security.contains("stable-x86_64-pc-windows-gnu"));
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

fn count_occurrences(haystack: &str, needle: &str) -> usize {
    haystack.match_indices(needle).count()
}

#[test]
fn release_dry_run_checks_public_api_compatibility() {
    let workflow =
        fs::read_to_string(".github/workflows/release-dry-run.yml").expect("read release dry run");
    let publish_crate =
        fs::read_to_string(".github/workflows/publish-crate.yml").expect("read publish crate");
    let security_docs =
        fs::read_to_string("docs/src/security-automation.md").expect("read security automation");
    let docs_site_security =
        fs::read_to_string("docs-site/src/content/docs/guides/security-automation.md")
            .expect("read docs-site security automation");
    let devsecops = fs::read_to_string("docs/src/devsecops-automation-upgrade.md")
        .expect("read devsecops automation upgrade");
    let docs_site_devsecops =
        fs::read_to_string("docs-site/src/content/docs/guides/devsecops-automation-upgrade.md")
            .expect("read docs-site devsecops automation upgrade");

    assert!(workflow.contains("cargo-semver-checks"));
    assert_eq!(count_occurrences(&workflow, "continue-on-error: true"), 1);
    assert!(workflow.contains("cargo semver-checks check-release --release-type minor"));
    assert!(workflow.contains("Release surface evidence"));
    assert!(workflow.contains("shell: pwsh"));
    assert!(workflow.contains("./scripts/verify-release-surface-refresh.ps1"));
    assert!(workflow.contains("scripts/verify-release-surface-refresh.ps1"));
    assert!(workflow.contains("scripts/release-status.ps1"));
    assert!(workflow.contains(".github/workflows/publish-mcp-registry.yml"));
    assert!(workflow.contains("conductor/tracks/69-marketplace-submission-evidence/**"));
    assert!(workflow.contains("conductor/tracks/70-release-surface-refresh-cadence/**"));
    assert!(workflow.contains("conductor/evidence-ledger.json"));
    assert!(workflow.contains("docs-site/src/content/docs/**"));
    assert!(workflow.contains("tests/public_surface_refresh_policy.rs"));
    let semver_step = workflow
        .split("- name: Advisory public API compatibility")
        .nth(1)
        .expect("semver step should exist")
        .split("- name:")
        .next()
        .expect("semver step section should parse");
    assert!(semver_step.contains("continue-on-error: true"));
    assert!(publish_crate.contains("Release surface evidence"));
    assert!(publish_crate.contains("./scripts/verify-release-surface-refresh.ps1"));
    assert!(security_docs.contains("scripts/verify-release-surface-refresh.ps1"));
    assert!(security_docs.contains("release-surface evidence boundaries"));
    assert!(docs_site_security.contains("scripts/verify-release-surface-refresh.ps1"));
    assert!(docs_site_security.contains("release-surface evidence boundaries"));
    assert!(devsecops.contains("Release surface evidence"));
    assert!(devsecops.contains("accepted/prepared/deferred evidence"));
    assert!(docs_site_devsecops.contains("Release surface evidence"));
    assert!(docs_site_devsecops.contains("accepted/prepared/deferred evidence"));
}

#[test]
fn pull_request_template_keeps_release_surface_claim_gate_visible() {
    let template =
        fs::read_to_string(".github/pull_request_template.md").expect("read pull request template");

    assert!(template.contains("scripts/verify-release-surface-refresh.ps1"));
    assert!(template.contains("accepted/prepared/deferred states aligned"));
    assert!(template.contains("Public wording does not claim production readiness"));
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
