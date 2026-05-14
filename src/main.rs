#![forbid(unsafe_code)]

use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;

use serde::Serialize;
use sourceright::{
    CitationSyncConfig, ExportFormat, JournalPlatform, ReviewDecisionImport, SourcerightPolicy,
    SourcerightWorkspace, discover_plugins, evaluate_policy, parse_csl_json, run_benchmark_suite,
    run_citation_sync,
};

mod mcp;

fn main() {
    if let Err(error) = run(std::env::args().skip(1)) {
        eprintln!("{error}");
        std::process::exit(error.exit_code());
    }
}

fn run(args: impl Iterator<Item = String>) -> Result<(), CliError> {
    let mut args: VecDeque<String> = args.collect();

    match args.pop_front().as_deref() {
        Some("--version") | Some("-V") => println!("sourceright {}", env!("CARGO_PKG_VERSION")),
        Some("--help") | Some("-h") | None => print_help(),
        Some("init") => {
            if maybe_print_command_help("init", &mut args, INIT_HELP)? {
                return Ok(());
            }

            let target = args
                .pop_front()
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("."));
            reject_extra_args("init", &args)?;

            let workspace = SourcerightWorkspace::for_document_or_dir(target);
            workspace.init().map_err(|error| error.to_string())?;
            println!("{}", workspace.root.display());
        }
        Some("validate-csl") => {
            if maybe_print_command_help("validate-csl", &mut args, VALIDATE_CSL_HELP)? {
                return Ok(());
            }

            let options = parse_validate_csl_args(args)?;

            let diagnostics = SourcerightWorkspace::validate_csl_file(&options.path)
                .map_err(|error| error.to_string())?;
            let output = ValidateCslOutput::new(&options.path, &diagnostics);

            if options.json {
                println!("{}", output.to_json()?);
            }

            if diagnostics.is_empty() {
                if !options.json {
                    println!("valid");
                }
            } else {
                if !options.json {
                    for diagnostic in &diagnostics {
                        println!("{diagnostic}");
                    }
                }
                return Err(CliError::validation_failed("CSL validation failed"));
            }
        }
        Some("report") => {
            if maybe_print_command_help("report", &mut args, REPORT_HELP)? {
                return Ok(());
            }

            let options = parse_report_args(args)?;

            let workspace = SourcerightWorkspace::from_root(options.workspace_root);
            match options.format {
                ReportFormat::Markdown => {
                    let report = workspace
                        .reference_report_markdown()
                        .map_err(|error| error.to_string())?;
                    println!("{report}");
                }
                ReportFormat::Json => {
                    let report = workspace
                        .reference_report_json()
                        .map_err(|error| error.to_string())?;
                    println!("{}", serde_json::to_string(&report)?);
                }
                ReportFormat::McpResource => {
                    let resource = workspace
                        .reference_report_mcp_resource()
                        .map_err(|error| error.to_string())?;
                    println!("{}", serde_json::to_string(&resource)?);
                }
            }
        }
        Some("conflicts") => {
            if maybe_print_command_help("conflicts", &mut args, CONFLICTS_HELP)? {
                return Ok(());
            }

            let workspace_root = args
                .pop_front()
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(".sourceright"));
            reject_extra_args("conflicts", &args)?;

            let report = SourcerightWorkspace::from_root(workspace_root)
                .conflict_resolution_report()
                .map_err(|error| error.to_string())?;
            println!("{}", report.to_markdown());
        }
        Some("citations") => {
            if maybe_print_command_help("citations", &mut args, CITATIONS_HELP)? {
                return Ok(());
            }

            let manuscript = required_arg("citations", args.pop_front(), "manuscript text path")?;
            let workspace_root = args
                .pop_front()
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(".sourceright"));
            reject_extra_args("citations", &args)?;

            let text = fs::read_to_string(manuscript).map_err(|error| error.to_string())?;
            let report = SourcerightWorkspace::from_root(workspace_root)
                .citation_reconciliation_report(&text)
                .map_err(|error| error.to_string())?;
            println!("{}", report.to_markdown());
        }
        Some("review") => match args.pop_front().as_deref() {
            Some("--help") | Some("-h") => {
                reject_extra_args("review", &args)?;
                println!("{REVIEW_HELP}");
            }
            Some("queue") => {
                let workspace_root = args
                    .pop_front()
                    .map(PathBuf::from)
                    .unwrap_or_else(|| PathBuf::from(".sourceright"));
                reject_extra_args("review queue", &args)?;
                let workspace = SourcerightWorkspace::from_root(workspace_root);
                workspace
                    .refresh_review_queue()
                    .map_err(|error| error.to_string())?;
                let jsonl = fs::read_to_string(&workspace.review_queue_jsonl)
                    .map_err(|error| error.to_string())?;
                print!("{jsonl}");
            }
            Some("partitions") => {
                let options = parse_review_partitions_args(args)?;
                let partitions = SourcerightWorkspace::from_root(options.workspace_root)
                    .review_queue_partitions(options.max_entries)
                    .map_err(|error| error.to_string())?;
                println!("{}", serde_json::to_string(&partitions)?);
            }
            Some("import-decisions") => {
                let decisions_path = required_arg(
                    "review import-decisions",
                    args.pop_front(),
                    "decisions JSON path",
                )?;
                let workspace_root = args
                    .pop_front()
                    .map(PathBuf::from)
                    .unwrap_or_else(|| PathBuf::from(".sourceright"));
                reject_extra_args("review import-decisions", &args)?;
                let decisions_json =
                    fs::read_to_string(decisions_path).map_err(|error| error.to_string())?;
                let decisions: Vec<ReviewDecisionImport> = serde_json::from_str(&decisions_json)?;
                let report = SourcerightWorkspace::from_root(workspace_root)
                    .import_review_decisions(&decisions)
                    .map_err(|error| error.to_string())?;
                println!("{}", serde_json::to_string(&report)?);
            }
            Some(arg) => {
                return Err(CliError::usage(format!(
                    "unexpected argument for `review`: {arg}\nrun `sourceright review --help` for usage"
                )));
            }
            None => {
                return Err(CliError::usage(
                    "review requires `queue`, `partitions`, or `import-decisions`\nrun `sourceright review --help` for usage",
                ));
            }
        },
        Some("journal-screen") => {
            if maybe_print_command_help("journal-screen", &mut args, JOURNAL_SCREEN_HELP)? {
                return Ok(());
            }

            let options = parse_journal_screen_args(args)?;
            let report = SourcerightWorkspace::from_root(options.workspace_root)
                .journal_screening_report(
                    options.submission_id,
                    options.platform,
                    options.manuscript_label,
                )
                .map_err(|error| error.to_string())?;
            println!("{}", serde_json::to_string(&report)?);
        }
        Some("legal") => {
            if maybe_print_command_help("legal", &mut args, LEGAL_HELP)? {
                return Ok(());
            }

            let path = required_arg("legal", args.pop_front(), "text path")?;
            reject_extra_args("legal", &args)?;
            let text = fs::read_to_string(path).map_err(|error| error.to_string())?;
            let report = sourceright::analyze_legal_citations(&text);
            println!("{}", serde_json::to_string(&report)?);
        }
        Some("provenance") => {
            if maybe_print_command_help("provenance", &mut args, PROVENANCE_HELP)? {
                return Ok(());
            }

            let path = required_arg("provenance", args.pop_front(), "text path")?;
            reject_extra_args("provenance", &args)?;
            let text = fs::read_to_string(path).map_err(|error| error.to_string())?;
            let report = sourceright::analyze_claim_source_provenance(&text);
            println!("{}", serde_json::to_string(&report)?);
        }
        Some("policy") => {
            if maybe_print_command_help("policy", &mut args, POLICY_HELP)? {
                return Ok(());
            }

            let options = parse_policy_args(args)?;
            let csl_json = fs::read_to_string(options.references_csl_json)
                .map_err(|error| error.to_string())?;
            let document = parse_csl_json(&csl_json)?;
            let policy = if let Some(policy_path) = options.policy_json {
                let policy_json =
                    fs::read_to_string(policy_path).map_err(|error| error.to_string())?;
                serde_json::from_str(&policy_json)?
            } else {
                SourcerightPolicy::journal_vancouver()
            };
            let report = evaluate_policy(&document, &policy);
            println!("{}", serde_json::to_string(&report)?);
        }
        Some("export") => {
            if maybe_print_command_help("export", &mut args, EXPORT_HELP)? {
                return Ok(());
            }

            let options = parse_export_args(args)?;
            let workspace = SourcerightWorkspace::from_root(options.workspace_root);
            if options.preview {
                let manifest = workspace
                    .export_manifest(options.format)
                    .map_err(|error| error.to_string())?;
                println!("{}", serde_json::to_string(&manifest)?);
            } else {
                let paths = workspace
                    .write_exports(options.format)
                    .map_err(|error| error.to_string())?;
                for path in paths {
                    println!("{}", path.display());
                }
            }
        }
        Some("plugins") => {
            if maybe_print_command_help("plugins", &mut args, PLUGINS_HELP)? {
                return Ok(());
            }

            let validate = if args.front().is_some_and(|arg| arg == "validate") {
                args.pop_front();
                true
            } else {
                false
            };
            let json = if args.front().is_some_and(|arg| arg == "--json") {
                args.pop_front();
                true
            } else {
                false
            };

            reject_extra_args("plugins", &args)?;
            let report = discover_plugins().map_err(|error| error.to_string())?;

            if json {
                println!("{}", serde_json::to_string(&report)?);
            } else {
                println!("{}", report.summary_text());
            }

            if validate && !report.is_valid() {
                return Err(CliError::validation_failed(
                    "plugin registry validation failed",
                ));
            }
        }
        Some("bench") => {
            if maybe_print_command_help("bench", &mut args, BENCH_HELP)? {
                return Ok(());
            }

            let options = parse_bench_args(args)?;
            let report =
                run_benchmark_suite(&options.manifest_path).map_err(|error| error.to_string())?;
            if options.json {
                println!("{}", serde_json::to_string(&report)?);
            } else {
                print!("{}", report.summary_text());
            }
            if report.failed_count > 0 {
                return Err(CliError::validation_failed("benchmark suite failed"));
            }
        }
        Some("citation-sync") => {
            if maybe_print_command_help("citation-sync", &mut args, CITATION_SYNC_HELP)? {
                return Ok(());
            }

            let options = parse_citation_sync_args(args)?;
            let workspace = SourcerightWorkspace::from_root(options.workspace_root);
            let report =
                run_citation_sync(&workspace, options.config).map_err(|error| error.to_string())?;
            println!("{}", serde_json::to_string_pretty(&report)?);
            if report.conflict_count > 0 {
                return Err(CliError::validation_failed(
                    "citation sync reported conflicts",
                ));
            }
        }
        Some("mcp") => match args.pop_front().as_deref() {
            Some("--help") | Some("-h") => {
                reject_extra_args("mcp", &args)?;
                println!("{MCP_HELP}");
            }
            Some("tools") => {
                print_mcp_manifest("mcp tools", args, MCP_TOOLS_MANIFEST)?;
            }
            Some("resources") => {
                print_mcp_manifest("mcp resources", args, MCP_RESOURCES_MANIFEST)?;
            }
            Some("prompts") => {
                print_mcp_manifest("mcp prompts", args, MCP_PROMPTS_MANIFEST)?;
            }
            Some("status") | Some("--status") => {
                print_mcp_status(args)?;
            }
            Some("--json") => {
                reject_extra_args("mcp --json", &args)?;
                println!("{}", serde_json::to_string(&McpStatusOutput::current())?);
            }
            Some(arg) => {
                return Err(CliError::usage(format!(
                    "unexpected argument for `mcp`: {arg}\nrun `sourceright mcp --help` for usage"
                )));
            }
            None => {
                reject_extra_args("mcp", &args)?;
                mcp::serve_stdio().map_err(|error| error.to_string())?;
            }
        },
        Some(command) => {
            return Err(CliError::usage(format!(
                "unknown command: {command}\nrun `sourceright --help` for available commands"
            )));
        }
    }

    Ok(())
}

