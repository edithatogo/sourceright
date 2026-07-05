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

    assert_eq!(
        server["$schema"],
        json!("https://static.modelcontextprotocol.io/schemas/2025-12-11/server.schema.json")
    );
    assert_eq!(server["name"], json!("io.github.edithatogo/sourceright"));
    assert_eq!(server["version"], json!(cargo_version.as_str()));
    assert_eq!(
        server["repository"]["url"],
        json!("https://github.com/edithatogo/sourceright")
    );
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
    let cargo_version = parse_package_version_from_cargo_toml();
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
    assert!(
        dockerfile.contains(&format!(
            "org.opencontainers.image.version=\"{cargo_version}\""
        )),
        "Dockerfile OCI version label must match Cargo.toml"
    );
}

#[test]
fn release_workflow_declares_oci_version_label() {
    let release_workflow = read_repo_file(".github/workflows/release.yml");

    assert!(
        release_workflow.contains("release-version"),
        "release workflow should derive a bare version from the tag",
    );
    assert!(
        release_workflow.contains(
            "org.opencontainers.image.version=${{ steps.release-version.outputs.version }}"
        ),
        "release workflow should label OCI image with version",
    );
}

#[test]
fn publish_workflow_binds_registry_submission_to_release_image() {
    let publish_workflow = read_repo_file(".github/workflows/publish-mcp-registry.yml");

    assert!(
        publish_workflow.contains("workflow_run"),
        "MCP registry publication should follow the release workflow"
    );
    assert!(
        publish_workflow.contains("workflow_run.conclusion == 'success'"),
        "MCP registry publication must be gated on successful release workflow completion"
    );
    assert!(
        publish_workflow.contains("data[\"name\"] == \"io.github.edithatogo/sourceright\""),
        "publish workflow should validate the server name before submission"
    );
    assert!(
        publish_workflow
            .contains("ghcr.io/edithatogo/sourceright-mcp:${{ steps.version.outputs.version }}"),
        "publish workflow should wait for the release-versioned GHCR image"
    );
    assert!(
        publish_workflow.contains("./mcp-publisher login github-oidc"),
        "publish workflow should use GitHub OIDC namespace verification"
    );
    assert!(
        publish_workflow.contains("./mcp-publisher publish"),
        "publish workflow should submit server.json through mcp-publisher"
    );
}

#[test]
fn release_status_records_mcp_registry_acceptance_and_ghcr_boundary() {
    let cargo_version = parse_package_version_from_cargo_toml();
    let release_status = read_repo_file("docs/src/release-status.md");
    let docs_site_release_status = read_repo_file("docs-site/src/content/docs/release-status.md");

    for content in [&release_status, &docs_site_release_status] {
        assert!(
            content.contains(&format!(
                "| Official MCP Registry | accepted | {cargo_version} |"
            )),
            "release status must record the official MCP Registry as accepted for the Cargo version"
        );
        assert!(
            content.contains(&format!(
                "ghcr.io/edithatogo/sourceright-mcp:{cargo_version}"
            )),
            "release status must bind the MCP Registry entry to the versioned GHCR image"
        );
        assert!(
            content.contains(&format!("| GHCR MCP image | prepared | {cargo_version} |")),
            "release status must keep direct GHCR visibility as prepared until verified"
        );
        assert!(
            content.contains("GHCR package visibility must be confirmed"),
            "release status must preserve the GHCR direct-visibility boundary"
        );
    }
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
    assert!(
        read_repo_file("LICENSE").contains("Dual-licensed under Apache-2.0 OR MIT"),
        "top-level LICENSE should advertise the repo-wide dual license"
    );
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
