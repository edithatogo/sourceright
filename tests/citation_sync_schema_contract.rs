use serde_json::Value;
use sourceright::{CitationSyncReport, CitationSyncSuggestionKind};

fn citation_sync_schema() -> Value {
    serde_json::from_str(include_str!(
        "../schemas/sourceright.citation-sync.schema.json"
    ))
    .expect("citation-sync schema should be valid JSON")
}

#[test]
fn citation_sync_schema_tracks_serialized_report_fields() {
    let schema = citation_sync_schema();
    assert_eq!(
        schema["properties"]["schema_version"]["const"],
        "sourceright.citation_sync.v1"
    );

    let report = CitationSyncReport {
        schema_version: "sourceright.citation_sync.v1".to_string(),
        workspace_root: ".".to_string(),
        preview: true,
        applied: false,
        create_count: 0,
        update_count: 0,
        skip_count: 0,
        conflict_count: 0,
        suppressed_count: 0,
        review_required_count: 0,
        actions: Vec::new(),
        audit_log_path: None,
    };
    let report = serde_json::to_value(report).expect("report should serialize");
    let required = schema["required"]
        .as_array()
        .expect("schema should list required top-level fields");

    for key in report
        .as_object()
        .expect("serialized report should be an object")
        .keys()
    {
        assert!(
            required.iter().any(|value| value == key),
            "schema should require serialized report field {key}"
        );
        assert!(
            schema["properties"].get(key).is_some(),
            "schema should define serialized report field {key}"
        );
    }
}

#[test]
fn citation_sync_schema_tracks_low_noise_suggestion_classes() {
    let schema = citation_sync_schema();
    let enum_values = schema["$defs"]["suggestion"]["enum"]
        .as_array()
        .expect("schema should define suggestion enum");

    let serialized_suggestions = [
        CitationSyncSuggestionKind::SafeUpdate,
        CitationSyncSuggestionKind::NoOp,
        CitationSyncSuggestionKind::LowConfidence,
        CitationSyncSuggestionKind::Suppressed,
        CitationSyncSuggestionKind::ReviewRequired,
        CitationSyncSuggestionKind::Conflict,
    ]
    .map(|kind| serde_json::to_value(kind).expect("suggestion kind should serialize"));

    for suggestion in serialized_suggestions {
        assert!(
            enum_values.contains(&suggestion),
            "schema should include suggestion class {suggestion}"
        );
    }
}
