use std::collections::BTreeMap;
use std::fs;
use std::io::{self, BufRead, BufWriter, Write};
use std::path::PathBuf;

use serde_json::{Map, Value, json};

use sourceright::{
    ExportFormat, JournalPlatform, SourcerightPolicy, SourcerightWorkspace,
    analyze_claim_source_provenance, analyze_legal_citations, discover_plugins, evaluate_policy,
    parse_csl_json,
};

const SERVER_NAME: &str = "sourceright";
const SUPPORTED_PROTOCOL_VERSIONS: &[&str] =
    &["2025-11-25", "2025-06-18", "2025-03-26", "2024-11-05"];
const MCP_AUDIT_LOG_NAME: &str = "mcp-audit.jsonl";
const MCP_WRITE_PLAN_SCHEMA_VERSION: &str = "sourceright.mcp_write_plan.v1";
const MCP_AUDIT_SCHEMA_VERSION: &str = "sourceright.mcp_audit.v1";

pub fn serve_stdio() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut runtime = McpRuntime::new(default_workspace_root());
    let mut writer = BufWriter::new(stdout.lock());

    for line in stdin.lock().lines() {
        let line = line?;
        let responses = runtime.handle_line(&line);
        for response in responses {
            writeln!(
                writer,
                "{}",
                serde_json::to_string(&response).expect("serialize response")
            )?;
        }
        writer.flush()?;
    }

    Ok(())
}

struct McpRuntime {
    workspace: SourcerightWorkspace,
    initialized: bool,
}

impl McpRuntime {
    fn new(workspace_root: PathBuf) -> Self {
        Self {
            workspace: SourcerightWorkspace::from_root_or_parent(workspace_root),
            initialized: false,
        }
    }

    fn handle_line(&mut self, line: &str) -> Vec<Value> {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return Vec::new();
        }

