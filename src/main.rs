use std::collections::VecDeque;
use std::path::PathBuf;

use sourceright::SourcerightWorkspace;

fn main() {
    if let Err(error) = run(std::env::args().skip(1)) {
        eprintln!("{error}");
        std::process::exit(2);
    }
}

fn run(args: impl Iterator<Item = String>) -> Result<(), String> {
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

            let path = required_arg(
                "validate-csl",
                args.pop_front(),
                "path to references.csl.json",
            )?;
            reject_extra_args("validate-csl", &args)?;

            let diagnostics =
                SourcerightWorkspace::validate_csl_file(path).map_err(|error| error.to_string())?;
            if diagnostics.is_empty() {
                println!("valid");
            } else {
                for diagnostic in diagnostics {
                    println!("{diagnostic}");
                }
                return Err("CSL validation failed".to_string());
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
                return Err(format!(
                    "unexpected argument for `mcp`: {arg}\nrun `sourceright mcp --help` for usage"
                ));
            }
            None => {
                println!("{MCP_STATUS}");
                return Err(
                    "MCP server mode is not implemented; no MCP server was started".to_string(),
                );
            }
        },
        Some(command) => {
            return Err(format!(
                "unknown command: {command}\nrun `sourceright --help` for available commands"
            ));
        }
    }

    Ok(())
}

fn maybe_print_command_help(
    command: &str,
    args: &mut VecDeque<String>,
    help: &str,
) -> Result<bool, String> {
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

fn required_arg(command: &str, value: Option<String>, label: &str) -> Result<String, String> {
    value.ok_or_else(|| {
        format!("{command} requires {label}\nrun `sourceright {command} --help` for usage")
    })
}

fn reject_extra_args(command: &str, args: &VecDeque<String>) -> Result<(), String> {
    if let Some(extra) = args.front() {
        return Err(format!(
            "unexpected argument for `{command}`: {extra}\nrun `sourceright {command} --help` for usage"
        ));
    }

    Ok(())
}

fn print_help() {
    println!("{HELP}");
}

const HELP: &str = "sourceright

Reference verification infrastructure for academic and legal citation workflows.

Usage:
  sourceright --help
  sourceright --version
  sourceright init [document-or-directory]
  sourceright validate-csl <references.csl.json>
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
  sourceright validate-csl <references.csl.json>

Output:
  Prints `valid` when no diagnostics are found.
  Prints stable diagnostic lines as `<code> <path> <message>` when validation fails.";

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
        let error = required_arg("validate-csl", None, "path").expect_err("missing path");

        assert!(error.contains("validate-csl requires path"));
        assert!(error.contains("sourceright validate-csl --help"));
    }

    #[test]
    fn extra_arguments_are_rejected_with_usage_hint() {
        let args = VecDeque::from(vec!["one".to_string()]);
        let error = reject_extra_args("report", &args).expect_err("unexpected argument");

        assert!(error.contains("unexpected argument for `report`: one"));
        assert!(error.contains("sourceright report --help"));
    }

    #[test]
    fn mcp_status_is_explicitly_not_a_server() {
        assert!(MCP_STATUS.contains("server_mode: not-implemented"));
        assert!(MCP_STATUS.contains("server_started: false"));
        assert!(MCP_STATUS.contains("available_tools: 0"));
    }
}
