use serde_json::Value;
use sourceright::{CitationSyncAction, CitationSyncReport, CitationSyncSuggestionKind};

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

#[test]
fn citation_sync_schema_tracks_serialized_action_shapes() {
    let schema = citation_sync_schema();
    let definitions = schema["$defs"]
        .as_object()
        .expect("schema should define action definitions");
    let actions = [
        (
            "create_action",
            CitationSyncAction::Create {
                reference_id: "smith-2024".to_string(),
                zotero_key: None,
                suggestion: CitationSyncSuggestionKind::LowConfidence,
                explanation: "Create preview".to_string(),
            },
        ),
        (
            "update_action",
            CitationSyncAction::Update {
                reference_id: "smith-2024".to_string(),
                zotero_key: "Z1".to_string(),
                changed_fields: vec!["title".to_string()],
                suggestion: CitationSyncSuggestionKind::SafeUpdate,
                explanation: "Safe update".to_string(),
            },
        ),
        (
            "skip_action",
            CitationSyncAction::Skip {
                reference_id: "smith-2024".to_string(),
                zotero_key: "Z1".to_string(),
                suggestion: CitationSyncSuggestionKind::NoOp,
                explanation: "No change".to_string(),
            },
        ),
        (
            "conflict_action",
            CitationSyncAction::Conflict {
                reference_id: "smith-2024".to_string(),
                zotero_key: Some("Z1".to_string()),
                changed_fields: vec!["doi".to_string()],
                message: "Conflict".to_string(),
                suggestion: CitationSyncSuggestionKind::ReviewRequired,
                explanation: "Review required".to_string(),
            },
        ),
    ];

    for (definition_name, action) in actions {
        let serialized = serde_json::to_value(action).expect("action should serialize");
        let serialized = serialized
            .as_object()
            .expect("serialized action should be an object");
        let definition = definitions
            .get(definition_name)
            .unwrap_or_else(|| panic!("missing action definition {definition_name}"));
        let required = definition["required"]
            .as_array()
            .expect("action definition should list required fields");

        for key in serialized.keys() {
            assert!(
                required.iter().any(|value| value == key),
                "schema should require serialized {definition_name} field {key}"
            );
            assert!(
                definition["properties"].get(key).is_some(),
                "schema should define serialized {definition_name} field {key}"
            );
        }
    }
}
