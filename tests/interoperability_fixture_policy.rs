use std::{collections::HashSet, fs, path::Path};

#[test]
fn interoperability_manifest_is_provenance_complete_and_path_grounded() {
    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string("fixtures/interoperability/manifest.json")
            .expect("interoperability manifest"),
    )
    .expect("valid interoperability manifest JSON");

    assert_eq!(
        manifest["schema_version"],
        "sourceright.interoperability-fixture.v1"
    );

    let allowed_licenses = HashSet::from(["MIT OR Apache-2.0"]);
    let fixtures = manifest["fixtures"].as_array().expect("fixture array");
    assert!(fixtures.len() >= 3);

    for fixture in fixtures {
        for field in [
            "id",
            "source_url",
            "upstream_revision",
            "license",
            "provenance",
            "reuse_status",
            "input_format",
            "input_path",
            "expected_csl_path",
        ] {
            assert!(!fixture[field].as_str().unwrap_or("").is_empty(), "{field}");
        }
        assert!(allowed_licenses.contains(fixture["license"].as_str().unwrap()));
        assert!(
            !fixture["expected_semantic_assertions"]
                .as_object()
                .unwrap()
                .is_empty()
        );
        assert!(Path::new(fixture["input_path"].as_str().unwrap()).is_file());
        assert!(Path::new(fixture["expected_csl_path"].as_str().unwrap()).is_file());
    }

    let adversarial = fixtures
        .iter()
        .find(|fixture| fixture["id"] == "bibtex-adversarial-self-authored")
        .expect("adversarial BibTeX fixture should be registered");
    assert_eq!(
        adversarial["expected_semantic_assertions"]["title"],
        "A Case-Sensitive Fixture: XML and CSL"
    );
    assert_eq!(
        adversarial["expected_semantic_assertions"]["author"][1],
        "de Morgan, Augustus"
    );
}

#[test]
fn expected_outputs_are_csl_json_and_do_not_replace_canonical_state() {
    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string("fixtures/interoperability/manifest.json")
            .expect("interoperability manifest"),
    )
    .expect("valid interoperability manifest JSON");

    for fixture in manifest["fixtures"].as_array().unwrap() {
        let expected: serde_json::Value = serde_json::from_str(
            &fs::read_to_string(fixture["expected_csl_path"].as_str().unwrap())
                .expect("expected CSL fixture"),
        )
        .expect("expected CSL JSON");
        assert!(expected.is_object());
        assert_ne!(fixture["expected_csl_path"], "references.csl.json");
        assert_ne!(fixture["expected_csl_path"], "references.verification.json");
    }
}

#[test]
fn optional_matrix_lane_is_secret_free_and_retains_failure_evidence() {
    let workflow = fs::read_to_string(".github/workflows/interoperability.yml")
        .expect("interoperability workflow");
    assert!(workflow.contains("permissions:\n  contents: read"));
    assert!(workflow.contains("npm ci --ignore-scripts"));
    assert!(workflow.contains("actions/upload-artifact@"));
    assert!(workflow.contains("if: always()"));
    assert!(workflow.contains("fixtures/interoperability/bibtex-adversarial.bib"));
    assert!(!workflow.contains("secrets."));
}
