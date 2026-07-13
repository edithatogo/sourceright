use std::fs;

use serde_json::Value;
use sourceright::{adapt_scholarly_document, decode_references_tei};

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("failed to read {path}: {error}"))
}

#[test]
fn manifest_backed_fixture_matches_neutral_and_adapter_contracts() {
    let manifest: Value = serde_json::from_str(&read("fixtures/grobid/manifest.json")).unwrap();
    assert_eq!(manifest["schema_version"], "sourceright.grobid-fixtures.v1");
    assert_eq!(manifest["license"], "self-authored");

    let neutral = decode_references_tei(
        &read("fixtures/grobid/reference-basic.tei.xml"),
        Some("fixture".to_string()),
    )
    .unwrap();
    let expected: Value =
        serde_json::from_str(&read("fixtures/grobid/reference-basic.neutral.json")).unwrap();
    assert_eq!(serde_json::to_value(&neutral).unwrap(), expected);

    let adapted = adapt_scholarly_document(&neutral, "reference-basic.tei.xml");
    let expected_adapter: Value =
        serde_json::from_str(&read("fixtures/grobid/reference-basic.adapter.json")).unwrap();
    assert_eq!(serde_json::to_value(&adapted).unwrap(), expected_adapter);
    assert!(expected_adapter.get("references.csl.json").is_none());
}
