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
        "\"/sourceright-bench/**\"",
        "\"/docs/src/**\"",
        "\"/README.md\"",
    ] {
        assert!(
            cargo_toml.contains(required),
            "Cargo package include list should anchor {required}"
        );
    }
}
