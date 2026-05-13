use std::fs;
use std::path::Path;

use serde_json::Value;

fn read(path: impl AsRef<Path>) -> String {
    fs::read_to_string(path).expect("fixture should be present")
}

fn read_json(path: impl AsRef<Path>) -> Value {
    serde_json::from_str(&read(path)).expect("fixture should be valid JSON")
}

fn read_yaml(path: impl AsRef<Path>) -> serde_yaml::Value {
    serde_yaml::from_str(&read(path)).expect("fixture should be valid YAML")
}

#[test]
fn case_study_manifest_points_to_synthetic_fixture_files() {
    let manifest_path = Path::new("examples/case-studies/manifest.json");
    let manifest = read_json(manifest_path);
    let readme = read("examples/case-studies/README.md");

    assert_eq!(
        manifest["schema_version"],
        "sourceright.case_study_manifest.v1"
    );
    assert!(readme.contains("synthetic"));
    assert!(readme.contains("not tied to live providers"));

    let base = manifest_path.parent().expect("case-study manifest parent");
    let cases = manifest["cases"]
        .as_array()
        .expect("case-study manifest should list cases");
    assert!(!cases.is_empty());

    for case in cases {
        let input = case["input_file"].as_str().expect("case input file");
        let notes = case["expected_notes_file"]
            .as_str()
            .expect("case expected notes file");
        assert!(base.join(input).exists(), "missing case input {input}");
        assert!(base.join(notes).exists(), "missing case notes {notes}");
    }
}

#[test]
fn citation_manager_examples_default_to_dry_run_without_stored_credentials() {
    let profiles = fs::read_dir("examples/citation-manager-profiles")
        .expect("citation-manager profile directory should be readable")
        .filter_map(|entry| {
            let path = entry.expect("profile entry").path();
            (path.extension().and_then(|value| value.to_str()) == Some("yaml")).then_some(path)
        })
        .collect::<Vec<_>>();
    assert!(!profiles.is_empty());

    for path in profiles {
        let profile = read_yaml(&path);
        assert_eq!(
            profile["schema_version"],
            "sourceright.citation_manager_profile.v1",
            "profile schema mismatch: {}",
            path.display()
        );
        assert_eq!(
            profile["default_mode"],
            "dry_run",
            "profile should default to dry-run: {}",
            path.display()
        );
        assert_eq!(
            profile["privacy"]["store_credentials"],
            false,
            "profile should not store credentials: {}",
            path.display()
        );
    }

    let manifest = read_json("examples/citation-manager-profiles/sync-manifest.dry-run.json");
    assert_eq!(manifest["schema_version"], "sourceright.sync_manifest.v1");
    assert_eq!(manifest["mode"], "dry_run");
    assert_eq!(manifest["privacy"]["store_credentials"], false);
    for action in manifest["actions"]
        .as_array()
        .expect("sync manifest should list actions")
    {
        assert_eq!(action["dry_run"], true);
    }
}

#[test]
fn mcp_example_keeps_read_only_config_and_explicit_apply_boundary() {
    let config = read_json("examples/mcp-readonly/stdio-config.json");
    let readme = read("examples/mcp-readonly/README.md");
    let transcript = read("examples/mcp-readonly/transcript.md");

    assert_eq!(
        config["mcpServers"]["sourceright"]["command"],
        "sourceright"
    );
    assert_eq!(config["mcpServers"]["sourceright"]["args"][0], "mcp");
    assert!(readme.contains("does not run a live server"));
    assert!(readme.contains("mutate files"));
    assert!(readme.contains("only set `apply: true`"));
    assert!(transcript.contains("apply_requested"));
    assert!(transcript.contains("false"));
    assert!(transcript.contains("Apply only when the mutation is intended"));
    assert!(transcript.contains("\"apply\":true"));
}
