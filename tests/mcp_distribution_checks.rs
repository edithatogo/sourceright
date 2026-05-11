use serde_json::Value;
use serde_json::json;
use std::{fs, path::PathBuf};

fn read_repo_file(relative_path: &str) -> String {
    fs::read_to_string(repo_root().join(relative_path))
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", relative_path))
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn parse_package_version_from_cargo_toml() -> String {
    let cargo_toml = read_repo_file("Cargo.toml");
    let mut in_package = false;

    for line in cargo_toml.lines() {
        let line = line.trim();

        if line == "[package]" {
            in_package = true;
            continue;
        }

        if line.starts_with('[') && line != "[package]" {
            in_package = false;
        }

        if !in_package {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            if key.trim() != "version" {
                continue;
            }

            let rhs = value.trim();
            if rhs.starts_with('"') && rhs.ends_with('"') {
                return rhs.trim_matches('"').to_string();
            }
        }
    }

    panic!("could not parse package version from Cargo.toml");
}

#[test]
fn server_json_version_matches_cargo_package_version() {
    let cargo_version = parse_package_version_from_cargo_toml();
    let server: Value = serde_json::from_str(&read_repo_file("server.json"))
        .expect("server.json must be valid JSON");

    assert_eq!(server["name"], json!("io.github.edithatogo/sourceright"));
    assert_eq!(server["version"], json!(cargo_version.as_str()));
}

#[test]
fn server_json_has_expected_oci_distribution_target() {
    let cargo_version = parse_package_version_from_cargo_toml();
    let server: Value = serde_json::from_str(&read_repo_file("server.json"))
        .expect("server.json must be valid JSON");
    let packages = server["packages"]
        .as_array()
        .expect("server.json packages must be array");
    let oci = packages
        .iter()
        .filter(|pkg| pkg["registryType"].as_str() == Some("oci"))
        .collect::<Vec<_>>();

    assert!(
        !oci.is_empty(),
        "server.json must include an OCI package entry"
    );
    assert_eq!(
        oci.len(),
        1,
        "server.json should currently expose one OCI package entry"
    );

    let oci_package = oci[0];
    assert_eq!(oci_package["transport"]["type"], json!("stdio"));
    assert_eq!(
        oci_package["identifier"],
        json!(format!(
            "ghcr.io/edithatogo/sourceright-mcp:{cargo_version}"
        ))
    );
}

#[test]
fn dockerfile_has_mcp_registry_ownership_labels() {
    let dockerfile = read_repo_file("Dockerfile");

    assert!(
        dockerfile
            .contains("io.modelcontextprotocol.server.name=\"io.github.edithatogo/sourceright\"")
    );
    assert!(
        dockerfile.contains(
            "org.opencontainers.image.source=\"https://github.com/edithatogo/sourceright\""
        )
    );
}

#[test]
fn release_workflow_declares_oci_version_label() {
    let release_workflow = read_repo_file(".github/workflows/release.yml");

    assert!(
        release_workflow.contains("org.opencontainers.image.version=${{ github.ref_name }}"),
        "release workflow should label OCI image with version",
    );
}

#[test]
fn glama_metadata_is_present_and_valid() {
    let glama: Value =
        serde_json::from_str(&read_repo_file("glama.json")).expect("glama.json must be valid JSON");

    assert_eq!(
        glama["$schema"],
        json!("https://glama.ai/mcp/schemas/server.json")
    );
    let maintainers = glama["maintainers"]
        .as_array()
        .expect("glama maintainers must be array");
    assert!(maintainers.iter().any(|value| value == "edithatogo"));
}

#[test]
fn docs_and_readme_reference_distribution_artifacts() {
    let readme = read_repo_file("README.md");
    let mcp_page = read_repo_file("docs/src/mcp.md");
    let publishing_page = read_repo_file("docs/src/publishing.md");

    assert!(readme.contains("server.json"));
    assert!(mcp_page.contains("glama.json"));
    assert!(publishing_page.contains("Smithery"));
    assert!(readme.contains("glama.json"));
}
