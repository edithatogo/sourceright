use sourceright::{
    FixtureTextLayoutExtractor, LAYOUT_SCHEMA_VERSION, LayoutDiagnosticCode, LayoutExtractor,
    LayoutLimits,
};

#[test]
fn layout_fixture_is_versioned_and_backend_neutral() {
    let input = include_bytes!("../fixtures/layout/basic.txt");
    let document = FixtureTextLayoutExtractor::default()
        .extract(input, LayoutLimits::default())
        .expect("layout fixture");
    assert_eq!(document.schema_version, LAYOUT_SCHEMA_VERSION);
    assert_eq!(document.provenance.backend, "fixture-text");
    assert!(
        document
            .pages
            .iter()
            .flat_map(|page| &page.blocks)
            .all(|block| {
                block
                    .tokens
                    .iter()
                    .all(|token| token.source_id.starts_with("page:"))
            })
    );
}

#[test]
fn layout_document_round_trips_without_csl_fields() {
    let document = FixtureTextLayoutExtractor::default()
        .extract(b"alpha beta", LayoutLimits::default())
        .expect("layout fixture");
    let encoded = serde_json::to_string(&document).expect("serialize layout");
    let decoded: sourceright::LayoutDocument =
        serde_json::from_str(&encoded).expect("deserialize layout");
    assert_eq!(decoded, document);
    assert!(!encoded.contains("references.csl.json"));
}

#[test]
fn empty_fixture_is_explicitly_ocr_required() {
    let document = FixtureTextLayoutExtractor::default()
        .extract(&[], LayoutLimits::default())
        .expect("empty fixture");
    assert!(document.pages.iter().all(|page| page.blocks.is_empty()));
    assert!(
        document
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == LayoutDiagnosticCode::OcrRequired)
    );
}