        match serde_json::from_str::<Value>(trimmed) {
            Ok(Value::Array(messages)) => messages
                .into_iter()
                .filter_map(|message| self.handle_message(message))
                .collect(),
            Ok(message) => self.handle_message(message).into_iter().collect(),
            Err(_) => vec![jsonrpc_error(
                Value::Null,
                -32700,
                "Parse error",
                Some(json!({"line": trimmed})),
            )],
        }
    }

    fn handle_message(&mut self, message: Value) -> Option<Value> {
        let method = message.get("method")?.as_str()?.to_string();
        let id = message.get("id").cloned();

        if is_notification(&message) {
            self.handle_notification(&method, message.get("params"));
            return None;
        }

        let response = match method.as_str() {
            "initialize" => self.handle_initialize(message.get("params")),
            "ping" => Ok(json!({})),
            "tools/list" => self
                .ensure_initialized()
                .map(|_| json!({ "tools": tools_list() })),
            "tools/call" => self
                .ensure_initialized()
                .and_then(|_| self.handle_tool_call(message.get("params"))),
            "resources/list" => self
                .ensure_initialized()
                .map(|_| json!({ "resources": resources_list() })),
            "resources/read" => self
                .ensure_initialized()
                .and_then(|_| self.handle_resource_read(message.get("params"))),
            "prompts/list" => self
                .ensure_initialized()
                .map(|_| json!({ "prompts": prompts_list() })),
            "prompts/get" => self
                .ensure_initialized()
                .and_then(|_| self.handle_prompt_get(message.get("params"))),
            "shutdown" => Ok(json!({})),
            _ => Err(jsonrpc_error_details(
                -32601,
                "Method not found",
                Some(json!({ "method": method })),
            )),
        };

        Some(match response {
            Ok(result) => jsonrpc_result(id.unwrap_or(Value::Null), result),
            Err(error) => jsonrpc_error_from_id(id.unwrap_or(Value::Null), error),
        })
    }

    fn handle_notification(&mut self, method: &str, _params: Option<&Value>) {
        if method == "notifications/initialized" {
            self.initialized = true;
        }
    }

    fn ensure_initialized(&self) -> Result<(), RpcError> {
        if self.initialized {
            Ok(())
        } else {
            Err(RpcError::new(
                -32600,
                "Server not initialized",
                Some(json!({"expected":"initialize"})),
            ))
        }
    }

    fn handle_initialize(&mut self, params: Option<&Value>) -> Result<Value, RpcError> {
        let protocol_version = params
            .and_then(|params| params.get("protocolVersion"))
            .and_then(Value::as_str)
            .unwrap_or(SUPPORTED_PROTOCOL_VERSIONS[0]);
        let negotiated_version = negotiate_protocol_version(protocol_version);

        self.initialized = true;

        Ok(json!({
            "protocolVersion": negotiated_version,
            "serverInfo": {
                "name": SERVER_NAME,
                "version": env!("CARGO_PKG_VERSION"),
            },
            "capabilities": {
                "tools": {},
                "resources": {},
                "prompts": {},
            },
            "instructions": "Local reference verification server with read-only resources and explicit apply-gated write tools",
        }))
    }

    fn handle_tool_call(&self, params: Option<&Value>) -> Result<Value, RpcError> {
        let params =
            params.ok_or_else(|| RpcError::new(-32602, "Missing tool call parameters", None))?;
        let name = params
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| RpcError::new(-32602, "Tool name is required", None))?;
        let arguments = params.get("arguments").unwrap_or(&Value::Null);

        match name {
            "mcp.status" => Ok(text_result(serde_json::to_string(&status_payload())?)),
            "references.validate_csl" => {
                let path = required_path(arguments, "path", "references.csl.json")?;
                let diagnostics = sourceright::validate_csl_json(&fs::read_to_string(&path)?)
                    .map_err(|error| RpcError::new(-32603, error.to_string(), None))?;
                let output = json!({
                    "ok": diagnostics.is_empty(),
                    "path": path.display().to_string(),
                    "diagnostics": diagnostics,
                });
                Ok(text_result(serde_json::to_string(&output)?))
            }
            "references.report" => {
                let workspace = workspace_from_arguments(&self.workspace, arguments);
                let report = workspace
                    .reference_report_json()
                    .map_err(|error| RpcError::new(-32603, error.to_string(), None))?;
                Ok(text_result(serde_json::to_string(&report)?))
            }
            "references.review_queue" => {
                let workspace = workspace_from_arguments(&self.workspace, arguments);
                Ok(text_result(review_queue_jsonl(&workspace)?))
            }
            "references.citations" => {
                let manuscript = required_path(arguments, "manuscript", "manuscript.txt")?;
                let workspace = workspace_from_arguments(&self.workspace, arguments);
                let text = fs::read_to_string(&manuscript)
                    .map_err(|error| RpcError::new(-32603, error.to_string(), None))?;
                let report = workspace
                    .citation_reconciliation_report(&text)
                    .map_err(|error| RpcError::new(-32603, error.to_string(), None))?;
                Ok(text_result(report.to_markdown()))
            }
            "journal.screen_submission" => {
                let workspace = workspace_from_arguments(&self.workspace, arguments);
                let platform = arguments
                    .get("platform")
                    .and_then(Value::as_str)
                    .map(parse_journal_platform)
                    .transpose()?
                    .unwrap_or(JournalPlatform::GenericWebhook);
                let submission_id = string_arg(arguments, "submission_id")
                    .unwrap_or("mcp-submission")
                    .to_string();
                let manuscript_label = string_arg(arguments, "manuscript_label")
                    .unwrap_or("manuscript")
                    .to_string();
                let report = workspace
                    .journal_screening_report(submission_id, platform, manuscript_label)
                    .map_err(|error| RpcError::new(-32603, error.to_string(), None))?;
                Ok(text_result(serde_json::to_string(&report)?))
            }
            "legal.analyze_citations" => {
                let path = required_path(arguments, "path", "legal-text.txt")?;
                let text = fs::read_to_string(&path)
                    .map_err(|error| RpcError::new(-32603, error.to_string(), None))?;
                let report = analyze_legal_citations(&text);
                Ok(text_result(serde_json::to_string(&report)?))
            }
            "provenance.analyze_claim_sources" => {
                let path = required_path(arguments, "path", "document-text.txt")?;
                let text = fs::read_to_string(&path)
                    .map_err(|error| RpcError::new(-32603, error.to_string(), None))?;
                let report = analyze_claim_source_provenance(&text);
                Ok(text_result(serde_json::to_string(&report)?))
            }
            "references.policy" => {
                let references_csl_json = path_arg(arguments, "references_csl_json")
                    .unwrap_or_else(|| self.workspace.references_csl_json.clone());
                let csl_json = fs::read_to_string(&references_csl_json)
                    .map_err(|error| RpcError::new(-32603, error.to_string(), None))?;
                let document = parse_csl_json(&csl_json)
                    .map_err(|error| RpcError::new(-32603, error.to_string(), None))?;
                let policy = if let Some(policy_path) = path_arg(arguments, "policy_json") {
                    let policy_json = fs::read_to_string(policy_path)
                        .map_err(|error| RpcError::new(-32603, error.to_string(), None))?;
                    serde_json::from_str(&policy_json)
                        .map_err(|error| RpcError::new(-32603, error.to_string(), None))?
                } else {
                    SourcerightPolicy::journal_vancouver()
                };
                let report = evaluate_policy(&document, &policy);
                Ok(text_result(serde_json::to_string(&report)?))
            }
            "exports.preview" => {
                let workspace = workspace_from_arguments(&self.workspace, arguments);
                let format = arguments
                    .get("format")
                    .and_then(Value::as_str)
                    .map(parse_export_format)
                    .transpose()?;
                let manifest = workspace
                    .export_manifest(format)
                    .map_err(|error| RpcError::new(-32603, error.to_string(), None))?;
                Ok(text_result(serde_json::to_string(&manifest)?))
            }
            "plugins.list" => {
                if arguments
                    .as_object()
                    .is_some_and(|values| !values.is_empty())
                {
                    return Err(RpcError::new(
                        -32602,
                        "plugins.list does not accept arguments",
                        None,
                    ));
                }
                let report = discover_plugins()
                    .map_err(|error| RpcError::new(-32603, error.to_string(), None))?;
                Ok(text_result(serde_json::to_string(&report)?))
            }
            "workspace.init" => {
                let workspace = init_workspace_from_arguments(&self.workspace, arguments);
                let apply_requested = bool_arg(arguments, "apply").unwrap_or(false);
                let changes = vec![
                    write_change(
                        "create_directory",
                        workspace.exports_dir.display().to_string(),
                    ),
                    write_change(
                        "create_file_if_missing",
                        workspace.references_csl_json.display().to_string(),
                    ),
                    write_change(
                        "create_file_if_missing",
                        workspace.verification_sidecar_json.display().to_string(),
                    ),
                    write_change(
                        "create_file_if_missing",
                        workspace.review_queue_jsonl.display().to_string(),
                    ),
                ];
                let result = if apply_requested {
                    workspace
                        .init()
                        .map_err(|error| RpcError::new(-32603, error.to_string(), None))?;
                    let audit_path = append_audit_record(
                        &workspace,
                        json!({
                            "schema_version": MCP_AUDIT_SCHEMA_VERSION,
                            "tool": name,
                            "applied": true,
                            "workspace": workspace.root.display().to_string(),
                            "changes": changes,
                        }),
                    )?;
                    json!({
                        "schema_version": MCP_WRITE_PLAN_SCHEMA_VERSION,
                        "tool": name,
                        "apply_requested": true,
                        "applied": true,
                        "workspace": workspace.root.display().to_string(),
                        "changes": [
                            {"kind": "create_directory", "path": workspace.exports_dir.display().to_string()},
                            {"kind": "create_file_if_missing", "path": workspace.references_csl_json.display().to_string()},
                            {"kind": "create_file_if_missing", "path": workspace.verification_sidecar_json.display().to_string()},
                            {"kind": "create_file_if_missing", "path": workspace.review_queue_jsonl.display().to_string()},
                        ],
                        "audit_log": audit_path.display().to_string(),
                    })
                } else {
                    write_plan_payload(name, false, &workspace, changes)
                };
                Ok(text_result(serde_json::to_string(&result)?))
            }
            "review.import_decisions" => {
                let workspace = literal_workspace_from_arguments(&self.workspace, arguments);
                let apply_requested = bool_arg(arguments, "apply").unwrap_or(false);
                let decisions_value = arguments
                    .get("decisions")
                    .cloned()
                    .ok_or_else(|| RpcError::new(-32602, "decisions is required", None))?;
                let decisions: Vec<sourceright::ReviewDecisionImport> =
                    serde_json::from_value(decisions_value)?;
                let changes = vec![
                    write_change(
                        "update_file",
                        workspace.verification_sidecar_json.display().to_string(),
                    ),
                    write_change(
                        "update_file",
                        workspace.review_queue_jsonl.display().to_string(),
                    ),
                    write_change(
                        "import_review_decisions",
                        format!("{} decision record(s)", decisions.len()),
                    ),
                ];
                let result = if apply_requested {
                    let report = workspace
                        .import_review_decisions(&decisions)
                        .map_err(|error| RpcError::new(-32603, error.to_string(), None))?;
                    let audit_path = append_audit_record(
                        &workspace,
                        json!({
                            "schema_version": MCP_AUDIT_SCHEMA_VERSION,
                            "tool": name,
                            "applied": true,
                            "workspace": workspace.root.display().to_string(),
                            "decision_count": decisions.len(),
                            "changes": changes,
                        }),
                    )?;
                    json!({
                        "schema_version": MCP_WRITE_PLAN_SCHEMA_VERSION,
                        "tool": name,
                        "apply_requested": true,
                        "applied": true,
                        "workspace": workspace.root.display().to_string(),
                        "decision_count": decisions.len(),
                        "report": report,
                        "changes": [
                            {"kind": "update_file", "path": workspace.verification_sidecar_json.display().to_string()},
                            {"kind": "update_file", "path": workspace.review_queue_jsonl.display().to_string()},
                            {"kind": "import_review_decisions", "path": format!("{} decision record(s)", decisions.len())},
                        ],
                        "audit_log": audit_path.display().to_string(),
                    })
                } else {
                    write_plan_payload(name, false, &workspace, changes)
                };
                Ok(text_result(serde_json::to_string(&result)?))
            }
            "exports.write" => {
                let workspace = literal_workspace_from_arguments(&self.workspace, arguments);
                let apply_requested = bool_arg(arguments, "apply").unwrap_or(false);
                let format = arguments
                    .get("format")
                    .and_then(Value::as_str)
                    .map(parse_export_format)
                    .transpose()?;
                let planned = workspace
                    .export_manifest(format)
                    .map_err(|error| RpcError::new(-32603, error.to_string(), None))?;
                let changes = planned
                    .artifacts
                    .iter()
                    .map(|artifact| write_change("write_file", artifact.filename.clone()))
                    .collect::<Vec<_>>();
                let result = if apply_requested {
                    let written_paths = workspace
                        .write_exports(format)
                        .map_err(|error| RpcError::new(-32603, error.to_string(), None))?;
                    let audit_path = append_audit_record(
                        &workspace,
                        json!({
                            "schema_version": MCP_AUDIT_SCHEMA_VERSION,
                            "tool": name,
                            "applied": true,
                            "workspace": workspace.root.display().to_string(),
                            "format": format.map(|value| format!("{value:?}")).unwrap_or_else(|| "suite".to_string()),
                            "changes": changes,
                            "written_paths": written_paths.iter().map(|path| path.display().to_string()).collect::<Vec<_>>(),
                        }),
                    )?;
                    json!({
                        "schema_version": MCP_WRITE_PLAN_SCHEMA_VERSION,
                        "tool": name,
                        "apply_requested": true,
                        "applied": true,
                        "workspace": workspace.root.display().to_string(),
                        "format": format.map(|value| format!("{value:?}")).unwrap_or_else(|| "suite".to_string()),
                        "written_paths": written_paths.iter().map(|path| path.display().to_string()).collect::<Vec<_>>(),
                        "audit_log": audit_path.display().to_string(),
                    })
                } else {
                    write_plan_payload(name, false, &workspace, changes)
                };
                Ok(text_result(serde_json::to_string(&result)?))
            }
            _ => Err(RpcError::new(
                -32602,
                format!("Unknown tool: {name}"),
                Some(json!({ "name": name })),
            )),
        }
    }

    fn handle_resource_read(&self, params: Option<&Value>) -> Result<Value, RpcError> {
        let params = params
            .ok_or_else(|| RpcError::new(-32602, "Missing resource read parameters", None))?;
        let uri = params
            .get("uri")
            .and_then(Value::as_str)
            .ok_or_else(|| RpcError::new(-32602, "Resource URI is required", None))?;
        let (base_uri, query) = split_uri(uri);
        let workspace = workspace_from_query(&self.workspace, &query);

        match base_uri {
            "sourceright://reports/reference-integrity" => {
                let report = workspace
                    .reference_report_mcp_resource()
                    .map_err(|error| RpcError::new(-32603, error.to_string(), None))?;
                Ok(json!({
                    "contents": [{
                        "uri": uri,
                        "mimeType": "application/json",
                        "text": serde_json::to_string(&report)?,
                    }]
                }))
            }
            "sourceright://workspaces/local/review-queue" => Ok(json!({
                "contents": [{
                    "uri": uri,
                    "mimeType": "application/jsonl",
                    "text": review_queue_jsonl(&workspace)?,
                }]
            })),
            "sourceright://reports/policy" => {
                let report = policy_report(&workspace)?;
                Ok(json!({
                    "contents": [{
                        "uri": uri,
                        "mimeType": "application/json",
                        "text": serde_json::to_string(&report)?,
                    }]
                }))
            }
            "sourceright://plugins/registry" => {
                let report = discover_plugins()
                    .map_err(|error| RpcError::new(-32603, error.to_string(), None))?;
                Ok(json!({
                    "contents": [{
                        "uri": uri,
                        "mimeType": "application/json",
                        "text": serde_json::to_string(&report)?,
                    }]
                }))
            }
            "sourceright://reports/citation-reconciliation" => {
                let manuscript = query
                    .get("manuscript")
                    .or_else(|| query.get("path"))
                    .ok_or_else(|| {
                        RpcError::new(
                            -32602,
                            "citation-reconciliation requires manuscript or path query parameter",
                            None,
                        )
                    })?;
                let text = fs::read_to_string(manuscript)
                    .map_err(|error| RpcError::new(-32603, error.to_string(), None))?;
                let report = workspace
                    .citation_reconciliation_report(&text)
                    .map_err(|error| RpcError::new(-32603, error.to_string(), None))?;
                Ok(json!({
                    "contents": [{
                        "uri": uri,
                        "mimeType": "text/markdown",
                        "text": report.to_markdown(),
                    }]
                }))
            }
            "sourceright://reports/journal-screening" => {
                let submission_id = query
                    .get("submission_id")
                    .cloned()
                    .unwrap_or_else(|| "mcp-submission".to_string());
                let manuscript_label = query
                    .get("manuscript_label")
                    .cloned()
                    .unwrap_or_else(|| "manuscript".to_string());
                let platform = query
                    .get("platform")
                    .map(|value| parse_journal_platform(value))
                    .transpose()?
                    .unwrap_or(JournalPlatform::GenericWebhook);
                let report = workspace
                    .journal_screening_report(submission_id, platform, manuscript_label)
                    .map_err(|error| RpcError::new(-32603, error.to_string(), None))?;
                Ok(json!({
                    "contents": [{
                        "uri": uri,
                        "mimeType": "application/json",
                        "text": serde_json::to_string(&report)?,
                    }]
                }))
            }
            "sourceright://reports/legal-citations" => {
                let path = query.get("path").ok_or_else(|| {
                    RpcError::new(
                        -32602,
                        "legal-citations requires a path query parameter",
                        None,
                    )
                })?;
                let text = fs::read_to_string(path)
                    .map_err(|error| RpcError::new(-32603, error.to_string(), None))?;
                let report = analyze_legal_citations(&text);
                Ok(json!({
                    "contents": [{
                        "uri": uri,
                        "mimeType": "application/json",
                        "text": serde_json::to_string(&report)?,
                    }]
                }))
            }
            "sourceright://reports/claim-source-provenance" => {
                let path = query.get("path").ok_or_else(|| {
                    RpcError::new(
                        -32602,
                        "claim-source-provenance requires a path query parameter",
                        None,
                    )
                })?;
                let text = fs::read_to_string(path)
                    .map_err(|error| RpcError::new(-32603, error.to_string(), None))?;
                let report = analyze_claim_source_provenance(&text);
                Ok(json!({
                    "contents": [{
                        "uri": uri,
                        "mimeType": "application/json",
                        "text": serde_json::to_string(&report)?,
                    }]
                }))
            }
            _ => Err(RpcError::new(
                -32602,
                format!("Unknown resource URI: {uri}"),
                Some(json!({ "uri": uri })),
            )),
        }
    }

    fn handle_prompt_get(&self, params: Option<&Value>) -> Result<Value, RpcError> {
        let params =
            params.ok_or_else(|| RpcError::new(-32602, "Missing prompt parameters", None))?;
        let name = params
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| RpcError::new(-32602, "Prompt name is required", None))?;
        let arguments = params.get("arguments").and_then(Value::as_object);

        let (title, description, body) = prompt_definition(name, arguments)
            .ok_or_else(|| RpcError::new(-32602, format!("Unknown prompt: {name}"), None))?;

        Ok(json!({
            "description": description,
            "messages": [{
                "role": "user",
                "content": {
                    "type": "text",
                    "text": body,
                }
            }],
            "title": title,
        }))
    }
}

