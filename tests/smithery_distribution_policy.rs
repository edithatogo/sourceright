use serde_json::{Value, json};
use std::{fs, path::PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read_repo_file(relative_path: &str) -> String {
    fs::read_to_string(repo_root().join(relative_path))
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", relative_path))
}

fn cargo_version() -> String {
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
        if in_package && line.starts_with("version") {
            return line
                .split_once('=')
                .expect("version assignment")
                .1
                .trim()
                .trim_matches('"')
                .to_string();
        }
    }

    panic!("missing Cargo.toml package version")
}

#[test]
fn smithery_mcpb_manifest_template_matches_stdio_runtime_contract() {
    let manifest: Value =
        serde_json::from_str(&read_repo_file("smithery/mcpb/manifest.template.json"))
            .expect("smithery MCPB template must be valid JSON");

    assert_eq!(manifest["manifest_version"], json!("0.3"));
    assert_eq!(manifest["name"], json!("sourceright"));
    assert_eq!(manifest["version"], json!(cargo_version()));
    assert_eq!(manifest["server"]["type"], json!("binary"));
    assert_eq!(manifest["server"]["mcp_config"]["args"], json!(["mcp"]));
    assert_eq!(manifest["tools_generated"], json!(true));
    assert_eq!(manifest["prompts_generated"], json!(true));

    let command = manifest["server"]["mcp_config"]["command"]
        .as_str()
        .expect("command must be string");
    assert!(
        command.contains("${__dirname}/bin/sourceright"),
        "Smithery MCPB bundle should run the bundled Sourceright binary"
    );
}

#[test]
fn smithery_builder_and_docs_preserve_prepared_not_accepted_boundary() {
    let builder = read_repo_file("scripts/build-smithery-mcpb.ps1");
    let publishing = read_repo_file("docs/src/publishing.md");
    let release_status = read_repo_file("docs/src/release-status.md");
    let track_review =
        read_repo_file("conductor/tracks/57-smithery-distribution-readiness/review.md");

    assert!(builder.contains("sourceright-smithery-$version-$Platform.mcpb"));
    assert!(builder.contains("manifest.template.json"));
    assert!(builder.contains("mcp"));

    assert!(publishing.contains("smithery/mcpb/manifest.template.json"));
    assert!(publishing.contains("scripts/build-smithery-mcpb.ps1"));
    assert!(publishing.contains("not accepted-listing evidence"));

    assert!(release_status.contains("| Smithery | prepared |"));
    assert!(release_status.contains("No accepted Smithery listing recorded"));
    assert!(track_review.contains("Smithery MCPB readiness is implemented"));
}
