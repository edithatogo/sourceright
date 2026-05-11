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
            workspace: SourcerightWorkspace::from_root(workspace_root),
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
            "instructions": "Read-only local reference verification server",
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
                let workspace = workspace_from_arguments(&self.workspace, arguments);
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
                let workspace = workspace_from_arguments(&self.workspace, arguments);
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
                let workspace = workspace_from_arguments(&self.workspace, arguments);
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
            "sourceright review queue|partitions|import-decisions",
            "sourceright journal-screen [.sourceright-directory]",
            "sourceright legal <legal-text.txt>",
            "sourceright provenance <document-text.txt>",
            "sourceright policy <references.csl.json>",
            "sourceright plugins [validate] [--json]",
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
                    "platform": { "type": "string" },
                    "submission_id": { "type": "string" },
                    "manuscript_label": { "type": "string" }
                },
                "additionalProperties": false
            }),
        ),
        tool_definition(
            "legal.analyze_citations",
            "Extract legal citations into the separate legal model.",
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
            "Review separate legal citation records and jurisdiction/provider issues.",
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
            "Review separate legal citation records and jurisdiction/provider issues.",
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
            "Review separate legal citation records and jurisdiction/provider issues."
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
        "audit_log": workspace.root.join(MCP_AUDIT_LOG_NAME).display().to_string(),
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
        .map(SourcerightWorkspace::from_root)
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
            let key = pair.next().unwrap_or_default().to_string();
            let value = pair.next().unwrap_or_default().to_string();
            query.insert(key, value);
        }
    }

    (base, query)
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
            "Smith (2024) cites the benchmark reference.",
        )
        .expect("write manuscript");
        fs::write(
            tempdir.path().join("legal.txt"),
            "Smith v Jones 2024 NSWSC 1.",
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
        assert!(response_text(&status).contains("server_mode: stdio"));

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
        assert!(response_text(&citations).contains("Smith"));

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
        assert!(response_text(&legal).contains("legal_citations"));

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
            status: crate::sidecar::ReviewStatus::Resolved,
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
        assert!(response_text(&reference_resource).contains("reference-integrity"));

        let review_queue_resource = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 27,
                "method": "resources/read",
                "params": { "uri": "sourceright://workspaces/local/review-queue" }
            }))
            .expect("review queue resource");
        assert!(response_text(&review_queue_resource).contains("smith-2024"));

        let plugins_resource = runtime
            .handle_message(json!({
                "jsonrpc": "2.0",
                "id": 28,
                "method": "resources/read",
                "params": { "uri": "sourceright://plugins/registry" }
            }))
            .expect("plugins resource");
        assert!(response_text(&plugins_resource).contains("provider.crossref"));

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
        assert!(response_text(&prompt).contains("provider/canonical conflicts"));
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
    fn status_payload_reports_stdio_server_mode() {
        let status = status_payload();
        assert_eq!(status["server_mode"], "stdio");
        assert_eq!(status["transport"], "stdio");
        assert_eq!(status["available_tools"], 14);
        assert_eq!(status["available_resources"], 8);
        assert_eq!(status["available_prompts"], 5);
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
