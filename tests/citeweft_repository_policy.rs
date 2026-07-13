use std::fs;

#[test]
fn extraction_manifest_keeps_core_and_adapters_separate() {
    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string("conductor/citeweft-extraction-manifest.json")
            .expect("extraction manifest"),
    )
    .expect("parse extraction manifest");
    assert_eq!(
        manifest["schema_version"],
        "sourceright.citeweft_extraction_manifest.v1"
    );
    assert_eq!(
        manifest["publication_claim"],
        "not_permitted_from_local_inventory"
    );
    assert!(manifest["standalone_core"].as_array().unwrap().is_empty());
    assert_eq!(
        manifest["external_core"]["repository"],
        "https://github.com/edithatogo/citeweft"
    );
    assert_eq!(manifest["external_core"]["release"], "v0.1.0-candidate.1");
    assert!(
        manifest["sourceright_adapters"]
            .as_array()
            .unwrap()
            .contains(&serde_json::json!("src/grobid.rs"))
    );
    let core = manifest["standalone_core"].as_array().unwrap();
    let adapters = manifest["sourceright_adapters"].as_array().unwrap();
    assert!(core.iter().all(|path| !adapters.contains(path)));
    assert!(!manifest["schema_contracts"].as_array().unwrap().is_empty());
    assert_eq!(
        manifest["external_gates"]["destination_approval"],
        "complete"
    );
    assert_eq!(
        manifest["external_gates"]["history_preservation"],
        "complete_for_committed_slice"
    );
    assert_eq!(
        manifest["external_gates"]["independent_ci_security"],
        "complete"
    );
    assert_eq!(
        manifest["external_gates"]["package_release"],
        "candidate_published"
    );
    assert_eq!(
        manifest["external_gates"]["downstream_compatibility"],
        "complete_for_candidate_smoke"
    );
    assert_eq!(
        manifest["external_gates"]["rollback_issue_migration"],
        "open"
    );
}
