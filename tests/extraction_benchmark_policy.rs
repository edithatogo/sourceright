use std::fs;

use sourceright::{
    EXTRACTION_BENCHMARK_SCHEMA_VERSION, ExtractionBenchmarkFixture, ExtractionBenchmarkManifest,
    ExtractionCohort,
};

fn fixture(hash: &str, split: &str) -> ExtractionBenchmarkFixture {
    ExtractionBenchmarkFixture {
        id: "fixture".to_string(),
        input: "input.txt".to_string(),
        gold: "gold.json".to_string(),
        prediction: "prediction.json".to_string(),
        source: "self".to_string(),
        license: "MIT OR Apache-2.0".to_string(),
        access: "checked_in".to_string(),
        content_sha256: hash.to_string(),
        split: split.to_string(),
        cohort: ExtractionCohort {
            language: "en".to_string(),
            layout: "plain_text".to_string(),
            scan_status: "born_digital".to_string(),
            domain: "bibliography".to_string(),
        },
    }
}

#[test]
fn checked_in_manifest_is_valid_and_hash_verified() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("sourceright-bench/extraction/manifest.json");
    let manifest = ExtractionBenchmarkManifest::load(path).expect("load extraction manifest");
    assert_eq!(manifest.schema_version, EXTRACTION_BENCHMARK_SCHEMA_VERSION);
    assert!(!manifest.live_network);
}

#[test]
fn duplicate_content_across_splits_is_rejected() {
    let manifest = ExtractionBenchmarkManifest {
        schema_version: EXTRACTION_BENCHMARK_SCHEMA_VERSION.to_string(),
        runner_status: "test".to_string(),
        live_network: false,
        redistribution: "test".to_string(),
        fixtures: vec![
            fixture(&"a".repeat(64), "train"),
            fixture(&"a".repeat(64), "test"),
        ],
    };
    let error = manifest
        .validate()
        .expect_err("cross-split duplicate must fail");
    assert!(error.to_string().contains("crosses split boundaries"));
}

#[test]
fn fixture_text_is_not_empty() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("sourceright-bench/extraction/fixtures/reference-basic.txt");
    assert!(
        !fs::read_to_string(path)
            .expect("read fixture")
            .trim()
            .is_empty()
    );
}