fn default_workspace_root() -> PathBuf {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    if cwd.file_name().and_then(|name| name.to_str()) == Some(".sourceright") {
        cwd
    } else {
        cwd.join(".sourceright")
    }
}

/// Static MCP server card for Smithery URL publish and SEP-1649 discovery.
///
/// Hosted at `/.well-known/mcp/server-card.json` on the docs site so registries
/// can scan tool/resource/prompt surfaces without a Streamable HTTP transport.
pub fn server_card() -> Value {
    json!({
        "serverInfo": {
            "name": SERVER_NAME,
            "version": env!("CARGO_PKG_VERSION"),
        },
        "authentication": {
            "required": false,
        },
        "tools": tools_list(),
        "resources": resources_list(),
        "prompts": prompts_list(),
    })
}

fn status_payload() -> Value {
    json!({
        "server_mode": "stdio",
        "transport": "stdio",
        "server_started": false,
        "available_tools": tools_list().as_array().map(|items| items.len()).unwrap_or(0),
        "available_resources": resources_list().as_array().map(|items| items.len()).unwrap_or(0),
        "available_prompts": prompts_list().as_array().map(|items| items.len()).unwrap_or(0),
        "implemented_read_only_surfaces": [
            "sourceright validate-csl <references.csl.json>",
            "sourceright report --json [.sourceright-directory]",
            "sourceright report --mcp-resource [.sourceright-directory]",
            "sourceright conflicts [.sourceright-directory]",
            "sourceright citations <manuscript.txt> [.sourceright-directory]",
            "sourceright review queue|partitions",
            "sourceright journal-screen [.sourceright-directory]",
            "sourceright legal <legal-text.txt>",
            "sourceright provenance <document-text.txt>",
            "sourceright policy <references.csl.json>",
            "sourceright plugins [validate] [--json]",
        ],
        "implemented_apply_gated_write_surfaces": [
            "workspace.init apply=true",
            "sourceright review import-decisions apply=true",
            "review.import_decisions apply=true",
            "exports.write apply=true",
            "sourceright export --all [.sourceright-directory]",
        ],
        "resource_uris": [
            "sourceright://reports/reference-integrity",
            "sourceright://reports/citation-reconciliation",
            "sourceright://workspaces/local/review-queue",
            "sourceright://reports/journal-screening",
            "sourceright://reports/legal-citations",
            "sourceright://reports/claim-source-provenance",
            "sourceright://reports/policy",
            "sourceright://plugins/registry",
        ],
        "message": "MCP server mode is implemented; run `sourceright mcp` to start the stdio server.",
    })
}

fn tools_list() -> Value {
    json!([
        tool_definition(
            "mcp.status",
            "Inspect server readiness and the read-only MCP surface.",
            json!({
                "type": "object",
                "properties": {
                    "format": { "type": "string", "enum": ["json"] }
                },
                "additionalProperties": false
            }),
        ),
        tool_definition(
            "references.validate_csl",
            "Validate canonical CSL JSON and return deterministic diagnostics.",
            json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string" }
                },
                "required": ["path"],
                "additionalProperties": false
            }),
        ),
        tool_definition(
            "references.report",
            "Generate the reference integrity report from a local workspace.",
            json!({
                "type": "object",
                "properties": {
                    "workspace": { "type": "string" }
                },
                "additionalProperties": false
            }),
        ),
        tool_definition(
            "references.review_queue",
            "Return the derived review queue as JSONL.",
            json!({
                "type": "object",
                "properties": {
                    "workspace": { "type": "string" }
                },
                "additionalProperties": false
            }),
        ),
        tool_definition(
            "references.citations",
            "Reconcile in-text citations against the workspace references.",
            json!({
                "type": "object",
                "properties": {
                    "manuscript": { "type": "string" },
                    "workspace": { "type": "string" }
                },
                "required": ["manuscript"],
                "additionalProperties": false
            }),
        ),
        tool_definition(
            "journal.screen_submission",
            "Generate a journal screening report from workspace references.",
            json!({
                "type": "object",
                "properties": {
                    "workspace": { "type": "string" },
                    "platform": {
                        "type": "string",
                        "description": "Journal screening contract label. Accepted aliases include generic-webhook, ojs, arxiv-submit-ce, arxiv_submission_core, scholarone, editorial-manager, ejournalpress, and manuscript-manager.",
                        "enum": [
                            "generic-webhook",
                            "generic_webhook",
                            "ojs",
                            "arxiv-submit-ce",
                            "arxiv_submit_ce",
                            "arxiv-submission-core",
                            "arxiv_submission_core",
                            "scholarone",
                            "scholar_one",
                            "editorial-manager",
                            "editorial_manager",
                            "ejournalpress",
                            "e-journal-press",
                            "e_journal_press",
                            "manuscript-manager",
                            "manuscript_manager"
                        ],
                        "examples": ["generic-webhook", "arxiv-submit-ce", "arxiv-submission-core"]
                    },
                    "submission_id": { "type": "string" },
                    "manuscript_label": { "type": "string" }
                },
                "additionalProperties": false
            }),
        ),
        tool_definition(
            "legal.analyze_citations",
            "Audit legal citations into the separate legal model with jurisdiction/provider hints and attorney-review flags; does not provide legal advice.",
            json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string" }
                },
                "required": ["path"],
                "additionalProperties": false
            }),
        ),
        tool_definition(
            "provenance.analyze_claim_sources",
            "Build a claim/source provenance report from document text.",
            json!({
                "type": "object",
                "properties": {
                    "path": { "type": "string" }
                },
                "required": ["path"],
                "additionalProperties": false
            }),
        ),
        tool_definition(
            "references.policy",
            "Evaluate deterministic style and recency policy checks.",
            json!({
                "type": "object",
                "properties": {
                    "references_csl_json": { "type": "string" },
                    "policy_json": { "type": "string" }
                },
                "required": ["references_csl_json"],
                "additionalProperties": false
            }),
        ),
        tool_definition(
            "exports.preview",
            "Preview export artifacts without writing files.",
            json!({
                "type": "object",
                "properties": {
                    "workspace": { "type": "string" },
                    "format": { "type": "string", "enum": ["yaml", "xml", "ris", "enw", "biblatex"] }
                },
                "additionalProperties": false
            }),
        ),
        tool_definition(
            "plugins.list",
            "Discover validated plugin manifests and runtime execution gates.",
            json!({
                "type": "object",
                "properties": {},
                "additionalProperties": false
            }),
        ),
        tool_definition_with_read_only(
            "workspace.init",
            "Create the local workspace structure with dry-run planning and explicit apply.",
            json!({
                "type": "object",
                "properties": {
                    "workspace": { "type": "string" },
                    "apply": { "type": "boolean" }
                },
                "additionalProperties": false
            }),
            false,
        ),
        tool_definition_with_read_only(
            "review.import_decisions",
            "Import review decisions with dry-run planning and audit logging.",
            json!({
                "type": "object",
                "properties": {
                    "workspace": { "type": "string" },
                    "apply": { "type": "boolean" },
                    "decisions": {
                        "type": "array",
                        "items": { "type": "object" }
                    }
                },
                "required": ["decisions"],
                "additionalProperties": false
            }),
            false,
        ),
        tool_definition_with_read_only(
            "exports.write",
            "Write export files with dry-run planning and audit logging.",
            json!({
                "type": "object",
                "properties": {
                    "workspace": { "type": "string" },
                    "apply": { "type": "boolean" },
                    "format": { "type": "string", "enum": ["yaml", "xml", "ris", "enw", "biblatex"] }
                },
                "additionalProperties": false
            }),
            false,
        ),
    ])
}

