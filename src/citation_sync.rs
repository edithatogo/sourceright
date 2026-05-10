use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::csl::{CslDocument, CslItem};
use crate::workspace::SourcerightWorkspace;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CitationSyncConfig {
    pub preview: bool,
    pub apply: bool,
    pub audit_log_path: Option<PathBuf>,
    pub remote_fixture_path: Option<PathBuf>,
    pub zotero_api_url: Option<String>,
    pub zotero_api_key: Option<String>,
    pub zotero_library_id: Option<String>,
    pub zotero_library_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CitationSyncReport {
    pub schema_version: String,
    pub workspace_root: String,
    pub preview: bool,
    pub applied: bool,
    pub create_count: usize,
    pub update_count: usize,
    pub skip_count: usize,
    pub conflict_count: usize,
    pub actions: Vec<CitationSyncAction>,
    pub audit_log_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum CitationSyncAction {
    Create {
        reference_id: String,
        zotero_key: Option<String>,
    },
    Update {
        reference_id: String,
        zotero_key: String,
        changed_fields: Vec<String>,
    },
    Skip {
        reference_id: String,
        zotero_key: String,
    },
    Conflict {
        reference_id: String,
        zotero_key: Option<String>,
        changed_fields: Vec<String>,
        message: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CitationSyncAuditEntry {
    pub timestamp_unix_seconds: u64,
    pub reference_id: String,
    pub action: String,
    pub zotero_key: Option<String>,
    pub result: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemoteCitationRecord {
    pub key: String,
    pub item_type: String,
    pub title: Option<String>,
    pub doi: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum CitationSyncError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("workspace error: {0}")]
    Workspace(#[from] crate::workspace::WorkspaceError),
    #[error("citation sync configuration error: {0}")]
    Configuration(String),
    #[error("zotero transport error: {0}")]
    Transport(String),
}

pub fn run_citation_sync(
    workspace: &SourcerightWorkspace,
    config: CitationSyncConfig,
) -> Result<CitationSyncReport, CitationSyncError> {
    let csl = load_csl(workspace)?;
    let remote_records = load_remote_records(&config)?;
    let actions = plan_sync_actions(&csl, &remote_records);

    let mut applied = false;
    let mut audit_log_path = None;
    if config.apply && !config.preview {
        applied = true;
        let audit_path = config.audit_log_path.unwrap_or_else(|| {
            workspace
                .root
                .join(".sourceright")
                .join("zotero-sync-audit.jsonl")
        });
        if let Some(parent) = audit_path.parent() {
            fs::create_dir_all(parent)?;
        }
        write_audit_log(&audit_path, &actions)?;
        audit_log_path = Some(audit_path.display().to_string());
        if let Some(path) = &config.remote_fixture_path {
            write_remote_fixture(path, &remote_records, &actions, &csl)?;
        }
    }

    let (create_count, update_count, skip_count, conflict_count) = count_actions(&actions);
    Ok(CitationSyncReport {
        schema_version: "sourceright.citation_sync.v1".to_string(),
        workspace_root: workspace.root.display().to_string(),
        preview: config.preview,
        applied,
        create_count,
        update_count,
        skip_count,
        conflict_count,
        actions,
        audit_log_path,
    })
}

fn load_csl(workspace: &SourcerightWorkspace) -> Result<CslDocument, CitationSyncError> {
    Ok(serde_json::from_str(&fs::read_to_string(
        &workspace.references_csl_json,
    )?)?)
}

fn load_remote_records(
    config: &CitationSyncConfig,
) -> Result<Vec<RemoteCitationRecord>, CitationSyncError> {
    if let Some(path) = &config.remote_fixture_path {
        let input = fs::read_to_string(path)?;
        return Ok(serde_json::from_str(&input)?);
    }

    if config.preview {
        return Ok(Vec::new());
    }

    let api_url = config
        .zotero_api_url
        .as_deref()
        .ok_or_else(|| CitationSyncError::Configuration("missing zotero_api_url".to_string()))?;
    let api_key = config
        .zotero_api_key
        .as_deref()
        .ok_or_else(|| CitationSyncError::Configuration("missing zotero_api_key".to_string()))?;
    let library_id = config
        .zotero_library_id
        .as_deref()
        .ok_or_else(|| CitationSyncError::Configuration("missing zotero_library_id".to_string()))?;
    let library_type = config.zotero_library_type.as_deref().unwrap_or("users");

    let url = format!(
        "{}/{}s/{}/items",
        api_url.trim_end_matches('/'),
        library_type.trim_end_matches('s'),
        library_id
    );

    let response = reqwest::blocking::Client::new()
        .get(url)
        .header("Zotero-API-Key", api_key)
        .send()
        .map_err(|error| CitationSyncError::Transport(error.to_string()))?
        .error_for_status()
        .map_err(|error| CitationSyncError::Transport(error.to_string()))?;
    let value: serde_json::Value = response
        .json()
        .map_err(|error| CitationSyncError::Transport(error.to_string()))?;
    Ok(parse_remote_records(&value))
}

fn parse_remote_records(value: &serde_json::Value) -> Vec<RemoteCitationRecord> {
    value
        .as_array()
        .into_iter()
        .flatten()
        .enumerate()
        .map(|(index, item)| RemoteCitationRecord {
            key: item
                .get("key")
                .and_then(|value| value.as_str())
                .map(|value| value.to_string())
                .unwrap_or_else(|| format!("remote-{index:04}")),
            item_type: item
                .get("itemType")
                .and_then(|value| value.as_str())
                .unwrap_or("document")
                .to_string(),
            title: item
                .get("title")
                .and_then(|value| value.as_str())
                .map(|value| value.to_string()),
            doi: item
                .get("DOI")
                .and_then(|value| value.as_str())
                .map(|value| value.to_string()),
        })
        .collect()
}

fn plan_sync_actions(
    csl: &CslDocument,
    remote_records: &[RemoteCitationRecord],
) -> Vec<CitationSyncAction> {
    let mut remote_by_doi = BTreeMap::<String, &RemoteCitationRecord>::new();
    let mut remote_by_title = BTreeMap::<String, &RemoteCitationRecord>::new();
    for record in remote_records {
        if let Some(key) = normalize(record.doi.as_deref()) {
            remote_by_doi.insert(key, record);
        }
        if let Some(key) = normalize(record.title.as_deref()) {
            remote_by_title.insert(key, record);
        }
    }

    let mut actions = Vec::new();
    for item in &csl.items {
        let local_doi = normalize(item.doi.as_deref());
        let local_title = normalize(item.title.as_deref());
        let remote_match = local_doi
            .as_ref()
            .and_then(|key| remote_by_doi.get(key))
            .or_else(|| {
                local_title
                    .as_ref()
                    .and_then(|key| remote_by_title.get(key))
            });

        match remote_match {
            None => actions.push(CitationSyncAction::Create {
                reference_id: item.id.clone(),
                zotero_key: None,
            }),
            Some(remote) if record_matches(item, remote) => {
                actions.push(CitationSyncAction::Skip {
                    reference_id: item.id.clone(),
                    zotero_key: remote.key.clone(),
                })
            }
            Some(remote) => {
                let changed_fields = changed_fields(item, remote);
                if local_doi == normalize(remote.doi.as_deref()) {
                    actions.push(CitationSyncAction::Update {
                        reference_id: item.id.clone(),
                        zotero_key: remote.key.clone(),
                        changed_fields,
                    });
                } else {
                    actions.push(CitationSyncAction::Conflict {
                        reference_id: item.id.clone(),
                        zotero_key: Some(remote.key.clone()),
                        changed_fields,
                        message:
                            "Local CSL and remote Zotero record disagree; resolve the conflict before applying."
                                .to_string(),
                    });
                }
            }
        }
    }

    actions
}

fn record_matches(item: &CslItem, remote: &RemoteCitationRecord) -> bool {
    normalize(item.doi.as_deref()) == normalize(remote.doi.as_deref())
        && normalize(item.title.as_deref()) == normalize(remote.title.as_deref())
        && normalize(Some(item.item_type.as_str())) == normalize(Some(remote.item_type.as_str()))
}

fn changed_fields(item: &CslItem, remote: &RemoteCitationRecord) -> Vec<String> {
    let mut fields = Vec::new();
    if normalize(item.title.as_deref()) != normalize(remote.title.as_deref()) {
        fields.push("title".to_string());
    }
    if normalize(item.doi.as_deref()) != normalize(remote.doi.as_deref()) {
        fields.push("doi".to_string());
    }
    if normalize(Some(item.item_type.as_str())) != normalize(Some(remote.item_type.as_str())) {
        fields.push("item_type".to_string());
    }
    fields
}

fn normalize(value: Option<&str>) -> Option<String> {
    value
        .map(|value| value.trim().to_ascii_lowercase())
        .filter(|value| !value.is_empty())
}

fn count_actions(actions: &[CitationSyncAction]) -> (usize, usize, usize, usize) {
    let mut create_count = 0;
    let mut update_count = 0;
    let mut skip_count = 0;
    let mut conflict_count = 0;
    for action in actions {
        match action {
            CitationSyncAction::Create { .. } => create_count += 1,
            CitationSyncAction::Update { .. } => update_count += 1,
            CitationSyncAction::Skip { .. } => skip_count += 1,
            CitationSyncAction::Conflict { .. } => conflict_count += 1,
        }
    }
    (create_count, update_count, skip_count, conflict_count)
}

fn write_audit_log(path: &Path, actions: &[CitationSyncAction]) -> Result<(), CitationSyncError> {
    let mut jsonl = String::new();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    for action in actions {
        let entry = match action {
            CitationSyncAction::Create {
                reference_id,
                zotero_key,
            } => CitationSyncAuditEntry {
                timestamp_unix_seconds: timestamp,
                reference_id: reference_id.clone(),
                action: action_kind(action).to_string(),
                zotero_key: zotero_key.clone(),
                result: "created".to_string(),
            },
            CitationSyncAction::Update {
                reference_id,
                zotero_key,
                ..
            } => CitationSyncAuditEntry {
                timestamp_unix_seconds: timestamp,
                reference_id: reference_id.clone(),
                action: action_kind(action).to_string(),
                zotero_key: Some(zotero_key.clone()),
                result: "updated".to_string(),
            },
            CitationSyncAction::Skip {
                reference_id,
                zotero_key,
            } => CitationSyncAuditEntry {
                timestamp_unix_seconds: timestamp,
                reference_id: reference_id.clone(),
                action: action_kind(action).to_string(),
                zotero_key: Some(zotero_key.clone()),
                result: "skipped".to_string(),
            },
            CitationSyncAction::Conflict {
                reference_id,
                zotero_key,
                ..
            } => CitationSyncAuditEntry {
                timestamp_unix_seconds: timestamp,
                reference_id: reference_id.clone(),
                action: action_kind(action).to_string(),
                zotero_key: zotero_key.clone(),
                result: "conflict".to_string(),
            },
        };
        jsonl.push_str(&serde_json::to_string(&entry)?);
        jsonl.push('\n');
    }
    fs::write(path, jsonl)?;
    Ok(())
}

fn action_kind(action: &CitationSyncAction) -> &'static str {
    match action {
        CitationSyncAction::Create { .. } => "create",
        CitationSyncAction::Update { .. } => "update",
        CitationSyncAction::Skip { .. } => "skip",
        CitationSyncAction::Conflict { .. } => "conflict",
    }
}

fn write_remote_fixture(
    path: &Path,
    remote_records: &[RemoteCitationRecord],
    actions: &[CitationSyncAction],
    csl: &CslDocument,
) -> Result<(), CitationSyncError> {
    let mut records = remote_records.to_vec();
    for action in actions {
        match action {
            CitationSyncAction::Create { reference_id, .. } => {
                if let Some(item) = csl.items.iter().find(|item| &item.id == reference_id) {
                    records.push(RemoteCitationRecord {
                        key: format!("zotero-{}", reference_id),
                        item_type: item.item_type.clone(),
                        title: item.title.clone(),
                        doi: item.doi.clone(),
                    });
                }
            }
            CitationSyncAction::Update {
                reference_id,
                zotero_key,
                ..
            } => {
                if let Some(item) = csl.items.iter().find(|item| &item.id == reference_id)
                    && let Some(record) =
                        records.iter_mut().find(|record| &record.key == zotero_key)
                {
                    record.item_type = item.item_type.clone();
                    record.title = item.title.clone();
                    record.doi = item.doi.clone();
                }
            }
            CitationSyncAction::Skip { .. } | CitationSyncAction::Conflict { .. } => {}
        }
    }
    fs::write(path, serde_json::to_string_pretty(&records)? + "\n")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_workspace() -> tempfile::TempDir {
        let tempdir = tempfile::tempdir().expect("create tempdir");
        let workspace = SourcerightWorkspace::for_document_or_dir(tempdir.path());
        workspace.init().expect("init workspace");
        fs::write(
            &workspace.references_csl_json,
            r#"[{"id":"smith-2024","type":"article-journal","title":"Benchmark reference","DOI":"10.1234/benchmark"}]"#,
        )
        .expect("write csl");
        tempdir
    }

    #[test]
    fn preview_plans_a_create_for_an_unmatched_record() {
        let tempdir = sample_workspace();
        let workspace = SourcerightWorkspace::for_document_or_dir(tempdir.path());
        let remote_path = tempdir.path().join("remote.json");
        fs::write(&remote_path, "[]").expect("write remote");

        let report = run_citation_sync(
            &workspace,
            CitationSyncConfig {
                preview: true,
                apply: false,
                audit_log_path: None,
                remote_fixture_path: Some(remote_path),
                zotero_api_url: None,
                zotero_api_key: None,
                zotero_library_id: None,
                zotero_library_type: None,
            },
        )
        .expect("run sync");

        assert_eq!(report.create_count, 1);
        assert_eq!(report.conflict_count, 0);
        assert!(!report.applied);
    }

    #[test]
    fn apply_writes_audit_log_and_fixture_remote_snapshot() {
        let tempdir = sample_workspace();
        let workspace = SourcerightWorkspace::for_document_or_dir(tempdir.path());
        let remote_path = tempdir.path().join("remote.json");
        fs::write(&remote_path, "[]").expect("write remote");
        let audit_log_path = tempdir.path().join("audit.jsonl");

        let report = run_citation_sync(
            &workspace,
            CitationSyncConfig {
                preview: false,
                apply: true,
                audit_log_path: Some(audit_log_path.clone()),
                remote_fixture_path: Some(remote_path.clone()),
                zotero_api_url: None,
                zotero_api_key: None,
                zotero_library_id: None,
                zotero_library_type: None,
            },
        )
        .expect("run sync");

        assert!(report.applied);
        assert!(audit_log_path.exists());
        assert!(remote_path.exists());
        assert!(
            fs::read_to_string(audit_log_path)
                .expect("audit")
                .contains("created")
        );
    }

    #[test]
    fn conflicts_are_reported_without_silent_overwrite() {
        let tempdir = sample_workspace();
        let workspace = SourcerightWorkspace::for_document_or_dir(tempdir.path());
        let remote_path = tempdir.path().join("remote.json");
        fs::write(
            &remote_path,
            r#"[{"key":"zotero-1","item_type":"article-journal","title":"Benchmark reference","doi":"10.9999/benchmark"}]"#,
        )
        .expect("write remote");

        let report = run_citation_sync(
            &workspace,
            CitationSyncConfig {
                preview: true,
                apply: false,
                audit_log_path: None,
                remote_fixture_path: Some(remote_path),
                zotero_api_url: None,
                zotero_api_key: None,
                zotero_library_id: None,
                zotero_library_type: None,
            },
        )
        .expect("run sync");

        assert_eq!(report.conflict_count, 1);
        assert!(matches!(
            report.actions[0],
            CitationSyncAction::Conflict { .. }
        ));
    }

    #[test]
    fn title_only_differences_are_promoted_to_updates() {
        let tempdir = sample_workspace();
        let workspace = SourcerightWorkspace::for_document_or_dir(tempdir.path());
        let remote_path = tempdir.path().join("remote.json");
        fs::write(
            &remote_path,
            r#"[{"key":"zotero-1","item_type":"article-journal","title":"Older title","doi":"10.1234/benchmark"}]"#,
        )
        .expect("write remote");

        let report = run_citation_sync(
            &workspace,
            CitationSyncConfig {
                preview: true,
                apply: false,
                audit_log_path: None,
                remote_fixture_path: Some(remote_path),
                zotero_api_url: None,
                zotero_api_key: None,
                zotero_library_id: None,
                zotero_library_type: None,
            },
        )
        .expect("run sync");

        assert_eq!(report.update_count, 1);
        assert!(matches!(
            report.actions[0],
            CitationSyncAction::Update { .. }
        ));
    }
}
