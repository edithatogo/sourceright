use std::collections::VecDeque;
use std::fs;
use std::path::PathBuf;

use serde::Serialize;
use sourceright::{ExportFormat, JournalPlatform, ReviewDecisionImport, SourcerightWorkspace};

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
        Some("export") => {
            if maybe_print_command_help("export", &mut args, EXPORT_HELP)? {
                return Ok(());
            }

            let options = parse_export_args(args)?;
            let workspace = SourcerightWorkspace::from_root(options.workspace_root);
            let paths = workspace
                .write_exports(options.format)
                .map_err(|error| error.to_string())?;
            for path in paths {
                println!("{}", path.display());
            }
        }
        Some("mcp") => match args.pop_front().as_deref() {
            Some("--help") | Some("-h") => {
                reject_extra_args("mcp", &args)?;
                println!("{MCP_HELP}");
            }
            Some("status") | Some("--status") => {
                reject_extra_args("mcp status", &args)?;
                println!("{MCP_STATUS}");
            }
            Some(arg) => {
                return Err(CliError::usage(format!(
                    "unexpected argument for `mcp`: {arg}\nrun `sourceright mcp --help` for usage"
                )));
            }
            None => {
                println!("{MCP_STATUS}");
                return Err(CliError::usage(
                    "MCP server mode is not implemented; no MCP server was started",
                ));
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

    if args.front().is_some_and(|arg| arg == "--format") {
        args.pop_front();
        let value = required_arg("export", args.pop_front(), "format name")?;
        format = Some(ExportFormat::parse(&value).ok_or_else(|| {
            CliError::usage(format!(
                "unsupported export format: {value}\nrun `sourceright export --help` for usage"
            ))
        })?);
    } else if args.front().is_some_and(|arg| arg == "--all") {
        args.pop_front();
    } else {
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
  sourceright export [--all|--format <format>] [.sourceright-directory]
  sourceright mcp [status|--status]

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
  export        Write clean reference exports from canonical CSL JSON.
  mcp           Show MCP implementation status; server mode is not implemented yet.

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

const EXPORT_HELP: &str = "sourceright export

Write clean reference exports from an existing workspace.

Usage:
  sourceright export [--all|--format <format>] [.sourceright-directory]

Formats:
  yaml, xml, ris, enw, biblatex

Behavior:
  `--format <format>` writes one explicitly requested format.
  `--all` writes the full export suite.
  No export files are written unless a format or `--all` is requested.";

const MCP_HELP: &str = "sourceright mcp

Report MCP server implementation status.

Usage:
  sourceright mcp
  sourceright mcp status
  sourceright mcp --status

Behavior:
  `sourceright mcp` prints the placeholder status and exits non-zero because no
  MCP server is started.
  `sourceright mcp status` prints the same status and exits successfully.";

const MCP_STATUS: &str = "Sourceright MCP status
server_mode: not-implemented
transport: none
server_started: false
available_tools: 8
available_resources: 6
available_prompts: 0
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
  - sourceright export --all [.sourceright-directory]
resource_uris:
  - sourceright://reports/reference-integrity
  - sourceright://reports/citation-reconciliation
  - sourceright://workspaces/local/review-queue
  - sourceright://reports/journal-screening
  - sourceright://reports/legal-citations
  - sourceright://reports/claim-source-provenance
message: MCP server mode is planned but not implemented yet.";

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
        assert_eq!(one.workspace_root, PathBuf::from(".sourceright"));
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
    fn mcp_status_is_explicitly_not_a_server() {
        assert!(MCP_STATUS.contains("server_mode: not-implemented"));
        assert!(MCP_STATUS.contains("server_started: false"));
        assert!(MCP_STATUS.contains("available_tools: 8"));
        assert!(MCP_STATUS.contains("sourceright://reports/reference-integrity"));
    }
}