fn resources_list() -> Value {
    json!([
        resource_definition(
            "sourceright://reports/reference-integrity",
            "reference-integrity",
            "Reference integrity report",
            "Reference integrity report derived from the workspace.",
            "application/json",
        ),
        resource_definition(
            "sourceright://reports/citation-reconciliation",
            "citation-reconciliation",
            "Citation reconciliation report",
            "Citation reconciliation report derived from a manuscript text input.",
            "text/markdown",
        ),
        resource_definition(
            "sourceright://workspaces/local/review-queue",
            "review-queue",
            "Review queue",
            "Derived JSONL review queue for the local workspace.",
            "application/jsonl",
        ),
        resource_definition(
            "sourceright://reports/journal-screening",
            "journal-screening",
            "Journal screening report",
            "Journal screening report derived from workspace references and screening inputs.",
            "application/json",
        ),
        resource_definition(
            "sourceright://reports/legal-citations",
            "legal-citations",
            "Legal citation report",
            "Legal citation report derived from a local legal text input.",
            "application/json",
        ),
        resource_definition(
            "sourceright://reports/claim-source-provenance",
            "claim-source-provenance",
            "Claim/source provenance report",
            "Claim/source provenance report derived from a local text input.",
            "application/json",
        ),
        resource_definition(
            "sourceright://reports/policy",
            "policy",
            "Policy report",
            "Deterministic policy report derived from the workspace CSL file.",
            "application/json",
        ),
        resource_definition(
            "sourceright://plugins/registry",
            "plugin-registry",
            "Plugin registry report",
            "Validated plugin registry and manifest discovery report.",
            "application/json",
        ),
    ])
}

fn prompts_list() -> Value {
    json!([
        prompt_definition_entry(
            "manual_reference_review",
            "Manual reference review",
            "Guide manual review of queued references using CSL and sidecar evidence.",
        ),
        prompt_definition_entry(
            "citation_integrity_explanation",
            "Citation integrity explanation",
            "Explain reference report issues without claiming author intent.",
        ),
        prompt_definition_entry(
            "provider_conflict_explanation",
            "Provider conflict explanation",
            "Explain provider/canonical conflicts and the no-silent-overwrite rule.",
        ),
        prompt_definition_entry(
            "legal_citation_review",
            "Legal citation review",
            "Review separate legal citation records, jurisdiction/provider issues, and attorney-review flags without giving legal advice.",
        ),
        prompt_definition_entry(
            "claim_source_provenance_review",
            "Claim/source provenance review",
            "Review claim/source linkage without claim-truth scoring.",
        ),
    ])
}

fn tool_definition(name: &str, description: &str, input_schema: Value) -> Value {
    json!({
        "name": name,
        "description": description,
        "inputSchema": input_schema,
        "annotations": {
            "readOnlyHint": true,
        },
    })
}

fn tool_definition_with_read_only(
    name: &str,
    description: &str,
    input_schema: Value,
    read_only: bool,
) -> Value {
    json!({
        "name": name,
        "description": description,
        "inputSchema": input_schema,
        "annotations": {
            "readOnlyHint": read_only,
        },
    })
}

fn resource_definition(
    uri: &str,
    name: &str,
    title: &str,
    description: &str,
    mime_type: &str,
) -> Value {
    json!({
        "uri": uri,
        "name": name,
        "title": title,
        "description": description,
        "mimeType": mime_type,
    })
}

fn prompt_definition_entry(name: &str, title: &str, description: &str) -> Value {
    json!({
        "name": name,
        "title": title,
        "description": description,
    })
}

fn prompt_definition(
    name: &str,
    arguments: Option<&Map<String, Value>>,
) -> Option<(&'static str, &'static str, String)> {
    let body = match name {
        "manual_reference_review" => prompt_body(
            "Manual reference review",
            "Review queued references using CSL JSON and verification sidecar evidence.",
            arguments,
        ),
        "citation_integrity_explanation" => prompt_body(
            "Citation integrity explanation",
            "Explain reference report issues without implying author intent or AI authorship.",
            arguments,
        ),
        "provider_conflict_explanation" => prompt_body(
            "Provider conflict explanation",
            "Explain provider/canonical conflicts and preserve the no-silent-overwrite rule.",
            arguments,
        ),
        "legal_citation_review" => prompt_body(
            "Legal citation review",
            "Review separate legal citation records, jurisdiction/provider issues, and attorney-review flags without giving legal advice.",
            arguments,
        ),
        "claim_source_provenance_review" => prompt_body(
            "Claim/source provenance review",
            "Review claim/source linkage without asserting claim truth.",
            arguments,
        ),
        _ => return None,
    };

    Some((prompt_title(name), prompt_description(name), body))
}

fn prompt_title(name: &str) -> &'static str {
    match name {
        "manual_reference_review" => "Manual reference review",
        "citation_integrity_explanation" => "Citation integrity explanation",
        "provider_conflict_explanation" => "Provider conflict explanation",
        "legal_citation_review" => "Legal citation review",
        "claim_source_provenance_review" => "Claim/source provenance review",
        _ => "Unknown prompt",
    }
}

fn prompt_description(name: &str) -> &'static str {
    match name {
        "manual_reference_review" => {
            "Guide manual review of queued references using CSL and sidecar evidence."
        }
        "citation_integrity_explanation" => {
            "Explain reference report issues without claiming author intent."
        }
        "provider_conflict_explanation" => {
            "Explain provider/canonical conflicts and the no-silent-overwrite rule."
        }
        "legal_citation_review" => {
            "Review separate legal citation records, jurisdiction/provider issues, and attorney-review flags without giving legal advice."
        }
        "claim_source_provenance_review" => {
            "Review claim/source linkage without claim-truth scoring."
        }
        _ => "Unknown prompt.",
    }
}

fn prompt_body(title: &str, purpose: &str, arguments: Option<&Map<String, Value>>) -> String {
    let mut body = format!("{title}\n\n{purpose}");
    if let Some(arguments) = arguments
        && !arguments.is_empty()
    {
        body.push_str("\n\nArguments:\n");
        body.push_str(
            &serde_json::to_string_pretty(arguments).unwrap_or_else(|_| "{}".to_string()),
        );
    }
    body
}

fn write_change(kind: &str, path: String) -> Value {
    json!({
        "kind": kind,
        "path": path,
    })
}