fn print_mcp_status(mut args: VecDeque<String>) -> Result<(), CliError> {
    if args.front().is_some_and(|arg| arg == "--json") {
        args.pop_front();
        reject_extra_args("mcp status", &args)?;
        println!("{}", serde_json::to_string(&McpStatusOutput::current())?);
    } else {
        reject_extra_args("mcp status", &args)?;
        println!("{MCP_STATUS}");
    }
    Ok(())
}

fn print_mcp_manifest(
    command: &str,
    mut args: VecDeque<String>,
    manifest: &str,
) -> Result<(), CliError> {
    if args.front().is_some_and(|arg| arg == "--json") {
        args.pop_front();
    }
    reject_extra_args(command, &args)?;

    let json: serde_json::Value = serde_json::from_str(manifest)?;
    println!("{}", serde_json::to_string(&json)?);
    Ok(())
}

fn maybe_print_command_help(
    command: &str,
    args: &mut VecDeque<String>,
    help: &str,
) -> Result<bool, CliError> {
    let Some(first) = args.front() else {
        return Ok(false);
    };

    if first == "--help" || first == "-h" {
        args.pop_front();
        reject_extra_args(command, args)?;
        println!("{help}");
        return Ok(true);
    }

    Ok(false)
}

fn parse_validate_csl_args(args: VecDeque<String>) -> Result<ValidateCslOptions, CliError> {
    let mut json = false;
    let mut path = None;

    for arg in args {
        if arg == "--json" {
            json = true;
        } else if path.is_none() {
            path = Some(PathBuf::from(arg));
        } else {
            return Err(CliError::usage(format!(
                "unexpected argument for `validate-csl`: {arg}\nrun `sourceright validate-csl --help` for usage"
            )));
        }
    }

    let path = required_arg("validate-csl", path, "path to references.csl.json")?;
    Ok(ValidateCslOptions { path, json })
}

