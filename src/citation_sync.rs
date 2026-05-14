use std::collections::{BTreeMap, BTreeSet};
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
    pub suppressed_count: usize,
    pub review_required_count: usize,
    pub actions: Vec<CitationSyncAction>,
    pub audit_log_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum CitationSyncAction {
    Create {
        reference_id: String,
        zotero_key: Option<String>,
        suggestion: CitationSyncSuggestionKind,
        explanation: String,
    },
    Update {
        reference_id: String,
        zotero_key: String,
        changed_fields: Vec<String>,
        suggestion: CitationSyncSuggestionKind,
        explanation: String,
    },
    Skip {
        reference_id: String,
        zotero_key: String,
        suggestion: CitationSyncSuggestionKind,
        explanation: String,
    },
    Conflict {
        reference_id: String,
        zotero_key: Option<String>,
        changed_fields: Vec<String>,
        message: String,
        suggestion: CitationSyncSuggestionKind,
        explanation: String,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CitationSyncSuggestionKind {
    SafeUpdate,
    NoOp,
    LowConfidence,
    Suppressed,
    ReviewRequired,
    Conflict,
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

    let (
        create_count,
        update_count,
        skip_count,
        conflict_count,
        suppressed_count,
        review_required_count,
    ) = count_actions(&actions);
    Ok(CitationSyncReport {
        schema_version: "sourceright.citation_sync.v1".to_string(),
        workspace_root: workspace.root.display().to_string(),
        preview: config.preview,
        applied,
        create_count,
        update_count,
        skip_count,
        conflict_count,
        suppressed_count,
        review_required_count,
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
        let value: serde_json::Value = serde_json::from_str(&input)?;
        if let Ok(records) = serde_json::from_value::<Vec<RemoteCitationRecord>>(value.clone()) {
            return Ok(records);
        }
        return Ok(parse_remote_records(&value));
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
        .map(|(index, item)| {
            let data = item.get("data").unwrap_or(item);
            RemoteCitationRecord {
                key: first_string(item, data, &["key"])
                    .map(str::to_string)
                    .unwrap_or_else(|| format!("remote-{index:04}")),
                item_type: first_string(item, data, &["itemType", "item_type"])
                    .map(zotero_item_type_to_csl)
                    .unwrap_or("document")
                    .to_string(),
                title: first_string(item, data, &["title"]).map(str::to_string),
                doi: first_string(item, data, &["DOI", "doi"]).map(str::to_string),
            }
        })
        .collect()
}

fn zotero_item_type_to_csl(item_type: &str) -> &str {
    match item_type {
        "journalArticle" => "article-journal",
        "bookSection" => "chapter",
        "conferencePaper" => "paper-conference",
        "thesis" => "thesis",
        "book" => "book",
        "document" => "document",
        other => other,
    }
}

fn first_string<'a>(
    item: &'a serde_json::Value,
    data: &'a serde_json::Value,
    keys: &[&str],
) -> Option<&'a str> {
    keys.iter().find_map(|key| {
        item.get(key)
            .or_else(|| data.get(key))
            .and_then(|value| value.as_str())
    })
}

fn plan_sync_actions(
    csl: &CslDocument,
    remote_records: &[RemoteCitationRecord],
) -> Vec<CitationSyncAction> {
    let mut remote_by_doi = BTreeMap::<String, Vec<&RemoteCitationRecord>>::new();
    let mut remote_by_title = BTreeMap::<String, Vec<&RemoteCitationRecord>>::new();
    for record in remote_records {
        if let Some(key) = normalize(record.doi.as_deref()) {
            remote_by_doi.entry(key).or_default().push(record);
        }
        if let Some(key) = normalize(record.title.as_deref()) {
            remote_by_title.entry(key).or_default().push(record);
        }
    }

    let mut actions = Vec::new();
    for item in &csl.items {
        let local_doi = normalize(item.doi.as_deref());
        let local_title = normalize(item.title.as_deref());
        let remote_matches = local_doi
            .as_ref()
            .and_then(|key| remote_by_doi.get(key))
            .or_else(|| {
                local_title
                    .as_ref()
                    .and_then(|key| remote_by_title.get(key))
            });

        match remote_matches {
            None => {
                let narrow_fit = best_narrow_fit(item, remote_records);
                let (zotero_key, suggestion, explanation) = match narrow_fit {
                    Some(fit) => (
                        Some(fit.remote.key.clone()),
                        fit.suggestion,
                        narrow_fit_explanation(item, fit.remote, fit.suggestion, fit.shared_tokens),
                    ),
                    None => (
                        None,
                        CitationSyncSuggestionKind::LowConfidence,
                        create_explanation(item),
                    ),
                };
                actions.push(CitationSyncAction::Create {
                    reference_id: item.id.clone(),
                    zotero_key,
                    suggestion,
                    explanation,
                });
            }
            Some(matches) if matches.len() > 1 => {
                actions.push(CitationSyncAction::Conflict {
                    reference_id: item.id.clone(),
                    zotero_key: None,
                    changed_fields: vec!["remote_duplicate_match".to_string()],
                    message: format!(
                        "Multiple remote Zotero records match reference `{}` by DOI or title.",
                        item.id
                    ),
                    suggestion: CitationSyncSuggestionKind::ReviewRequired,
                    explanation: format!(
                        "Sourceright found {} remote Zotero records with the same DOI or title evidence. Review the remote library before applying any writeback.",
                        matches.len()
                    ),
                });
            }
            Some(matches) => {
                let remote = matches[0];
                if record_matches(item, remote) {
                    actions.push(CitationSyncAction::Skip {
                        reference_id: item.id.clone(),
                        zotero_key: remote.key.clone(),
                        suggestion: CitationSyncSuggestionKind::NoOp,
                        explanation: format!(
                            "Remote Zotero record {} already matches the CSL DOI, title, and item type, so no writeback is needed.",
                            remote.key
                        ),
                    })
                } else {
                    let changed_fields = changed_fields(item, remote);
                    if local_doi == normalize(remote.doi.as_deref()) {
                        actions.push(CitationSyncAction::Update {
                            reference_id: item.id.clone(),
                            zotero_key: remote.key.clone(),
                            changed_fields,
                            suggestion: CitationSyncSuggestionKind::SafeUpdate,
                            explanation: update_explanation(item, remote),
                        });
                    } else {
                        actions.push(CitationSyncAction::Conflict {
                            reference_id: item.id.clone(),
                            zotero_key: Some(remote.key.clone()),
                            changed_fields,
                            message:
                                "Local CSL and remote Zotero record disagree; resolve the conflict before applying."
                                    .to_string(),
                            suggestion: CitationSyncSuggestionKind::ReviewRequired,
                            explanation: conflict_explanation(item, remote),
                        });
                    }
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

fn create_explanation(item: &CslItem) -> String {
    let mut evidence = Vec::new();
    if item
        .doi
        .as_deref()
        .is_some_and(|value| !value.trim().is_empty())
    {
        evidence.push("DOI");
    }
    if item
        .title
        .as_deref()
        .is_some_and(|value| !value.trim().is_empty())
    {
        evidence.push("title");
    }

    if evidence.is_empty() {
        "No remote Zotero record matched this CSL item, but the local item does not carry strong identifiers, so the preview keeps the create suggestion as low confidence.".to_string()
    } else {
        match evidence.as_slice() {
            [single] => format!(
                "No remote Zotero record matched on {single}, so this preview suggests a new record only as low confidence."
            ),
            [first, second] => format!(
                "No remote Zotero record matched on {first} or {second}, so this preview suggests a new record only as low confidence."
            ),
            _ => "No remote Zotero record matched the CSL item, so this preview suggests a new record only as low confidence.".to_string(),
        }
    }
}

fn update_explanation(item: &CslItem, remote: &RemoteCitationRecord) -> String {
    let fields = changed_fields(item, remote);
    match fields.as_slice() {
        [] => "The remote Zotero record already matches the CSL item, so the safe-update branch should not normally be used here.".to_string(),
        [field] => format!(
            "DOI matched, and only the {field} differs from the CSL item; this is a safe metadata update."
        ),
        _ => format!(
            "DOI matched, and the remote record differs from the CSL item in {}.",
            fields.join(", ")
        ),
    }
}

fn conflict_explanation(item: &CslItem, remote: &RemoteCitationRecord) -> String {
    let fields = changed_fields(item, remote);
    let field_text = match fields.as_slice() {
        [] => "metadata".to_string(),
        [field] => field.to_string(),
        _ => fields.join(", "),
    };
    format!(
        "The remote Zotero DOI does not match the CSL DOI, and the records also differ in {field_text}; keep this in review mode until the conflict is resolved."
    )
}

struct NarrowFit<'a> {
    remote: &'a RemoteCitationRecord,
    suggestion: CitationSyncSuggestionKind,
    shared_tokens: usize,
}

fn best_narrow_fit<'a>(
    item: &CslItem,
    remote_records: &'a [RemoteCitationRecord],
) -> Option<NarrowFit<'a>> {
    let mut best: Option<NarrowFit<'a>> = None;

    for remote in remote_records {
        let Some(candidate) = narrow_fit(item, remote) else {
            continue;
        };

        let replace = match &best {
            None => true,
            Some(current) => fit_priority(&candidate) > fit_priority(current),
        };
        if replace {
            best = Some(candidate);
        }
    }

    best
}

fn narrow_fit<'a>(item: &CslItem, remote: &'a RemoteCitationRecord) -> Option<NarrowFit<'a>> {
    if let (Some(local_doi), Some(remote_doi)) = (
        normalize(item.doi.as_deref()),
        normalize(remote.doi.as_deref()),
    ) && doi_prefix(&local_doi) == doi_prefix(&remote_doi)
        && local_doi != remote_doi
    {
        return Some(NarrowFit {
            remote,
            suggestion: CitationSyncSuggestionKind::ReviewRequired,
            shared_tokens: 0,
        });
    }

    if let (Some(local_title), Some(remote_title)) = (
        normalize(item.title.as_deref()),
        normalize(remote.title.as_deref()),
    ) {
        let local_compact = compact_text(&local_title);
        let remote_compact = compact_text(&remote_title);
        let shared_tokens = shared_title_tokens(&local_title, &remote_title);

        if local_compact == remote_compact || shared_tokens >= 2 {
            return Some(NarrowFit {
                remote,
                suggestion: CitationSyncSuggestionKind::ReviewRequired,
                shared_tokens,
            });
        }

        if shared_tokens == 1 && common_prefix_len(&local_compact, &remote_compact) >= 8 {
            return Some(NarrowFit {
                remote,
                suggestion: CitationSyncSuggestionKind::Suppressed,
                shared_tokens,
            });
        }
    }

    None
}

fn fit_priority(fit: &NarrowFit<'_>) -> (u8, usize, String) {
    let suggestion_priority = match fit.suggestion {
        CitationSyncSuggestionKind::ReviewRequired => 2,
        CitationSyncSuggestionKind::Suppressed => 1,
        _ => 0,
    };
    (
        suggestion_priority,
        fit.shared_tokens,
        fit.remote.key.clone(),
    )
}

fn narrow_fit_explanation(
    item: &CslItem,
    remote: &RemoteCitationRecord,
    suggestion: CitationSyncSuggestionKind,
    shared_tokens: usize,
) -> String {
    match suggestion {
        CitationSyncSuggestionKind::Suppressed => format!(
            "A narrow Zotero fit exists for {} on {shared_tokens} shared title token(s), but it is still too weak for an automatic create suggestion, so the preview suppresses it.",
            remote.key
        ),
        CitationSyncSuggestionKind::ReviewRequired => format!(
            "A narrow Zotero fit exists for {} and the preview keeps it review-required because the CSL item {} still disagrees on title or DOI details.",
            remote.key, item.id
        ),
        _ => create_explanation(item),
    }
}

fn shared_title_tokens(local_title: &str, remote_title: &str) -> usize {
    let local_tokens = title_tokens(local_title);
    let remote_tokens = title_tokens(remote_title);
    local_tokens.intersection(&remote_tokens).count()
}

fn title_tokens(value: &str) -> BTreeSet<String> {
    value
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|token| token.len() >= 4)
        .map(|token| token.to_ascii_lowercase())
        .collect()
}

fn compact_text(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .collect::<String>()
        .to_ascii_lowercase()
}

fn common_prefix_len(left: &str, right: &str) -> usize {
    left.chars()
        .zip(right.chars())
        .take_while(|(left, right)| left == right)
        .count()
}

fn doi_prefix(doi: &str) -> &str {
    doi.split_once('/').map(|(prefix, _)| prefix).unwrap_or(doi)
}

fn normalize(value: Option<&str>) -> Option<String> {
    value
        .map(|value| value.trim().to_ascii_lowercase())
        .filter(|value| !value.is_empty())
}

fn count_actions(actions: &[CitationSyncAction]) -> (usize, usize, usize, usize, usize, usize) {
    let mut create_count = 0;
    let mut update_count = 0;
    let mut skip_count = 0;
    let mut conflict_count = 0;
    let mut suppressed_count = 0;
    let mut review_required_count = 0;
    for action in actions {
        match action {
            CitationSyncAction::Create {
                suggestion: CitationSyncSuggestionKind::LowConfidence,
                ..
            } => create_count += 1,
            CitationSyncAction::Create {
                suggestion: CitationSyncSuggestionKind::Suppressed,
                ..
            } => suppressed_count += 1,
            CitationSyncAction::Create {
                suggestion: CitationSyncSuggestionKind::ReviewRequired,
                ..
            } => review_required_count += 1,
            CitationSyncAction::Update { .. } => update_count += 1,
            CitationSyncAction::Skip { .. } => skip_count += 1,
            CitationSyncAction::Conflict {
                suggestion: CitationSyncSuggestionKind::Conflict,
                ..
            } => conflict_count += 1,
            CitationSyncAction::Conflict {
                suggestion: CitationSyncSuggestionKind::ReviewRequired,
                ..
            } => review_required_count += 1,
            CitationSyncAction::Create {
                suggestion: CitationSyncSuggestionKind::Conflict,
                ..
            }
            | CitationSyncAction::Create {
                suggestion:
                    CitationSyncSuggestionKind::SafeUpdate | CitationSyncSuggestionKind::NoOp,
                ..
            }
            | CitationSyncAction::Conflict {
                suggestion:
                    CitationSyncSuggestionKind::SafeUpdate
                    | CitationSyncSuggestionKind::LowConfidence
                    | CitationSyncSuggestionKind::NoOp
                    | CitationSyncSuggestionKind::Suppressed,
                ..
            } => {}
        }
    }
    (
        create_count,
        update_count,
        skip_count,
        conflict_count,
        suppressed_count,
        review_required_count,
    )
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
                suggestion,
                ..
            } => CitationSyncAuditEntry {
                timestamp_unix_seconds: timestamp,
                reference_id: reference_id.clone(),
                action: action_kind(action).to_string(),
                zotero_key: zotero_key.clone(),
                result: match suggestion {
                    CitationSyncSuggestionKind::LowConfidence => "created".to_string(),
                    CitationSyncSuggestionKind::Suppressed => "suppressed".to_string(),
                    CitationSyncSuggestionKind::ReviewRequired => "review_required".to_string(),
                    CitationSyncSuggestionKind::Conflict => "conflict".to_string(),
                    CitationSyncSuggestionKind::SafeUpdate | CitationSyncSuggestionKind::NoOp => {
                        "created".to_string()
                    }
                },
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
                ..
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
                suggestion,
                ..
            } => CitationSyncAuditEntry {
                timestamp_unix_seconds: timestamp,
                reference_id: reference_id.clone(),
                action: action_kind(action).to_string(),
                zotero_key: zotero_key.clone(),
                result: match suggestion {
                    CitationSyncSuggestionKind::Conflict => "conflict".to_string(),
                    CitationSyncSuggestionKind::ReviewRequired => "review_required".to_string(),
                    CitationSyncSuggestionKind::LowConfidence
                    | CitationSyncSuggestionKind::Suppressed
                    | CitationSyncSuggestionKind::SafeUpdate
                    | CitationSyncSuggestionKind::NoOp => "conflict".to_string(),
                },
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
            CitationSyncAction::Create {
                reference_id,
                suggestion: CitationSyncSuggestionKind::LowConfidence,
                ..
            } => {
                if let Some(item) = csl.items.iter().find(|item| &item.id == reference_id) {
                    records.push(RemoteCitationRecord {
                        key: format!("zotero-{}", reference_id),
                        item_type: item.item_type.clone(),
                        title: item.title.clone(),
                        doi: item.doi.clone(),
                    });
                }
            }
            CitationSyncAction::Create { .. } => {}
            CitationSyncAction::Update {
                reference_id,
                zotero_key,
                suggestion: CitationSyncSuggestionKind::SafeUpdate,
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
            CitationSyncAction::Update { .. }
            | CitationSyncAction::Skip { .. }
            | CitationSyncAction::Conflict { .. } => {}
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

    fn workspace_with_reference(title: &str, doi: &str) -> tempfile::TempDir {
        let tempdir = tempfile::tempdir().expect("create tempdir");
        let workspace = SourcerightWorkspace::for_document_or_dir(tempdir.path());
        workspace.init().expect("init workspace");
        fs::write(
            &workspace.references_csl_json,
            format!(
                r#"[{{"id":"smith-2024","type":"article-journal","title":"{title}","DOI":"{doi}"}}]"#
            ),
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
        assert!(matches!(
            report.actions[0],
            CitationSyncAction::Create {
                suggestion: CitationSyncSuggestionKind::LowConfidence,
                ..
            }
        ));
        assert!(matches!(
            report.actions[0],
            CitationSyncAction::Create { ref explanation, .. }
                if explanation.contains("low confidence")
        ));
    }

    #[test]
    fn exact_matches_are_reported_as_no_ops() {
        let tempdir = sample_workspace();
        let workspace = SourcerightWorkspace::for_document_or_dir(tempdir.path());
        let remote_path = tempdir.path().join("remote.json");
        fs::write(
            &remote_path,
            r#"[{"key":"zotero-1","item_type":"article-journal","title":"Benchmark reference","doi":"10.1234/benchmark"}]"#,
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

        assert_eq!(report.skip_count, 1);
        assert!(matches!(
            report.actions[0],
            CitationSyncAction::Skip {
                suggestion: CitationSyncSuggestionKind::NoOp,
                ..
            }
        ));
        assert!(matches!(
            report.actions[0],
            CitationSyncAction::Skip { ref explanation, .. }
                if explanation.contains("already matches")
        ));
    }

    #[test]
    fn duplicate_remote_identity_matches_are_review_required() {
        let tempdir = sample_workspace();
        let workspace = SourcerightWorkspace::for_document_or_dir(tempdir.path());
        let remote_path = tempdir.path().join("remote.json");
        fs::write(
            &remote_path,
            r#"[{"key":"zotero-1","item_type":"article-journal","title":"Benchmark reference","doi":"10.1234/benchmark"},{"key":"zotero-2","item_type":"article-journal","title":"Benchmark reference","doi":"10.1234/benchmark"}]"#,
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

        assert_eq!(report.conflict_count, 0);
        assert_eq!(report.review_required_count, 1);
        assert!(matches!(
            report.actions[0],
            CitationSyncAction::Conflict {
                zotero_key: None,
                suggestion: CitationSyncSuggestionKind::ReviewRequired,
                ref changed_fields,
                ..
            } if changed_fields == &["remote_duplicate_match"]
        ));
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

        assert_eq!(report.conflict_count, 0);
        assert_eq!(report.review_required_count, 1);
        assert!(matches!(
            report.actions[0],
            CitationSyncAction::Conflict {
                suggestion: CitationSyncSuggestionKind::ReviewRequired,
                ..
            }
        ));
        assert!(matches!(
            report.actions[0],
            CitationSyncAction::Conflict { ref explanation, .. }
                if explanation.contains("keep this in review mode")
        ));
    }

    #[test]
    fn weak_narrow_fits_are_suppressed_in_preview() {
        let tempdir = workspace_with_reference("Benchmark atlas", "10.1234/atlas");
        let workspace = SourcerightWorkspace::for_document_or_dir(tempdir.path());
        let remote_path = tempdir.path().join("remote.json");
        fs::write(
            &remote_path,
            r#"[{"key":"zotero-1","item_type":"article-journal","title":"Benchmark analysis","doi":"10.9999/analysis"}]"#,
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

        assert_eq!(report.create_count, 0);
        assert_eq!(report.suppressed_count, 1);
        assert!(matches!(
            report.actions[0],
            CitationSyncAction::Create {
                suggestion: CitationSyncSuggestionKind::Suppressed,
                ..
            }
        ));
        assert!(matches!(
            report.actions[0],
            CitationSyncAction::Create { ref explanation, .. }
                if explanation.contains("suppresses it")
        ));
    }

    #[test]
    fn suppressed_preview_actions_stay_out_of_apply_writes() {
        let tempdir = workspace_with_reference("Benchmark atlas", "10.1234/atlas");
        let workspace = SourcerightWorkspace::for_document_or_dir(tempdir.path());
        let remote_path = tempdir.path().join("remote.json");
        fs::write(
            &remote_path,
            r#"[{"key":"zotero-1","item_type":"article-journal","title":"Benchmark analysis","doi":"10.9999/analysis"}]"#,
        )
        .expect("write remote");
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
        assert_eq!(report.suppressed_count, 1);
        assert_eq!(report.create_count, 0);

        let remote_after = fs::read_to_string(&remote_path).expect("remote after");
        assert!(remote_after.contains("Benchmark analysis"));
        assert!(!remote_after.contains("zotero-smith-2024"));
        assert!(
            fs::read_to_string(&audit_log_path)
                .expect("audit")
                .contains("suppressed")
        );
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
            CitationSyncAction::Update {
                suggestion: CitationSyncSuggestionKind::SafeUpdate,
                ..
            }
        ));
        assert!(matches!(
            report.actions[0],
            CitationSyncAction::Update { ref explanation, .. }
                if explanation.contains("safe metadata update")
        ));
    }

    #[test]
    fn fixture_backed_exact_match_is_no_op() {
        let tempdir = sample_workspace();
        let workspace = SourcerightWorkspace::for_document_or_dir(tempdir.path());
        let remote_path = tempdir.path().join("remote.json");
        let fixture = include_str!("../fixtures/providers/zotero/zotero-exact-match.json");
        fs::write(&remote_path, fixture).expect("write fixture");

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

        assert_eq!(report.skip_count, 1);
        assert_eq!(report.create_count, 0);
        assert_eq!(report.update_count, 0);
        assert!(matches!(
            report.actions[0],
            CitationSyncAction::Skip {
                suggestion: CitationSyncSuggestionKind::NoOp,
                ..
            }
        ));
    }

    #[test]
    fn fixture_backed_title_update_is_safe_update() {
        let tempdir = sample_workspace();
        let workspace = SourcerightWorkspace::for_document_or_dir(tempdir.path());
        let remote_path = tempdir.path().join("remote.json");
        let fixture = include_str!("../fixtures/providers/zotero/zotero-title-update.json");
        fs::write(&remote_path, fixture).expect("write fixture");

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
        assert_eq!(report.skip_count, 0);
        assert_eq!(report.create_count, 0);
        assert!(matches!(
            report.actions[0],
            CitationSyncAction::Update {
                suggestion: CitationSyncSuggestionKind::SafeUpdate,
                ..
            }
        ));
    }

    #[test]
    fn fixture_backed_empty_remote_proposes_create() {
        let tempdir = sample_workspace();
        let workspace = SourcerightWorkspace::for_document_or_dir(tempdir.path());
        let remote_path = tempdir.path().join("remote.json");
        let fixture = include_str!("../fixtures/providers/zotero/zotero-empty.json");
        fs::write(&remote_path, fixture).expect("write fixture");

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
        assert_eq!(report.skip_count, 0);
        assert_eq!(report.update_count, 0);
    }

    #[test]
    fn zotero_api_shaped_fixture_is_parsed_for_preview() {
        let tempdir = sample_workspace();
        let workspace = SourcerightWorkspace::for_document_or_dir(tempdir.path());
        let remote_path = tempdir.path().join("remote.json");
        fs::write(
            &remote_path,
            r#"[{"key":"ABC123DEF456","version":42,"data":{"key":"ABC123DEF456","itemType":"journalArticle","title":"Benchmark reference","DOI":"10.1234/benchmark"}}]"#,
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

        assert_eq!(report.skip_count, 1);
        assert!(matches!(
            report.actions[0],
            CitationSyncAction::Skip {
                ref zotero_key,
                suggestion: CitationSyncSuggestionKind::NoOp,
                ..
            } if zotero_key == "ABC123DEF456"
        ));
    }

    #[test]
    #[ignore = "requires explicit SOURCERIGHT_ZOTERO_LIVE_SMOKE=1 and disposable Zotero library credentials"]
    fn zotero_disposable_library_live_smoke_skips_without_credentials() {
        if std::env::var("SOURCERIGHT_ZOTERO_LIVE_SMOKE").as_deref() != Ok("1") {
            eprintln!("skipping Zotero live smoke: SOURCERIGHT_ZOTERO_LIVE_SMOKE is not 1");
            return;
        }

        let api_key = match std::env::var("SOURCERIGHT_ZOTERO_API_KEY") {
            Ok(value) if !value.trim().is_empty() => value,
            _ => {
                eprintln!("skipping Zotero live smoke: SOURCERIGHT_ZOTERO_API_KEY is not set");
                return;
            }
        };
        let library_id = match std::env::var("SOURCERIGHT_ZOTERO_LIBRARY_ID") {
            Ok(value) if !value.trim().is_empty() => value,
            _ => {
                eprintln!("skipping Zotero live smoke: SOURCERIGHT_ZOTERO_LIBRARY_ID is not set");
                return;
            }
        };
        let api_url = std::env::var("SOURCERIGHT_ZOTERO_API_URL")
            .unwrap_or_else(|_| "https://api.zotero.org".to_string());
        let library_type = std::env::var("SOURCERIGHT_ZOTERO_LIBRARY_TYPE")
            .unwrap_or_else(|_| "users".to_string());

        let tempdir = sample_workspace();
        let workspace = SourcerightWorkspace::for_document_or_dir(tempdir.path());

        let report = run_citation_sync(
            &workspace,
            CitationSyncConfig {
                preview: false,
                apply: false,
                audit_log_path: None,
                remote_fixture_path: None,
                zotero_api_url: Some(api_url),
                zotero_api_key: Some(api_key),
                zotero_library_id: Some(library_id),
                zotero_library_type: Some(library_type),
            },
        )
        .expect("live Zotero smoke should fetch the disposable library and plan without writes");

        assert!(!report.applied);
        assert!(report.audit_log_path.is_none());
        assert_eq!(
            report.actions.len(),
            report.create_count
                + report.update_count
                + report.skip_count
                + report.conflict_count
                + report.suppressed_count
                + report.review_required_count
        );
    }
}