fn write_plan_payload(
    tool: &str,
    apply_requested: bool,
    workspace: &SourcerightWorkspace,
    changes: Vec<Value>,
) -> Value {
    json!({
        "schema_version": MCP_WRITE_PLAN_SCHEMA_VERSION,
        "tool": tool,
        "apply_requested": apply_requested,
        "applied": false,
        "workspace": workspace.root.display().to_string(),
        "changes": changes,
    })
}

fn append_audit_record(
    workspace: &SourcerightWorkspace,
    record: Value,
) -> Result<PathBuf, RpcError> {
    fs::create_dir_all(&workspace.root)?;
    let path = workspace.root.join(MCP_AUDIT_LOG_NAME);
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;
    writeln!(file, "{}", serde_json::to_string(&record)?)?;
    Ok(path)
}

fn workspace_from_arguments(
    default_workspace: &SourcerightWorkspace,
    args: &Value,
) -> SourcerightWorkspace {
    path_arg(args, "workspace")
        .map(SourcerightWorkspace::from_root_or_parent)
        .unwrap_or_else(|| default_workspace.clone())
}

fn literal_workspace_from_arguments(
    default_workspace: &SourcerightWorkspace,
    args: &Value,
) -> SourcerightWorkspace {
    path_arg(args, "workspace")
        .map(SourcerightWorkspace::from_root)
        .unwrap_or_else(|| default_workspace.clone())
}

fn init_workspace_from_arguments(
    default_workspace: &SourcerightWorkspace,
    args: &Value,
) -> SourcerightWorkspace {
    path_arg(args, "workspace")
        .map(SourcerightWorkspace::from_root)
        .unwrap_or_else(|| default_workspace.clone())
}

fn workspace_from_query(
    default_workspace: &SourcerightWorkspace,
    query: &BTreeMap<String, String>,
) -> SourcerightWorkspace {
    query
        .get("workspace")
        .map(PathBuf::from)
        .map(SourcerightWorkspace::from_root_or_parent)
        .unwrap_or_else(|| default_workspace.clone())
}

fn review_queue_jsonl(workspace: &SourcerightWorkspace) -> Result<String, RpcError> {
    let sidecar = if workspace.verification_sidecar_json.exists() {
        let text = fs::read_to_string(&workspace.verification_sidecar_json)
            .map_err(|error| RpcError::new(-32603, error.to_string(), None))?;
        sourceright::parse_verification_sidecar_json(&text)
            .map_err(|error| RpcError::new(-32603, error.to_string(), None))?
    } else {
        sourceright::VerificationSidecar::empty()
    };
    sidecar
        .to_review_queue_jsonl()
        .map_err(|error| RpcError::new(-32603, error.to_string(), None))
}

fn policy_report(workspace: &SourcerightWorkspace) -> Result<sourceright::PolicyReport, RpcError> {
    let csl_json = fs::read_to_string(&workspace.references_csl_json)
        .map_err(|error| RpcError::new(-32603, error.to_string(), None))?;
    let document = parse_csl_json(&csl_json)
        .map_err(|error| RpcError::new(-32603, error.to_string(), None))?;
    Ok(evaluate_policy(
        &document,
        &SourcerightPolicy::journal_vancouver(),
    ))
}

fn path_arg(args: &Value, key: &str) -> Option<PathBuf> {
    string_arg(args, key).map(PathBuf::from)
}

fn required_path(args: &Value, key: &str, label: &str) -> Result<PathBuf, RpcError> {
    path_arg(args, key)
        .ok_or_else(|| RpcError::new(-32602, format!("{key} is required ({label})"), None))
}

fn string_arg<'a>(args: &'a Value, key: &str) -> Option<&'a str> {
    args.get(key).and_then(Value::as_str)
}

fn bool_arg(args: &Value, key: &str) -> Option<bool> {
    args.get(key).and_then(Value::as_bool)
}

fn parse_export_format(value: &str) -> Result<ExportFormat, RpcError> {
    ExportFormat::parse(value).ok_or_else(|| {
        RpcError::new(
            -32602,
            format!("Unsupported export format: {value}"),
            Some(json!({ "format": value })),
        )
    })
}

fn parse_journal_platform(value: &str) -> Result<JournalPlatform, RpcError> {
    match value.to_ascii_lowercase().as_str() {
        "generic-webhook" | "generic_webhook" => Ok(JournalPlatform::GenericWebhook),
        "ojs" => Ok(JournalPlatform::Ojs),
        "arxiv-submit-ce" | "arxiv_submit_ce" => Ok(JournalPlatform::ArxivSubmitCe),
        "arxiv-submission-core" | "arxiv_submission_core" => {
            Ok(JournalPlatform::ArxivSubmissionCore)
        }
        "scholarone" | "scholar_one" => Ok(JournalPlatform::ScholarOne),
        "editorial-manager" | "editorial_manager" => Ok(JournalPlatform::EditorialManager),
        "ejournalpress" | "e-journal-press" | "e_journal_press" => {
            Ok(JournalPlatform::EJournalPress)
        }
        "manuscript-manager" | "manuscript_manager" => Ok(JournalPlatform::ManuscriptManager),
        _ => Err(RpcError::new(
            -32602,
            format!("Unsupported journal platform: {value}"),
            Some(json!({ "platform": value })),
        )),
    }
}

fn negotiate_protocol_version(requested: &str) -> &'static str {
    SUPPORTED_PROTOCOL_VERSIONS
        .iter()
        .copied()
        .find(|version| *version == requested)
        .unwrap_or(SUPPORTED_PROTOCOL_VERSIONS[0])
}

fn split_uri(uri: &str) -> (&str, BTreeMap<String, String>) {
    let mut parts = uri.splitn(2, '?');
    let base = parts.next().unwrap_or(uri);
    let mut query = BTreeMap::new();

    if let Some(raw_query) = parts.next() {
        for item in raw_query.split('&') {
            if item.is_empty() {
                continue;
            }
            let mut pair = item.splitn(2, '=');
            let key = percent_decode_query_component(pair.next().unwrap_or_default());
            let value = percent_decode_query_component(pair.next().unwrap_or_default());
            query.insert(key, value);
        }
    }

    (base, query)
}

fn percent_decode_query_component(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut output = Vec::with_capacity(bytes.len());
    let mut index = 0;

    while index < bytes.len() {
        match bytes[index] {
            b'+' => {
                output.push(b' ');
                index += 1;
            }
            b'%' if index + 2 < bytes.len() => {
                let high = bytes[index + 1];
                let low = bytes[index + 2];
                if let (Some(high), Some(low)) = (hex_value(high), hex_value(low)) {
                    output.push((high << 4) | low);
                    index += 3;
                } else {
                    output.push(bytes[index]);
                    index += 1;
                }
            }
            value => {
                output.push(value);
                index += 1;
            }
        }
    }

    String::from_utf8_lossy(&output).into_owned()
}

fn hex_value(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        b'A'..=b'F' => Some(value - b'A' + 10),
        _ => None,
    }
}

fn is_notification(message: &Value) -> bool {
    message.get("id").is_none() && message.get("method").is_some()
}

fn jsonrpc_result(id: Value, result: Value) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": result,
    })
}

fn jsonrpc_error_from_id(id: Value, error: RpcError) -> Value {
    jsonrpc_error(id, error.code, error.message, error.data)
}

fn jsonrpc_error(id: Value, code: i32, message: impl Into<String>, data: Option<Value>) -> Value {
    let mut error = json!({
        "code": code,
        "message": message.into(),
    });
    if let Some(data) = data {
        error["data"] = data;
    }
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": error,
    })
}

fn jsonrpc_error_details(code: i32, message: impl Into<String>, data: Option<Value>) -> RpcError {
    RpcError::new(code, message, data)
}

fn text_result(text: String) -> Value {
    json!({
        "content": [{
            "type": "text",
            "text": text,
        }],
        "isError": false,
    })
}

#[derive(Debug)]
struct RpcError {
    code: i32,
    message: String,
    data: Option<Value>,
}

impl RpcError {
    fn new(code: i32, message: impl Into<String>, data: Option<Value>) -> Self {
        Self {
            code,
            message: message.into(),
            data,
        }
    }
}

impl From<io::Error> for RpcError {
    fn from(error: io::Error) -> Self {
        Self::new(-32603, error.to_string(), None)
    }
}

