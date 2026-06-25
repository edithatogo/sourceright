use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).expect("fixture should be present")
}

#[derive(Debug, Default)]
struct LearningEntry {
    fields: BTreeMap<String, String>,
    lists: BTreeMap<String, Vec<String>>,
}

fn parse_learning_entries(markdown: &str) -> Vec<LearningEntry> {
    let mut entries = Vec::new();
    let mut current: Option<LearningEntry> = None;
    let mut current_list: Option<String> = None;

    for line in markdown.lines() {
        if line.starts_with("## ") {
            if let Some(entry) = current.take() {
                entries.push(entry);
            }
            current = Some(LearningEntry::default());
            current_list = None;
            continue;
        }

        let Some(entry) = current.as_mut() else {
            continue;
        };

        if let Some(rest) = line.strip_prefix("- `") {
            if let Some((key, value)) = rest.split_once("`: ") {
                let key = key.to_string();
                let value = value.trim().trim_matches('`').to_string();
                entry.fields.insert(key, value);
                current_list = None;
            } else if let Some(key) = rest.strip_suffix("`:") {
                let key = key.to_string();
                entry.lists.entry(key.clone()).or_default();
                current_list = Some(key);
            }
            continue;
        }

        if let Some(item) = line.strip_prefix("  - ")
            && let Some(key) = current_list.as_ref()
        {
            entry
                .lists
                .entry(key.clone())
                .or_default()
                .push(item.trim().to_string());
        }
    }

    if let Some(entry) = current.take() {
        entries.push(entry);
    }

    entries
}

fn schema_required_keys(schema: &Value) -> Vec<String> {
    schema["required"]
        .as_array()
        .expect("schema required should be an array")
        .iter()
        .map(|value| {
            value
                .as_str()
                .expect("required key should be a string")
                .to_string()
        })
        .collect()
}

fn schema_enum(schema: &Value, property: &str) -> Vec<String> {
    schema["properties"][property]["enum"]
        .as_array()
        .expect("schema enum should be an array")
        .iter()
        .map(|value| {
            value
                .as_str()
                .expect("enum value should be a string")
                .to_string()
        })
        .collect()
}

#[test]
fn learning_log_entries_follow_learning_entry_schema_contract() {
    let schema: Value = serde_json::from_str(&read("conductor/learning-entry.schema.json"))
        .expect("learning entry schema should be valid JSON");
    let log = read("conductor/learning-log.md");
    let entries = parse_learning_entries(&log);

    assert_eq!(
        schema["$schema"],
        "https://json-schema.org/draft/2020-12/schema"
    );
    assert_eq!(
        schema["$id"],
        "https://legal-nz.local/conductor/learning-entry.schema.json"
    );
    assert!(!entries.is_empty(), "learning log should contain entries");

    let required = schema_required_keys(&schema);
    let valid_scopes = schema_enum(&schema, "scope");
    let valid_severities = schema_enum(&schema, "severity");
    let valid_statuses = schema_enum(&schema, "status");

    for entry in entries {
        for key in &required {
            assert!(
                entry.fields.contains_key(key) || entry.lists.contains_key(key),
                "learning entry is missing required key {key}: {entry:?}"
            );
        }

        let entry_id = entry.fields.get("entry_id").expect("entry_id required");
        assert!(
            entry_id.starts_with("track-18-"),
            "entry_id should follow schema pattern: {entry_id}"
        );
        assert!(
            entry_id
                .chars()
                .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-'),
            "entry_id should be lowercase kebab-case: {entry_id}"
        );

        let observed_on = entry
            .fields
            .get("observed_on")
            .expect("observed_on required");
        assert_eq!(observed_on.len(), 10, "observed_on should be YYYY-MM-DD");
        assert_eq!(&observed_on[4..5], "-");
        assert_eq!(&observed_on[7..8], "-");

        let scope = entry.fields.get("scope").expect("scope required");
        assert!(valid_scopes.contains(scope), "invalid scope: {scope}");

        if let Some(severity) = entry.fields.get("severity") {
            assert!(
                valid_severities.contains(severity),
                "invalid severity: {severity}"
            );
        }

        if let Some(status) = entry.fields.get("status") {
            assert!(valid_statuses.contains(status), "invalid status: {status}");
        }

        for key in ["lessons_learned", "next_check_to_add"] {
            let items = entry
                .lists
                .get(key)
                .expect("required list should be present");
            assert!(!items.is_empty(), "{key} should not be empty");
            assert!(
                items.iter().all(|item| !item.trim().is_empty()),
                "{key} should not contain blank items"
            );
        }
    }
}

#[test]
fn learning_candidate_workflow_is_non_committing_and_artifact_only() {
    let workflow = read(".github/workflows/ci-learning-candidates.yml");
    let script = read("scripts/record_learning_candidate.py");
    let backlog = read("conductor/improvement-backlog.md");

    assert!(workflow.contains("persist-credentials: false"));
    assert!(workflow.contains("- \"CI\""));
    assert!(workflow.contains("scripts/record_learning_candidate.py"));
    assert!(workflow.contains("Upload non-committing learning candidate artifact"));
    assert!(workflow.contains("conductor/.tmp/ci-learning-candidates-"));
    assert!(!workflow.contains("git commit"));
    assert!(!workflow.contains("git push"));

    assert!(script.contains("Append non-committing learning candidates"));
    assert!(script.contains("SECTION_HEADING = \"## Active candidates\""));
    assert!(script.contains("write_snapshot"));
    assert!(backlog.contains("## Active candidates"));
}