fn parse_report_args(args: VecDeque<String>) -> Result<ReportOptions, CliError> {
    let mut format = ReportFormat::Markdown;
    let mut workspace_root = None;

    for arg in args {
        match arg.as_str() {
            "--json" => format = ReportFormat::Json,
            "--mcp-resource" => format = ReportFormat::McpResource,
            _ if workspace_root.is_none() => workspace_root = Some(PathBuf::from(arg)),
            _ => {
                return Err(CliError::usage(format!(
                    "unexpected argument for `report`: {arg}\nrun `sourceright report --help` for usage"
                )));
            }
        }
    }

    Ok(ReportOptions {
        workspace_root: workspace_root.unwrap_or_else(|| PathBuf::from(".sourceright")),
        format,
    })
}

fn parse_export_args(mut args: VecDeque<String>) -> Result<ExportOptions, CliError> {
    let mut format = None;
    let mut selected = false;
    let mut preview = false;

    while let Some(arg) = args.front() {
        match arg.as_str() {
            "--preview" => {
                preview = true;
                args.pop_front();
            }
            "--format" => {
                if selected {
                    return Err(CliError::usage(
                        "export accepts only one of `--format <format>` or `--all`\nrun `sourceright export --help` for usage",
                    ));
                }
                args.pop_front();
                let value = required_arg("export", args.pop_front(), "format name")?;
                format = Some(ExportFormat::parse(&value).ok_or_else(|| {
                    CliError::usage(format!(
                        "unsupported export format: {value}\nrun `sourceright export --help` for usage"
                    ))
                })?);
                selected = true;
            }
            "--all" => {
                if selected {
                    return Err(CliError::usage(
                        "export accepts only one of `--format <format>` or `--all`\nrun `sourceright export --help` for usage",
                    ));
                }
                args.pop_front();
                selected = true;
            }
            _ => break,
        }
    }

    if !selected {
        return Err(CliError::usage(
            "export requires `--format <format>` or `--all`\nrun `sourceright export --help` for usage",
        ));
    }

    let workspace_root = args
        .pop_front()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(".sourceright"));
    reject_extra_args("export", &args)?;

    Ok(ExportOptions {
        workspace_root,
        format,
        preview,
    })
}

fn parse_review_partitions_args(
    mut args: VecDeque<String>,
) -> Result<ReviewPartitionsOptions, CliError> {
    let mut max_entries = 10;
    if args.front().is_some_and(|arg| arg == "--size") {
        args.pop_front();
        let value = required_arg("review partitions", args.pop_front(), "partition size")?;
        max_entries = value.parse::<usize>().map_err(|_| {
            CliError::usage(
                "review partitions requires --size to be a positive integer\nrun `sourceright review --help` for usage",
            )
        })?;
    }
    let workspace_root = args
        .pop_front()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(".sourceright"));
    reject_extra_args("review partitions", &args)?;
    Ok(ReviewPartitionsOptions {
        workspace_root,
        max_entries,
    })
}

fn parse_journal_screen_args(mut args: VecDeque<String>) -> Result<JournalScreenOptions, CliError> {
    let mut platform = JournalPlatform::GenericWebhook;
    let mut submission_id = "local-submission".to_string();
    let mut manuscript_label = "manuscript".to_string();

    while let Some(arg) = args.front() {
        match arg.as_str() {
            "--platform" => {
                args.pop_front();
                let value = required_arg("journal-screen", args.pop_front(), "platform")?;
                platform = parse_journal_platform(&value)?;
            }
            "--submission-id" => {
                args.pop_front();
                submission_id = required_arg("journal-screen", args.pop_front(), "submission id")?;
            }
            "--manuscript" => {
                args.pop_front();
                manuscript_label =
                    required_arg("journal-screen", args.pop_front(), "manuscript label")?;
            }
            _ => break,
        }
    }

    let workspace_root = args
        .pop_front()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(".sourceright"));
    reject_extra_args("journal-screen", &args)?;
    Ok(JournalScreenOptions {
        workspace_root,
        platform,
        submission_id,
        manuscript_label,
    })
}

