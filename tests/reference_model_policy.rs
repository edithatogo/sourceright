use sourceright::{
    DeterministicReferenceModel, REFERENCE_MODEL_SCHEMA_VERSION, ReferenceExtractionStatus,
};

#[test]
fn model_manifest_declares_required_provenance_without_artifact_downloads() {
    let manifest: serde_json::Value =
        serde_json::from_str(include_str!("../fixtures/reference-model/manifest.json"))
            .expect("model manifest");
    assert_eq!(
        manifest["schema_version"],
        "citeweft.reference-model-manifest.v1"
    );
    assert_eq!(manifest["runtime"], "rust-standard-library");
    assert!(manifest["label_map"].is_object());
    assert!(manifest["artifact_sha256"].is_null());
}

#[test]
fn reference_model_fixture_is_source_grounded_and_csl_independent() {
    let input = include_bytes!("../fixtures/reference-model/basic.txt");
    let report = DeterministicReferenceModel::default()
        .extract(input)
        .expect("reference model fixture");
    assert_eq!(report.schema_version, REFERENCE_MODEL_SCHEMA_VERSION);
    assert_eq!(report.references.len(), 1);
    let reference = &report.references[0];
    assert!(reference.span.start < reference.span.end);
    assert_eq!(reference.status, ReferenceExtractionStatus::Extracted);
    assert_eq!(
        reference.fields.authors.as_ref().unwrap().value,
        "Ada Lovelace"
    );
    assert_eq!(
        reference.fields.title.as_ref().unwrap().value,
        "A fixture reference"
    );
    assert_eq!(
        reference.fields.doi.as_ref().unwrap().value,
        "10.1000/fixture"
    );
    assert_eq!(report.callouts[0].reference_id.as_deref(), Some("r1"));
    let json = serde_json::to_string(&report).expect("serialize report");
    assert!(!json.contains("references.csl.json"));
}

#[test]
fn unknown_numeric_callouts_are_review_work() {
    let report = DeterministicReferenceModel::default()
        .extract(b"Claim [7].\n\nReferences:\n[1] Incomplete entry.")
        .expect("reference model fixture");
    assert_eq!(report.callouts[0].reference_id, None);
    assert_eq!(report.callouts[0].status, ReferenceExtractionStatus::Review);
}