impl From<serde_json::Error> for RpcError {
    fn from(error: serde_json::Error) -> Self {
        Self::new(-32603, error.to_string(), None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn response_text(response: &Value) -> &str {
        response["result"]["content"][0]["text"]
            .as_str()
            .expect("text result")
    }

    fn resource_text(response: &Value) -> &str {
        response["result"]["contents"][0]["text"]
            .as_str()
            .expect("resource text result")
    }

    fn prompt_text(response: &Value) -> &str {
        response["result"]["messages"][0]["content"]["text"]
            .as_str()
            .expect("prompt text result")
    }

    fn arxiv_fixture(path: &str) -> Value {
        serde_json::from_str(match path {
            "submit-ce" => include_str!("../fixtures/journal/arxiv-submit-ce-submission.json"),
            "submission-core" => {
                include_str!("../fixtures/journal/arxiv-submission-core-submission.json")
            }
            _ => panic!("unknown arXiv fixture: {path}"),
        })
        .expect("arXiv fixture JSON")
    }

    fn write_arxiv_fixture_workspace(workspace: &SourcerightWorkspace, fixture: &Value) {
        workspace.init().expect("init workspace");
        fs::write(
            &workspace.references_csl_json,
            serde_json::to_string(&fixture["csl_references"]).expect("serialize CSL fixture"),
        )
        .expect("write CSL fixture");
        fs::write(
            &workspace.verification_sidecar_json,
            serde_json::to_string(&fixture["verification_sidecar"])
                .expect("serialize sidecar fixture"),
        )
        .expect("write sidecar fixture");
    }

    fn percent_encode_query(input: &str) -> String {
        input
            .bytes()
            .flat_map(|byte| match byte {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    vec![byte as char]
                }
                _ => format!("%{byte:02X}").chars().collect(),
            })
            .collect()
    }

    fn seeded_workspace() -> (tempfile::TempDir, SourcerightWorkspace) {
        let tempdir = tempfile::tempdir().expect("workspace");
        let workspace = SourcerightWorkspace::from_root(tempdir.path().join(".sourceright"));
        workspace.init().expect("init workspace");
        fs::write(
            &workspace.references_csl_json,
            r#"[{"id":"smith-2024","type":"article-journal","title":"Benchmark reference","author":[{"family":"Smith"}],"DOI":"10.1234/benchmark"}]"#,
        )
        .expect("write csl");
        fs::write(
            &workspace.verification_sidecar_json,
            r#"{"schema_version":"sourceright.verification.v1","references":{"smith-2024":{"review_status":"queued"}}}"#,
        )
        .expect("write sidecar");
        fs::write(
            tempdir.path().join("manuscript.txt"),
            "Text cites (Smith, 2024).",
        )
        .expect("write manuscript");
        fs::write(
            tempdir.path().join("legal.txt"),
            "Smith v Jones [2024] NSWSC 1.",
        )
        .expect("write legal text");
        fs::write(
            tempdir.path().join("provenance.txt"),
            "The draft states that Smith (2024) supports the claim.",
        )
        .expect("write provenance text");
        (tempdir, workspace)
    }

    #[test]
    fn runtime_handles_initialize_manifest_queries_and_protocol_errors() {
        let mut runtime = McpRuntime::new(PathBuf::from(".sourceright"));

        let initialize = runtime.handle_line(
            r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25"}}"#,
        );
        assert_eq!(initialize.len(), 1);
        assert_eq!(initialize[0]["result"]["serverInfo"]["name"], SERVER_NAME);
        assert_eq!(initialize[0]["result"]["capabilities"]["tools"], json!({}));

        let tools = runtime.handle_line(r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#);
        assert_eq!(tools[0]["result"]["tools"].as_array().unwrap().len(), 14);

        let resources =
            runtime.handle_line(r#"{"jsonrpc":"2.0","id":3,"method":"resources/list"}"#);
        assert_eq!(
            resources[0]["result"]["resources"]
                .as_array()
                .unwrap()
                .len(),
            8
        );

        let prompts = runtime.handle_line(r#"{"jsonrpc":"2.0","id":4,"method":"prompts/list"}"#);
        assert_eq!(prompts[0]["result"]["prompts"].as_array().unwrap().len(), 5);

        let notification = runtime.handle_message(json!({
            "jsonrpc": "2.0",
            "method": "notifications/initialized"
        }));
        assert!(notification.is_none());

        let parse_error = runtime.handle_line("not json");
        assert_eq!(parse_error[0]["error"]["code"], -32700);

        let unknown = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 9,
                "method": "unknown.method"
            }))
            .expect("response");
        assert_eq!(unknown["error"]["code"], -32601);
    }

    #[test]
    fn runtime_handles_core_tools_resources_prompts_and_writes() {
        let (tempdir, workspace) = seeded_workspace();
        let mut runtime = McpRuntime {
            workspace: workspace.clone(),
            initialized: true,
        };

        let status = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 10,
                "method": "tools/call",
                "params": {
                    "name": "mcp.status",
                    "arguments": {}
                }
            }))
            .expect("status response");
        assert!(response_text(&status).contains("\"server_mode\":\"stdio\""));

        let validate = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 11,
                "method": "tools/call",
                "params": {
                    "name": "references.validate_csl",
                    "arguments": { "path": workspace.references_csl_json.display().to_string() }
                }
            }))
            .expect("validate response");
        assert!(response_text(&validate).contains("\"ok\":true"));

        let report = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 12,
                "method": "tools/call",
                "params": {
                    "name": "references.report",
                    "arguments": { "workspace": workspace.root.display().to_string() }
                }
            }))
            .expect("report response");
        assert!(response_text(&report).contains("smith-2024"));

        let review_queue = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 13,
                "method": "tools/call",
                "params": {
                    "name": "references.review_queue",
                    "arguments": { "workspace": workspace.root.display().to_string() }
                }
            }))
            .expect("review queue response");
        assert!(response_text(&review_queue).contains("smith-2024"));

        let citations = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 14,
                "method": "tools/call",
                "params": {
                    "name": "references.citations",
                    "arguments": {
                        "manuscript": tempdir.path().join("manuscript.txt").display().to_string(),
                        "workspace": workspace.root.display().to_string()
                    }
                }
            }))
            .expect("citations response");
        assert!(response_text(&citations).contains("Matched citations: 1"));

        let journal = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 15,
                "method": "tools/call",
                "params": {
                    "name": "journal.screen_submission",
                    "arguments": {
                        "workspace": workspace.root.display().to_string(),
                        "submission_id": "SUB-1",
                        "platform": "ojs",
                        "manuscript_label": "manuscript.docx"
                    }
                }
            }))
            .expect("journal response");
        assert!(response_text(&journal).contains("\"submission_id\":\"SUB-1\""));

        let legal = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 16,
                "method": "tools/call",
                "params": {
                    "name": "legal.analyze_citations",
                    "arguments": { "path": tempdir.path().join("legal.txt").display().to_string() }
                }
            }))
            .expect("legal response");
        assert!(response_text(&legal).contains("\"citation_type\":\"case\""));

        let provenance = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 17,
                "method": "tools/call",
                "params": {
                    "name": "provenance.analyze_claim_sources",
                    "arguments": { "path": tempdir.path().join("provenance.txt").display().to_string() }
                }
            }))
            .expect("provenance response");
        assert!(response_text(&provenance).contains("claim"));

        let policy = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 18,
                "method": "tools/call",
                "params": {
                    "name": "references.policy",
                    "arguments": {}
                }
            }))
            .expect("policy response");
        assert!(response_text(&policy).contains("sourceright.policy_report.v1"));

        let exports_preview = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 19,
                "method": "tools/call",
                "params": {
                    "name": "exports.preview",
                    "arguments": { "workspace": workspace.root.display().to_string() }
                }
            }))
            .expect("exports preview response");
        assert!(response_text(&exports_preview).contains("sourceright.export_manifest.v1"));

        let plugins = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 20,
                "method": "tools/call",
                "params": {
                    "name": "plugins.list",
                    "arguments": {}
                }
            }))
            .expect("plugins response");
        assert!(response_text(&plugins).contains("sourceright.plugin_registry_report.v1"));

        let workspace_init_dry_run = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 21,
                "method": "tools/call",
                "params": {
                    "name": "workspace.init",
                    "arguments": { "workspace": workspace.root.display().to_string() }
                }
            }))
            .expect("workspace init dry-run");
        let dry_run: Value = serde_json::from_str(response_text(&workspace_init_dry_run))
            .expect("parse dry-run payload");
        assert_eq!(dry_run["apply_requested"], false);
        assert_eq!(dry_run["applied"], false);
        assert!(dry_run.get("audit_log").is_none());

        let workspace_init_apply = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 22,
                "method": "tools/call",
                "params": {
                    "name": "workspace.init",
                    "arguments": { "workspace": workspace.root.display().to_string(), "apply": true }
                }
            }))
            .expect("workspace init apply");
        let apply: Value = serde_json::from_str(response_text(&workspace_init_apply))
            .expect("parse apply payload");
        assert!(apply["applied"].as_bool().expect("applied flag"));
        assert!(workspace.root.join(MCP_AUDIT_LOG_NAME).exists());

        let review_decisions = vec![sourceright::ReviewDecisionImport {
            reference_id: "smith-2024".to_string(),
            decision: "accepted".to_string(),
            reviewer: "tester".to_string(),
            decided_at: "2026-05-11T00:00:00Z".to_string(),
            status: sourceright::sidecar::ReviewStatus::Resolved,
            notes: None,
        }];
        let review_decisions_value =
            serde_json::to_value(&review_decisions).expect("serialize decisions");
        let review_import_preview = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 23,
                "method": "tools/call",
                "params": {
                    "name": "review.import_decisions",
                    "arguments": {
                        "workspace": workspace.root.display().to_string(),
                        "decisions": review_decisions_value
                    }
                }
            }))
            .expect("review import preview");
        let preview: Value = serde_json::from_str(response_text(&review_import_preview))
            .expect("parse preview payload");
        assert_eq!(preview["apply_requested"], false);
        assert!(preview.get("audit_log").is_none());

        let review_import_apply = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 24,
                "method": "tools/call",
                "params": {
                    "name": "review.import_decisions",
                    "arguments": {
                        "workspace": workspace.root.display().to_string(),
                        "decisions": serde_json::to_value(&review_decisions).expect("serialize decisions"),
                        "apply": true
                    }
                }
            }))
            .expect("review import apply");
        let import_apply: Value = serde_json::from_str(response_text(&review_import_apply))
            .expect("parse import payload");
        assert!(import_apply["applied"].as_bool().expect("applied flag"));

        let exports_write = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 25,
                "method": "tools/call",
                "params": {
                    "name": "exports.write",
                    "arguments": { "workspace": workspace.root.display().to_string(), "apply": true }
                }
            }))
            .expect("exports write");
        let write: Value =
            serde_json::from_str(response_text(&exports_write)).expect("parse write payload");
        assert!(write["applied"].as_bool().expect("applied flag"));
        assert!(workspace.exports_dir.join("references.ris").exists());

        let reference_resource = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 26,
                "method": "resources/read",
                "params": { "uri": "sourceright://reports/reference-integrity" }
            }))
            .expect("resource read");
        assert!(resource_text(&reference_resource).contains("reference-integrity"));

        let review_queue_resource = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 27,
                "method": "resources/read",
                "params": { "uri": "sourceright://workspaces/local/review-queue" }
            }))
            .expect("review queue resource");
        assert!(resource_text(&review_queue_resource).trim().is_empty());

        let plugins_resource = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 28,
                "method": "resources/read",
                "params": { "uri": "sourceright://plugins/registry" }
            }))
            .expect("plugins resource");
        assert!(resource_text(&plugins_resource).contains("provider.crossref"));

        let prompt = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 29,
                "method": "prompts/get",
                "params": {
                    "name": "provider_conflict_explanation",
                    "arguments": { "reference_id": "smith-2024" }
                }
            }))
            .expect("prompt response");
        assert!(prompt_text(&prompt).contains("provider/canonical conflicts"));
    }
    #[test]
    fn initialize_advertises_read_only_capabilities() {
        let mut runtime = McpRuntime::new(PathBuf::from(".sourceright"));
        let response = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "initialize",
                "params": {
                    "protocolVersion": "2025-11-25",
                    "capabilities": {},
                    "clientInfo": { "name": "test", "version": env!("CARGO_PKG_VERSION") }
                }
            }))
            .expect("response");

        assert_eq!(response["id"], 1);
        assert_eq!(response["result"]["protocolVersion"], "2025-11-25");
        assert_eq!(response["result"]["serverInfo"]["name"], SERVER_NAME);
        assert_eq!(response["result"]["capabilities"]["tools"], json!({}));
    }

    #[test]
    fn tools_list_matches_checked_in_manifest_count() {
        let tools = tools_list();
        assert_eq!(tools.as_array().map(|items| items.len()), Some(14));
        assert_eq!(tools[0]["annotations"]["readOnlyHint"], true);
        assert_eq!(tools[10]["annotations"]["readOnlyHint"], true);
        assert_eq!(tools[11]["annotations"]["readOnlyHint"], false);
    }

    #[test]
    fn checked_in_mcp_tools_manifest_tracks_journal_screening_platform_schema() {
        let runtime_tools = tools_list();
        let checked_in: Value =
            serde_json::from_str(include_str!("../mcp/tools.v1.json")).expect("tools manifest");
        let runtime_tool = runtime_tools
            .as_array()
            .expect("runtime tools")
            .iter()
            .find(|tool| tool["name"] == "journal.screen_submission")
            .expect("runtime journal tool");
        let manifest_tool = checked_in["tools"]
            .as_array()
            .expect("manifest tools")
            .iter()
            .find(|tool| tool["name"] == "journal.screen_submission")
            .expect("manifest journal tool");

        assert_eq!(manifest_tool["read_only"], true);
        assert_eq!(
            manifest_tool["input"]["platform_enum"],
            runtime_tool["inputSchema"]["properties"]["platform"]["enum"]
        );
        assert!(manifest_tool["input"].get("submission_id").is_some());
        assert!(manifest_tool["input"].get("manuscript_label").is_some());
        assert!(
            runtime_tool["inputSchema"]["properties"]["platform"]["description"]
                .as_str()
                .expect("platform description")
                .contains("screening contract label")
        );
    }

    #[test]
    fn resources_list_matches_checked_in_manifest_count() {
        let resources = resources_list();
        assert_eq!(resources.as_array().map(|items| items.len()), Some(8));
        assert_eq!(
            resources[0]["uri"],
            "sourceright://reports/reference-integrity"
        );
        assert_eq!(resources[7]["uri"], "sourceright://plugins/registry");
    }

    #[test]
    fn prompts_list_matches_checked_in_manifest_count() {
        let prompts = prompts_list();
        assert_eq!(prompts.as_array().map(|items| items.len()), Some(5));
        assert_eq!(prompts[0]["name"], "manual_reference_review");
    }

    #[test]
    fn server_card_matches_runtime_surface_counts() {
        let card = server_card();
        assert_eq!(card["serverInfo"]["name"], SERVER_NAME);
        assert_eq!(card["serverInfo"]["version"], env!("CARGO_PKG_VERSION"));
        assert_eq!(card["authentication"]["required"], false);
        assert_eq!(card["tools"].as_array().map(|items| items.len()), Some(14));
        assert_eq!(
            card["resources"].as_array().map(|items| items.len()),
            Some(8)
        );
        assert_eq!(card["prompts"].as_array().map(|items| items.len()), Some(5));
        assert_eq!(card["tools"], tools_list());
        assert_eq!(card["resources"], resources_list());
        assert_eq!(card["prompts"], prompts_list());
    }

    #[test]
    fn checked_in_server_card_tracks_runtime_surface() {
        let runtime_card = server_card();
        let checked_in: Value = serde_json::from_str(include_str!("../mcp/server-card.json"))
            .expect("checked-in MCP server card");
        assert_eq!(checked_in["serverInfo"], runtime_card["serverInfo"]);
        assert_eq!(checked_in["authentication"], runtime_card["authentication"]);
        assert_eq!(checked_in["tools"], runtime_card["tools"]);
        assert_eq!(checked_in["resources"], runtime_card["resources"]);
        assert_eq!(checked_in["prompts"], runtime_card["prompts"]);
    }

    #[test]
    fn status_payload_reports_stdio_server_mode() {
        let status = status_payload();
        assert_eq!(status["server_mode"], "stdio");
        assert_eq!(status["transport"], "stdio");
        assert_eq!(status["available_tools"], 14);
        assert_eq!(status["available_resources"], 8);
        assert_eq!(status["available_prompts"], 5);
        assert!(
            status["implemented_read_only_surfaces"]
                .as_array()
                .expect("read-only surfaces")
                .iter()
                .all(|surface| !surface.as_str().expect("surface").contains("export --all"))
        );
        assert!(
            status["implemented_read_only_surfaces"]
                .as_array()
                .expect("read-only surfaces")
                .iter()
                .all(|surface| !surface
                    .as_str()
                    .expect("surface")
                    .contains("import-decisions"))
        );
        assert!(
            status["implemented_apply_gated_write_surfaces"]
                .as_array()
                .expect("apply-gated surfaces")
                .iter()
                .any(|surface| surface.as_str().expect("surface").contains("export --all"))
        );
        assert!(
            status["implemented_apply_gated_write_surfaces"]
                .as_array()
                .expect("apply-gated surfaces")
                .iter()
                .any(|surface| surface
                    .as_str()
                    .expect("surface")
                    .contains("import-decisions"))
        );
    }

    #[test]
    fn tool_call_returns_reference_report_json() {
        let workspace_root = tempfile::tempdir().expect("workspace");
        let workspace = SourcerightWorkspace::from_root(workspace_root.path());
        workspace.init().expect("init workspace");
        fs::write(
            workspace_root.path().join("references.csl.json"),
            r#"[{"id":"smith-2024","type":"article-journal","title":"Example title"}]"#,
        )
        .expect("write csl");
        fs::write(
            workspace_root.path().join("references.verification.json"),
            r#"{"schema_version":"sourceright.verification.v1","references":{"smith-2024":{"review_status":"not_required"}}}"#,
        )
        .expect("write sidecar");
        let mut runtime = McpRuntime {
            workspace,
            initialized: true,
        };

        let response = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 2,
                "method": "tools/call",
                "params": {
                    "name": "references.report",
                    "arguments": { "workspace": workspace_root.path().display().to_string() }
                }
            }))
            .expect("response");

        assert_eq!(response["result"]["isError"], false);
        assert!(
            response["result"]["content"][0]["text"]
                .as_str()
                .expect("json text")
                .contains("smith-2024")
        );
    }

    #[test]
    fn tool_call_returns_plugin_registry_json() {
        let mut runtime = McpRuntime::new(PathBuf::from("."));
        runtime.initialized = true;

        let response = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 7,
                "method": "tools/call",
                "params": {
                    "name": "plugins.list",
                    "arguments": {}
                }
            }))
            .expect("response");

        let text = response["result"]["content"][0]["text"]
            .as_str()
            .expect("json text");

        assert!(text.contains("provider.crossref"));
        assert!(text.contains("sourceright.plugin_registry_report.v1"));
    }

    #[test]
    fn workspace_init_defaults_to_dry_run() {
        let tempdir = tempfile::tempdir().expect("workspace");
        let workspace = SourcerightWorkspace::from_root(tempdir.path().join(".sourceright"));
        let mut runtime = McpRuntime {
            workspace: workspace.clone(),
            initialized: true,
        };

        let response = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 5,
                "method": "tools/call",
                "params": {
                    "name": "workspace.init",
                    "arguments": {
                        "workspace": workspace.root.display().to_string()
                    }
                }
            }))
            .expect("response");

        let text = response["result"]["content"][0]["text"]
            .as_str()
            .expect("write plan text");
        let plan: Value = serde_json::from_str(text).expect("write plan JSON");

        assert_eq!(plan["schema_version"], MCP_WRITE_PLAN_SCHEMA_VERSION);
        assert_eq!(plan["tool"], "workspace.init");
        assert_eq!(plan["apply_requested"], false);
        assert_eq!(plan["applied"], false);
        assert!(plan.get("audit_log").is_none());
        assert!(!workspace.root.exists());
    }

    #[test]
    fn workspace_init_apply_writes_and_audits() {
        let tempdir = tempfile::tempdir().expect("workspace");
        let workspace = SourcerightWorkspace::from_root(tempdir.path().join(".sourceright"));
        let mut runtime = McpRuntime {
            workspace: workspace.clone(),
            initialized: true,
        };

        let response = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 6,
                "method": "tools/call",
                "params": {
                    "name": "workspace.init",
                    "arguments": {
                        "workspace": workspace.root.display().to_string(),
                        "apply": true
                    }
                }
            }))
            .expect("response");

        let text = response["result"]["content"][0]["text"]
            .as_str()
            .expect("write result text");
        let result: Value = serde_json::from_str(text).expect("write result JSON");

        assert_eq!(result["applied"], true);
        assert_eq!(result["apply_requested"], true);
        assert!(workspace.references_csl_json.exists());
        assert!(workspace.verification_sidecar_json.exists());
        assert!(workspace.review_queue_jsonl.exists());
        assert!(workspace.root.join(MCP_AUDIT_LOG_NAME).exists());
    }

    #[test]
    fn workspace_init_uses_explicit_workspace_path_literally() {
        let tempdir = tempfile::tempdir().expect("workspace");
        let explicit_root = tempdir.path().join("fresh-workspace");
        fs::create_dir_all(&explicit_root).expect("create workspace root");
        let expected_workspace = SourcerightWorkspace::from_root(&explicit_root);
        let mut runtime = McpRuntime {
            workspace: SourcerightWorkspace::from_root(tempdir.path().join("unused")),
            initialized: true,
        };

        let response = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 46,
                "method": "tools/call",
                "params": {
                    "name": "workspace.init",
                    "arguments": {
                        "workspace": explicit_root.display().to_string(),
                        "apply": true
                    }
                }
            }))
            .expect("response");
        let result: Value =
            serde_json::from_str(response_text(&response)).expect("write result JSON");

        assert_eq!(result["applied"], true);
        assert!(expected_workspace.references_csl_json.exists());
        assert!(expected_workspace.verification_sidecar_json.exists());
        assert!(expected_workspace.review_queue_jsonl.exists());
        assert!(!explicit_root.join(".sourceright").exists());
    }

    #[test]
    fn arxiv_journal_platform_labels_round_trip_through_mcp_tools() {
        for (fixture_name, platform_alias) in [
            ("submit-ce", "arxiv-submit-ce"),
            ("submission-core", "arxiv_submission_core"),
        ] {
            let tempdir = tempfile::tempdir().expect("workspace");
            let workspace = SourcerightWorkspace::from_root(tempdir.path().join(".sourceright"));
            let fixture = arxiv_fixture(fixture_name);
            write_arxiv_fixture_workspace(&workspace, &fixture);

            let mut runtime = McpRuntime {
                workspace: workspace.clone(),
                initialized: true,
            };
            let response = runtime
                .handle_message(json!({
                    "jsonrpc": "2.0",
                    "id": 44,
                    "method": "tools/call",
                    "params": {
                        "name": "journal.screen_submission",
                        "arguments": {
                            "workspace": workspace.root.display().to_string(),
                            "platform": platform_alias,
                            "submission_id": fixture["submission"]["submission_id"],
                            "manuscript_label": fixture["submission"]["manuscript_label"]
                        }
                    }
                }))
                .expect("journal response");
            let report: Value =
                serde_json::from_str(response_text(&response)).expect("journal report JSON");
            let expected = &fixture["expected_screening_report"];

            assert_eq!(report["schema_version"], expected["schema_version"]);
            assert_eq!(report["submission_id"], expected["submission_id"]);
            assert_eq!(report["platform"], expected["platform"]);
            assert_eq!(report["status"], expected["status"]);
        }
    }

    #[test]
    fn journal_screening_resource_decodes_encoded_workspace_paths() {
        let tempdir = tempfile::tempdir().expect("workspace");
        let fixture = arxiv_fixture("submit-ce");
        let encoded_parent = tempdir.path().join("workspace with spaces");
        let workspace = SourcerightWorkspace::from_root(encoded_parent.join(".sourceright"));
        write_arxiv_fixture_workspace(&workspace, &fixture);

        let encoded_workspace = percent_encode_query(&workspace.root.display().to_string());
        let mut runtime = McpRuntime {
            workspace: SourcerightWorkspace::from_root(tempdir.path().join("unused")),
            initialized: true,
        };
        let response = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 45,
                "method": "resources/read",
                "params": {
                    "uri": format!(
                        "sourceright://reports/journal-screening?workspace={encoded_workspace}&platform=arxiv-submit-ce&submission_id=ARXIV-CE-2026-0001&manuscript_label=source-package.tar.gz"
                    )
                }
            }))
            .expect("journal resource response");
        let report: Value =
            serde_json::from_str(resource_text(&response)).expect("journal resource JSON");

        assert_eq!(report["submission_id"], "ARXIV-CE-2026-0001");
        assert_eq!(report["platform"], "arxiv_submit_ce");
        assert_eq!(report["status"], "screened_with_warnings");
    }

    #[test]
    fn resource_read_derives_review_queue_without_writing() {
        let workspace_root = tempfile::tempdir().expect("workspace");
        let workspace = SourcerightWorkspace::from_root(workspace_root.path());
        workspace.init().expect("init workspace");
        fs::write(
            workspace_root.path().join("references.verification.json"),
            r#"{"schema_version":"sourceright.verification.v1","references":{"alpha":{"review_status":"queued","extraction":{"source":"input.docx","original_text":"Alpha","span":"paragraph:1"}}}}"#,
        )
        .expect("write sidecar");
        let runtime = McpRuntime {
            workspace,
            initialized: true,
        };
        let mut runtime = runtime;

        let response = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 3,
                "method": "resources/read",
                "params": {
                    "uri": "sourceright://workspaces/local/review-queue"
                }
            }))
            .expect("response");

        assert_eq!(
            response["result"]["contents"][0]["mimeType"],
            "application/jsonl"
        );
        assert!(
            response["result"]["contents"][0]["text"]
                .as_str()
                .expect("jsonl")
                .contains("alpha")
        );
    }

    #[test]
    fn resource_read_returns_plugin_registry_report() {
        let mut runtime = McpRuntime::new(PathBuf::from("."));
        runtime.initialized = true;

        let response = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 8,
                "method": "resources/read",
                "params": {
                    "uri": "sourceright://plugins/registry"
                }
            }))
            .expect("response");

        let text = response["result"]["contents"][0]["text"]
            .as_str()
            .expect("json text");

        assert!(text.contains("provider.crossref"));
        assert!(text.contains("citation-manager.zotero"));
    }

    #[test]
    fn prompt_get_returns_prompt_body() {
        let mut runtime = McpRuntime::new(PathBuf::from(".sourceright"));
        runtime.initialized = true;

        let response = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 4,
                "method": "prompts/get",
                "params": {
                    "name": "provider_conflict_explanation",
                    "arguments": { "reference_id": "smith-2024" }
                }
            }))
            .expect("response");

        assert!(
            response["result"]["messages"][0]["content"]["text"]
                .as_str()
                .expect("prompt text")
                .contains("provider/canonical conflicts")
        );
    }
}