fn parse_journal_platform(value: &str) -> Result<JournalPlatform, CliError> {
    match value {
        "generic-webhook" | "generic_webhook" => Ok(JournalPlatform::GenericWebhook),
        "ojs" => Ok(JournalPlatform::Ojs),
        "scholarone" => Ok(JournalPlatform::ScholarOne),
        "editorial-manager" | "editorial_manager" => Ok(JournalPlatform::EditorialManager),
        "ejournalpress" | "e-journal-press" => Ok(JournalPlatform::EJournalPress),
        "manuscript-manager" | "manuscript_manager" => Ok(JournalPlatform::ManuscriptManager),
        _ => Err(CliError::usage(format!(
            "unsupported journal platform: {value}\nrun `sourceright journal-screen --help` for usage"
        ))),
    }
}

fn parse_policy_args(mut args: VecDeque<String>) -> Result<PolicyOptions, CliError> {
    let mut policy_json = None;

    if args.front().is_some_and(|arg| arg == "--policy") {
        args.pop_front();
        policy_json = Some(required_arg(
            "policy",
            args.pop_front(),
            "policy JSON path",
        )?);
    }

    let references_csl_json = required_arg(
        "policy",
        args.pop_front().map(PathBuf::from),
        "path to references.csl.json",
    )?;
    reject_extra_args("policy", &args)?;

    Ok(PolicyOptions {
        references_csl_json,
        policy_json: policy_json.map(PathBuf::from),
    })
}

fn parse_bench_args(mut args: VecDeque<String>) -> Result<BenchOptions, CliError> {
    let mut json = false;
    let mut manifest_path = None;

    while let Some(arg) = args.pop_front() {
        match arg.as_str() {
            "--json" => json = true,
            "--manifest" => {
                manifest_path = Some(PathBuf::from(required_arg(
                    "bench",
                    args.pop_front(),
                    "benchmark manifest path",
                )?));
            }
            _ if arg.starts_with('-') => {
                return Err(CliError::usage(format!(
                    "unexpected argument for `bench`: {arg}\nrun `sourceright bench --help` for usage"
                )));
            }
            _ if manifest_path.is_none() => manifest_path = Some(PathBuf::from(arg)),
            _ => {
                return Err(CliError::usage(format!(
                    "unexpected argument for `bench`: {arg}\nrun `sourceright bench --help` for usage"
                )));
            }
        }
    }

    Ok(BenchOptions {
        manifest_path: manifest_path
            .unwrap_or_else(|| PathBuf::from("sourceright-bench/tasks.yaml")),
        json,
    })
}

fn parse_citation_sync_args(mut args: VecDeque<String>) -> Result<CitationSyncOptions, CliError> {
    let mut apply = false;
    let mut preview = false;
    let mut audit_log_path = None;
    let mut remote_fixture_path = None;
    let mut workspace_root = None;

    while let Some(arg) = args.pop_front() {
        match arg.as_str() {
            "--apply" => {
                if preview {
                    return Err(CliError::usage(
                        "citation-sync accepts only one of `--preview` or `--apply`\nrun `sourceright citation-sync --help` for usage",
                    ));
                }
                apply = true;
            }
            "--preview" => {
                if apply {
                    return Err(CliError::usage(
                        "citation-sync accepts only one of `--preview` or `--apply`\nrun `sourceright citation-sync --help` for usage",
                    ));
                }
                preview = true;
            }
            "--audit-log" => {
                audit_log_path = Some(PathBuf::from(required_arg(
                    "citation-sync",
                    args.pop_front(),
                    "audit log path",
                )?));
            }
            "--remote-fixture" => {
                remote_fixture_path = Some(PathBuf::from(required_arg(
                    "citation-sync",
                    args.pop_front(),
                    "remote fixture path",
                )?));
            }
            _ if arg.starts_with('-') => {
                return Err(CliError::usage(format!(
                    "unexpected argument for `citation-sync`: {arg}\nrun `sourceright citation-sync --help` for usage"
                )));
            }
            _ if workspace_root.is_none() => workspace_root = Some(PathBuf::from(arg)),
            _ => {
                return Err(CliError::usage(format!(
                    "unexpected argument for `citation-sync`: {arg}\nrun `sourceright citation-sync --help` for usage"
                )));
            }
        }
    }

    let apply = apply && !preview;
    Ok(CitationSyncOptions {
        workspace_root: workspace_root.unwrap_or_else(|| PathBuf::from(".sourceright")),
        config: CitationSyncConfig {
            preview: !apply,
            apply,
            audit_log_path,
            remote_fixture_path,
            zotero_api_url: std::env::var("SOURCERIGHT_ZOTERO_API_URL").ok(),
            zotero_api_key: std::env::var("SOURCERIGHT_ZOTERO_API_KEY").ok(),
            zotero_library_id: std::env::var("SOURCERIGHT_ZOTERO_LIBRARY_ID").ok(),
            zotero_library_type: std::env::var("SOURCERIGHT_ZOTERO_LIBRARY_TYPE").ok(),
        },
    })
}

fn required_arg<T>(command: &str, value: Option<T>, label: &str) -> Result<T, CliError> {
    value.ok_or_else(|| {
        CliError::usage(format!(
            "{command} requires {label}\nrun `sourceright {command} --help` for usage"
        ))
    })
}

fn reject_extra_args(command: &str, args: &VecDeque<String>) -> Result<(), CliError> {
    if let Some(extra) = args.front() {
        return Err(CliError::usage(format!(
            "unexpected argument for `{command}`: {extra}\nrun `sourceright {command} --help` for usage"
        )));
    }

    Ok(())
}

