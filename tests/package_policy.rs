use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).expect("fixture should be present")
}

#[test]
fn crate_package_include_list_is_anchored_to_release_surfaces() {
    let cargo_toml = read("Cargo.toml");

    for forbidden in [
        ".codex-plan",
        "docs-site",
        "node_modules",
        "Cargo.toml.orig",
    ] {
        assert!(
            !cargo_toml.contains(forbidden),
            "Cargo package policy should not include {forbidden}"
        );
    }

    for required in [
        "\"/src/**\"",
        "\"/mcp/**\"",
        "\"/plugins/**\"",
        "\"/schemas/**\"",
        "\"/examples/**\"",
        "\"/fixtures/**\"",
        "\"/provider-fixtures/**\"",
        "\"/sourceright-bench/**\"",
        "\"/docs/src/**\"",
        "\"/smithery/**\"",
        "\"/conductor/submission-requirements.json\"",
        "\"/conductor/release-channels.md\"",
        "\"/conductor/evidence-ledger.json\"",
        "\"/conductor/tracks/69-marketplace-submission-evidence/marketplace-evidence.md\"",
        "\"/scripts/build-smithery-mcpb.ps1\"",
        "\"/scripts/verify-local-windows-gnu.ps1\"",
        "\"/scripts/verify-release-surface-refresh.ps1\"",
        "\"/scripts/verify-submission-readiness.ps1\"",
        "\"/scripts/release-status.ps1\"",
        "\"/README.md\"",
    ] {
        assert!(
            cargo_toml.contains(required),
            "Cargo package include list should anchor {required}"
        );
    }

    assert!(
        !cargo_toml.contains("\"/scripts/**\""),
        "Cargo package policy should not ship repo-local validation scripts by glob"
    );
}
