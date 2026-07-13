use sourceright::{
    DeterministicEntityRecognizer, ENTITY_MODEL_SCHEMA_VERSION, ENTITY_VOCABULARY_VERSION,
    EntityClass, EntityLinkCandidate, EntityPattern, MappingRelation,
};

fn ada_pattern() -> EntityPattern {
    EntityPattern {
        surface: "Ada Lovelace".to_string(),
        original_label: "person".to_string(),
        class: Some(EntityClass {
            scheme: "citeweft".to_string(),
            id: "person".to_string(),
            label: "Person".to_string(),
        }),
        relation: MappingRelation::Exact,
        confidence: 0.95,
        links: vec![EntityLinkCandidate {
            registry: "orcid".to_string(),
            version: "fixture-2026".to_string(),
            query: "Ada Lovelace".to_string(),
            identifier: None,
            method: "recorded_fixture_exact".to_string(),
            score: Some(1.0),
        }],
    }
}

#[test]
fn entity_fixture_preserves_span_mapping_and_link_separation() {
    let input = include_bytes!("../fixtures/entity-model/basic.txt");
    let report = DeterministicEntityRecognizer::scholarly(vec![ada_pattern()])
        .recognize(input)
        .expect("entity fixture");
    assert_eq!(report.schema_version, ENTITY_MODEL_SCHEMA_VERSION);
    assert_eq!(report.vocabulary_version, ENTITY_VOCABULARY_VERSION);
    assert_eq!(report.mentions.len(), 1);
    let mention = &report.mentions[0];
    assert_eq!(mention.text, "Ada Lovelace");
    assert_eq!(
        &input[mention.span.start..mention.span.end],
        b"Ada Lovelace"
    );
    assert_eq!(mention.links[0].query, mention.text);
    assert_eq!(mention.class.as_ref().unwrap().id, "person");
}

#[test]
fn entity_manifest_contains_ledger_boundaries() {
    let manifest: serde_json::Value =
        serde_json::from_str(include_str!("../fixtures/entity-model/manifest.json"))
            .expect("entity manifest");
    for field in [
        "license",
        "access",
        "redistribution",
        "domain",
        "language",
        "split",
    ] {
        assert!(!manifest[field].as_str().unwrap_or_default().is_empty());
    }
    assert!(manifest["model_artifact_sha256"].is_null());
}

#[test]
fn grobid_ner_audit_keeps_bridge_optional_and_data_caveats_visible() {
    let audit: serde_json::Value = serde_json::from_str(include_str!(
        "../conductor/third-party/grobid-ner-audit.json"
    ))
    .expect("GROBID-NER audit");
    assert_eq!(audit["taxonomy"]["documented_class_count"], 27);
    assert_eq!(audit["runtime"]["mandatory_dependency"], false);
    assert_eq!(
        audit["data_and_model_access"]["deprecated_or_unavailable_source"],
        true
    );
}