fn print_help() {
    println!("{HELP}");
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ValidateCslOptions {
    path: PathBuf,
    json: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ReportOptions {
    workspace_root: PathBuf,
    format: ReportFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReportFormat {
    Markdown,
    Json,
    McpResource,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ExportOptions {
    workspace_root: PathBuf,
    format: Option<ExportFormat>,
    preview: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ReviewPartitionsOptions {
    workspace_root: PathBuf,
    max_entries: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct JournalScreenOptions {
    workspace_root: PathBuf,
    platform: JournalPlatform,
    submission_id: String,
    manuscript_label: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PolicyOptions {
    references_csl_json: PathBuf,
    policy_json: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BenchOptions {
    manifest_path: PathBuf,
    json: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CitationSyncOptions {
    workspace_root: PathBuf,
    config: CitationSyncConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct ValidateCslOutput {
    ok: bool,
    path: String,
    diagnostics: Vec<ValidateCslDiagnostic>,
}

impl ValidateCslOutput {
    fn new(path: &std::path::Path, diagnostics: &[String]) -> Self {
        let diagnostics = diagnostics
            .iter()
            .map(|diagnostic| ValidateCslDiagnostic::from_line(diagnostic))
            .collect::<Vec<_>>();

        Self {
            ok: diagnostics.is_empty(),
            path: path.display().to_string(),
            diagnostics,
        }
    }

    fn to_json(&self) -> Result<String, CliError> {
        serde_json::to_string(self).map_err(|error| error.to_string().into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct McpStatusOutput {
    server_mode: &'static str,
    transport: &'static str,
    server_started: bool,
    available_tools: usize,
    available_resources: usize,
    available_prompts: usize,
    implemented_read_only_surfaces: Vec<&'static str>,
    resource_uris: Vec<&'static str>,
    message: &'static str,
}

impl McpStatusOutput {
    fn current() -> Self {
        let implemented_read_only_surfaces = vec![
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
        ];
        let resource_uris = vec![
            "sourceright://reports/reference-integrity",
            "sourceright://reports/citation-reconciliation",
            "sourceright://workspaces/local/review-queue",
            "sourceright://reports/journal-screening",
            "sourceright://reports/legal-citations",
            "sourceright://reports/claim-source-provenance",
            "sourceright://reports/policy",
            "sourceright://plugins/registry",
        ];

        Self {
            server_mode: "stdio",
            transport: "stdio",
            server_started: false,
            available_tools: 14,
            available_resources: 8,
            available_prompts: 5,
            implemented_read_only_surfaces,
            resource_uris,
            message: "MCP server mode is implemented; run `sourceright mcp` to start the stdio server.",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct ValidateCslDiagnostic {
    code: String,
    path: String,
    message: String,
}

impl ValidateCslDiagnostic {
    fn from_line(line: &str) -> Self {
        let mut parts = line.splitn(3, ' ');
        Self {
            code: parts.next().unwrap_or_default().to_string(),
            path: parts.next().unwrap_or_default().to_string(),
            message: parts.next().unwrap_or_default().to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CliError {
    message: String,
    exit_code: i32,
}

impl CliError {
    fn usage(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            exit_code: 2,
        }
    }

    fn validation_failed(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            exit_code: 1,
        }
    }

    fn exit_code(&self) -> i32 {
        self.exit_code
    }
}

impl From<String> for CliError {
    fn from(message: String) -> Self {
        Self::usage(message)
    }
}

impl From<&str> for CliError {
    fn from(message: &str) -> Self {
        Self::usage(message)
    }
}

impl From<serde_json::Error> for CliError {
    fn from(error: serde_json::Error) -> Self {
        Self::usage(error.to_string())
    }
}

impl std::fmt::Display for CliError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(formatter)
    }
}

const HELP: &str = "sourceright

Reference verification infrastructure for academic and legal citation workflows.

Usage:
  sourceright --help
  sourceright --version
  sourceright init [document-or-directory]
  sourceright validate-csl [--json] <references.csl.json>
  sourceright report [--json|--mcp-resource] [.sourceright-directory]
  sourceright conflicts [.sourceright-directory]
  sourceright citations <manuscript.txt> [.sourceright-directory]
  sourceright review queue|partitions|import-decisions ...
  sourceright journal-screen [options] [.sourceright-directory]
  sourceright legal <legal-text.txt>
  sourceright provenance <document-text.txt>
  sourceright policy [--policy <policy.json>] <references.csl.json>
  sourceright export [--all|--format <format>] [.sourceright-directory]
  sourceright plugins [validate] [--json]
  sourceright bench [--json] [--manifest <tasks.yaml>]
  sourceright citation-sync [--preview|--apply] [options] [.sourceright-directory]
  sourceright mcp
  sourceright mcp status|--status|--json
  sourceright mcp tools|resources|prompts [--json]

Commands:
  init          Create or confirm a local .sourceright workspace.
  validate-csl  Validate canonical CSL JSON and print deterministic diagnostics.
  report        Print a reference integrity report from a .sourceright workspace.
  conflicts     Explain deterministic provider merge and conflict decisions.
  citations     Reconcile in-text citations against canonical references.
  review        Inspect review queues, partition work, and import decisions.
  journal-screen  Produce a platform-neutral journal citation-screening report.
  legal         Extract and model legal citations separately from CSL.
  provenance    Build a claim/source provenance graph from document text.
  policy        Evaluate deterministic style and recency policy checks.
  export        Write clean reference exports from canonical CSL JSON.
  plugins       Discover and validate runtime plugin manifests.
  bench         Run deterministic fixture-backed benchmark tasks.
  citation-sync Preview or apply citation-manager sync plans.
  mcp           Start the local MCP server or inspect its readiness/status.

Run `sourceright <command> --help` for command-specific usage.";

const INIT_HELP: &str = "sourceright init

Create or confirm a local Sourceright workspace layout.

Usage:
  sourceright init [document-or-directory]

Output:
  Prints the workspace directory path on success.";

const VALIDATE_CSL_HELP: &str = "sourceright validate-csl

Validate canonical CSL JSON input.

Usage:
  sourceright validate-csl [--json] <references.csl.json>

Output:
  Prints `valid` when no diagnostics are found.
  Prints stable diagnostic lines as `<code> <path> <message>` when validation fails.
  With `--json`, prints `{\"ok\":bool,\"path\":string,\"diagnostics\":[...]}` to stdout.

Exit codes:
  0 when the CSL file is valid.
  1 when the CSL file is readable JSON but has validation diagnostics.
  2 for usage, I/O, and JSON parse errors.";

const REPORT_HELP: &str = "sourceright report

Print a reference integrity report from an existing workspace.

Usage:
  sourceright report [--json|--mcp-resource] [.sourceright-directory]

Default:
  Uses `.sourceright` when no directory is supplied.

Output:
  Markdown by default.
  `--json` prints compact `sourceright.reference_report.v1` JSON.
  `--mcp-resource` prints the MCP-ready JSON resource envelope.";

const CONFLICTS_HELP: &str = "sourceright conflicts

Explain deterministic provider merge and conflict decisions.

Usage:
  sourceright conflicts [.sourceright-directory]

Default:
  Uses `.sourceright` when no directory is supplied.

Behavior:
  Prints a Markdown conflict-resolution report.
  High-confidence provider values may fill missing canonical fields.
  Disagreements with existing canonical values are preserved as review conflicts.";

const CITATIONS_HELP: &str = "sourceright citations

Reconcile in-text citations against canonical references.

Usage:
  sourceright citations <manuscript.txt> [.sourceright-directory]

Output:
  Prints a Markdown citation reconciliation report.";

const REVIEW_HELP: &str = "sourceright review

Inspect review queues, partition work, and import decisions.

Usage:
  sourceright review queue [.sourceright-directory]
  sourceright review partitions [--size <n>] [.sourceright-directory]
  sourceright review import-decisions <decisions.json> [.sourceright-directory]

Decision import:
  Expects a JSON array of {reference_id, decision, reviewer, decided_at, status, notes?}.";

const JOURNAL_SCREEN_HELP: &str = "sourceright journal-screen

Produce a platform-neutral journal citation-screening report.

Usage:
  sourceright journal-screen [--platform <platform>] [--submission-id <id>] [--manuscript <label>] [.sourceright-directory]

Platforms:
  generic-webhook, ojs, scholarone, editorial-manager, ejournalpress, manuscript-manager";

const LEGAL_HELP: &str = "sourceright legal

Extract and model legal citations separately from academic CSL JSON.

Usage:
  sourceright legal <legal-text.txt>

Output:
  Prints compact JSON with legal citation records, jurisdiction/provider hints, and review issues.";

const PROVENANCE_HELP: &str = "sourceright provenance

Build a claim/source provenance graph from document text.

Usage:
  sourceright provenance <document-text.txt>

Output:
  Prints compact JSON with claims, detected citation source nodes, links, and provenance issues.";

const POLICY_HELP: &str = "sourceright policy

Evaluate deterministic style and recency policy checks over canonical CSL JSON.

Usage:
  sourceright policy [--policy <policy.json>] <references.csl.json>

Default:
  Uses the built-in journal-vancouver policy when no policy JSON is supplied.

Output:
  Prints compact `sourceright.policy_report.v1` JSON.";

const EXPORT_HELP: &str = "sourceright export

Write clean reference exports from an existing workspace.

Usage:
  sourceright export [--all|--format <format>] [.sourceright-directory]

Formats:
  yaml, xml, ris, enw, biblatex

Behavior:
  `--format <format>` selects one explicitly requested format.
  `--all` selects the full export suite.
  `--preview` prints compact `sourceright.export_manifest.v1` JSON without
  writing files.
  No export files are written unless a format or `--all` is requested.";

const PLUGINS_HELP: &str = "sourceright plugins

Discover and validate runtime plugin manifests from the repository registry.

Usage:
  sourceright plugins [validate] [--json]

Behavior:
  With no arguments, prints a human-readable discovery summary.
  `--json` prints compact `sourceright.plugin_registry_report.v1` JSON.
  `validate` exits with a non-zero status if any discovered manifest fails
  validation.";

const BENCH_HELP: &str = "sourceright bench

Run deterministic fixture-backed benchmark tasks.

Usage:
  sourceright bench [--json] [--manifest <tasks.yaml>]
  sourceright bench [--json] <tasks.yaml>

Default:
  Uses `sourceright-bench/tasks.yaml` when no manifest is supplied.

Output:
  Human-readable pass/fail summary by default.
  `--json` prints compact `sourceright.benchmark_run.v1` JSON.

Exit codes:
  0 when all benchmark tasks match their baselines.
  1 when any task differs from its checked-in baseline.
  2 for usage, I/O, manifest, or parse errors.";

const CITATION_SYNC_HELP: &str = "sourceright citation-sync

Preview or apply citation-manager sync plans.

Usage:
  sourceright citation-sync [--preview|--apply] [--remote-fixture <remote.json>] [--audit-log <audit.jsonl>] [.sourceright-directory]

Default:
  Uses `.sourceright` when no workspace directory is supplied.
  Runs in preview mode unless `--apply` is supplied.

Zotero live sync:
  Live transport is opt-in and reads SOURCERIGHT_ZOTERO_API_URL,
  SOURCERIGHT_ZOTERO_API_KEY, SOURCERIGHT_ZOTERO_LIBRARY_ID, and optional
  SOURCERIGHT_ZOTERO_LIBRARY_TYPE from the environment.

Output:
  Prints pretty `sourceright.citation_sync.v1` JSON.
  Conflicts are reported without silently overwriting CSL data.";

const MCP_HELP: &str = "sourceright mcp

Start the local MCP server or inspect its readiness.

Usage:
  sourceright mcp
  sourceright mcp status
  sourceright mcp --status
  sourceright mcp status --json
  sourceright mcp --json

Behavior:
  `sourceright mcp` starts the stdio MCP server and stays attached to it.
  `sourceright mcp status` prints the same readiness status and exits successfully.
  `sourceright mcp tools|resources|prompts --json` prints compact read-only
  manifest JSON for adapter development.
  The MCP server also exposes `plugins.list` and `sourceright://plugins/registry`
  for validated plugin discovery.
  `--json` prints a compact machine-readable readiness envelope.";

const MCP_STATUS: &str = "Sourceright MCP status
server_mode: stdio
transport: stdio
server_started: false
available_tools: 14
available_resources: 8
available_prompts: 5
implemented_read_only_surfaces:
  - sourceright validate-csl <references.csl.json>
  - sourceright report --json [.sourceright-directory]
  - sourceright report --mcp-resource [.sourceright-directory]
  - sourceright conflicts [.sourceright-directory]
  - sourceright citations <manuscript.txt> [.sourceright-directory]
  - sourceright review queue|partitions|import-decisions
  - sourceright journal-screen [.sourceright-directory]
  - sourceright legal <legal-text.txt>
  - sourceright provenance <document-text.txt>
  - sourceright policy <references.csl.json>
  - sourceright plugins [validate] [--json]
  - sourceright export --all [.sourceright-directory]
resource_uris:
  - sourceright://reports/reference-integrity
  - sourceright://reports/citation-reconciliation
  - sourceright://workspaces/local/review-queue
  - sourceright://reports/journal-screening
  - sourceright://reports/legal-citations
  - sourceright://reports/claim-source-provenance
  - sourceright://reports/policy
  - sourceright://plugins/registry
message: MCP server mode is implemented; run `sourceright mcp` to start the stdio server.";

const MCP_TOOLS_MANIFEST: &str = include_str!("../mcp/tools.v1.json");
const MCP_RESOURCES_MANIFEST: &str = include_str!("../mcp/resources.v1.json");
const MCP_PROMPTS_MANIFEST: &str = include_str!("../mcp/prompts.v1.json");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_required_argument_reports_command_help() {
        let error =
            required_arg("validate-csl", Option::<String>::None, "path").expect_err("missing path");
        let error = error.to_string();

        assert!(error.contains("validate-csl requires path"));
        assert!(error.contains("sourceright validate-csl --help"));
    }

    #[test]
    fn extra_arguments_are_rejected_with_usage_hint() {
        let args = VecDeque::from(vec!["one".to_string()]);
        let error = reject_extra_args("report", &args).expect_err("unexpected argument");
        let error = error.to_string();

        assert!(error.contains("unexpected argument for `report`: one"));
        assert!(error.contains("sourceright report --help"));
    }

    #[test]
    fn validate_csl_accepts_json_option_before_path() {
        let options = parse_validate_csl_args(VecDeque::from(vec![
            "--json".to_string(),
            "references.csl.json".to_string(),
        ]))
        .expect("parse validate-csl args");

        assert!(options.json);
        assert_eq!(options.path, PathBuf::from("references.csl.json"));
    }

    #[test]
    fn report_accepts_json_and_mcp_resource_formats() {
        let json = parse_report_args(VecDeque::from(vec![
            "--json".to_string(),
            ".sourceright".to_string(),
        ]))
        .expect("parse json report args");
        let resource = parse_report_args(VecDeque::from(vec![
            "--mcp-resource".to_string(),
            ".sourceright".to_string(),
        ]))
        .expect("parse mcp resource report args");

        assert_eq!(json.format, ReportFormat::Json);
        assert_eq!(resource.format, ReportFormat::McpResource);
        assert_eq!(json.workspace_root, PathBuf::from(".sourceright"));
    }

    #[test]
    fn conflicts_command_rejects_extra_arguments() {
        let args = VecDeque::from(vec!["one".to_string()]);
        let error = reject_extra_args("conflicts", &args).expect_err("unexpected argument");

        assert!(
            error
                .to_string()
                .contains("unexpected argument for `conflicts`")
        );
    }

    #[test]
    fn review_partitions_accepts_size_and_workspace() {
        let options = parse_review_partitions_args(VecDeque::from(vec![
            "--size".to_string(),
            "3".to_string(),
            ".sourceright".to_string(),
        ]))
        .expect("parse review partitions");

        assert_eq!(options.max_entries, 3);
        assert_eq!(options.workspace_root, PathBuf::from(".sourceright"));
    }

    #[test]
    fn journal_screen_options_parse_platform_and_submission_metadata() {
        let options = parse_journal_screen_args(VecDeque::from(vec![
            "--platform".to_string(),
            "ojs".to_string(),
            "--submission-id".to_string(),
            "SUB-1".to_string(),
            "--manuscript".to_string(),
            "manuscript.docx".to_string(),
            ".sourceright".to_string(),
        ]))
        .expect("parse journal screen args");

        assert_eq!(options.platform, JournalPlatform::Ojs);
        assert_eq!(options.submission_id, "SUB-1");
        assert_eq!(options.manuscript_label, "manuscript.docx");
    }

    #[test]
    fn legal_and_provenance_commands_require_input_paths() {
        let legal = required_arg("legal", Option::<String>::None, "text path")
            .expect_err("legal path required")
            .to_string();
        let provenance = required_arg("provenance", Option::<String>::None, "text path")
            .expect_err("provenance path required")
            .to_string();

        assert!(legal.contains("legal requires text path"));
        assert!(provenance.contains("provenance requires text path"));
    }

    #[test]
    fn policy_accepts_optional_json_policy_and_csl_path() {
        let options = parse_policy_args(VecDeque::from(vec![
            "--policy".to_string(),
            "policy.json".to_string(),
            "references.csl.json".to_string(),
        ]))
        .expect("parse policy args");

        assert_eq!(options.policy_json, Some(PathBuf::from("policy.json")));
        assert_eq!(
            options.references_csl_json,
            PathBuf::from("references.csl.json")
        );
    }

    #[test]
    fn bench_accepts_manifest_path_and_json_output() {
        let options = parse_bench_args(VecDeque::from(vec![
            "--json".to_string(),
            "--manifest".to_string(),
            "sourceright-bench/tasks.yaml".to_string(),
        ]))
        .expect("parse bench args");

        assert!(options.json);
        assert_eq!(
            options.manifest_path,
            PathBuf::from("sourceright-bench/tasks.yaml")
        );
    }

    #[test]
    fn citation_sync_defaults_to_preview_workspace_and_accepts_apply() {
        let default = parse_citation_sync_args(VecDeque::new()).expect("parse default sync");
        let apply = parse_citation_sync_args(VecDeque::from(vec![
            "--apply".to_string(),
            "--audit-log".to_string(),
            "audit.jsonl".to_string(),
            "--remote-fixture".to_string(),
            "remote.json".to_string(),
            ".sourceright".to_string(),
        ]))
        .expect("parse apply sync");

        assert_eq!(default.workspace_root, PathBuf::from(".sourceright"));
        assert!(default.config.preview);
        assert!(!default.config.apply);
        assert!(apply.config.apply);
        assert!(!apply.config.preview);
        assert_eq!(
            apply.config.audit_log_path,
            Some(PathBuf::from("audit.jsonl"))
        );
        assert_eq!(
            apply.config.remote_fixture_path,
            Some(PathBuf::from("remote.json"))
        );
    }

    #[test]
    fn export_accepts_single_format_or_full_suite() {
        let one = parse_export_args(VecDeque::from(vec![
            "--format".to_string(),
            "ris".to_string(),
            ".sourceright".to_string(),
        ]))
        .expect("parse single export");
        let all = parse_export_args(VecDeque::from(vec![
            "--all".to_string(),
            ".sourceright".to_string(),
        ]))
        .expect("parse full export");

        assert_eq!(one.format, Some(ExportFormat::Ris));
        assert_eq!(all.format, None);
        assert!(!one.preview);
        assert_eq!(one.workspace_root, PathBuf::from(".sourceright"));
    }

    #[test]
    fn export_preview_selects_artifacts_without_write_mode() {
        let options = parse_export_args(VecDeque::from(vec![
            "--preview".to_string(),
            "--format".to_string(),
            "ris".to_string(),
            ".sourceright".to_string(),
        ]))
        .expect("parse preview export");

        assert!(options.preview);
        assert_eq!(options.format, Some(ExportFormat::Ris));
        assert_eq!(options.workspace_root, PathBuf::from(".sourceright"));
    }

    #[test]
    fn export_requires_explicit_format_or_all() {
        let error = parse_export_args(VecDeque::from(vec![".sourceright".to_string()]))
            .expect_err("export should require explicit output selection")
            .to_string();

        assert!(error.contains("export requires `--format <format>` or `--all`"));
        assert!(error.contains("sourceright export --help"));
    }

    #[test]
    fn validate_csl_json_output_is_stable_machine_readable() {
        let output = ValidateCslOutput::new(
            std::path::Path::new("references.csl.json"),
            &["csl.title.empty $[0].title CSL item title must not be empty".to_string()],
        );

        let json = output.to_json().expect("serialize JSON output");

        assert_eq!(
            json,
            r#"{"ok":false,"path":"references.csl.json","diagnostics":[{"code":"csl.title.empty","path":"$[0].title","message":"CSL item title must not be empty"}]}"#
        );
    }

    #[test]
    fn validation_errors_use_exit_code_one() {
        let error = CliError::validation_failed("CSL validation failed");

        assert_eq!(error.exit_code(), 1);
    }

    #[test]
    fn mcp_status_reports_stdio_server_mode() {
        assert!(MCP_STATUS.contains("server_mode: stdio"));
        assert!(MCP_STATUS.contains("transport: stdio"));
        assert!(MCP_STATUS.contains("server_started: false"));
        assert!(MCP_STATUS.contains("available_tools: 14"));
        assert!(MCP_STATUS.contains("available_resources: 8"));
        assert!(MCP_STATUS.contains("available_prompts: 5"));
        assert!(MCP_STATUS.contains("sourceright://reports/reference-integrity"));
        assert!(MCP_STATUS.contains("sourceright://plugins/registry"));

        let json = serde_json::to_value(McpStatusOutput::current()).expect("serialize status");
        assert_eq!(json["server_mode"], "stdio");
        assert_eq!(json["transport"], "stdio");
        assert_eq!(json["server_started"], false);
        assert_eq!(json["available_tools"], 14);
        assert_eq!(json["available_resources"], 8);
        assert_eq!(json["available_prompts"], 5);
    }

    #[test]
    fn mcp_manifests_are_valid_json() {
        let tools: serde_json::Value =
            serde_json::from_str(MCP_TOOLS_MANIFEST).expect("tools manifest is valid JSON");
        let resources: serde_json::Value =
            serde_json::from_str(MCP_RESOURCES_MANIFEST).expect("resources manifest is valid JSON");
        let prompts: serde_json::Value =
            serde_json::from_str(MCP_PROMPTS_MANIFEST).expect("prompts manifest is valid JSON");

        assert_eq!(tools["schema_version"], "sourceright.mcp_tools.v1");
        assert_eq!(resources["schema_version"], "sourceright.mcp_resources.v1");
        assert_eq!(prompts["schema_version"], "sourceright.mcp_prompts.v1");
    }

    #[test]
    fn mcp_manifest_commands_accept_optional_json_flag() {
        print_mcp_manifest(
            "mcp tools",
            VecDeque::from(vec!["--json".to_string()]),
            MCP_TOOLS_MANIFEST,
        )
        .expect("print tools manifest");

        let error = print_mcp_manifest(
            "mcp tools",
            VecDeque::from(vec!["--yaml".to_string()]),
            MCP_TOOLS_MANIFEST,
        )
        .expect_err("reject unsupported manifest flag");

        assert!(
            error
                .to_string()
                .contains("unexpected argument for `mcp tools`")
        );
    }
}
