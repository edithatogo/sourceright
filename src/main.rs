use std::collections::VecDeque;
use std::path::PathBuf;

use serde::Serialize;
use sourceright::SourcerightWorkspace;

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

            let workspace_root = args
                .pop_front()
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(".sourceright"));
            reject_extra_args("report", &args)?;

            let workspace = SourcerightWorkspace::from_root(workspace_root);
            let report = workspace
                .reference_report_markdown()
                .map_err(|error| error.to_string())?;
            println!("{report}");
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
  sourceright report [.sourceright-directory]
  sourceright mcp [status|--status]

Commands:
  init          Create or confirm a local .sourceright workspace.
  validate-csl  Validate canonical CSL JSON and print deterministic diagnostics.
  report        Print a reference integrity report from a .sourceright workspace.
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
  sourceright report [.sourceright-directory]

Default:
  Uses `.sourceright` when no directory is supplied.";

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
available_tools: 0
available_resources: 0
available_prompts: 0
planned_first_increment: read-only local CSL validation and reference reporting
recommended_today:
  - sourceright validate-csl <references.csl.json>
  - sourceright report [.sourceright-directory]
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
        assert!(MCP_STATUS.contains("available_tools: 0"));
    }
}
