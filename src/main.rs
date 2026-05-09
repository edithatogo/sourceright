use std::path::PathBuf;

use sourceright::SourcerightWorkspace;

fn main() {
    if let Err(error) = run(std::env::args().skip(1)) {
        eprintln!("{error}");
        std::process::exit(2);
    }
}

fn run(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    match args.next().as_deref() {
        Some("--version") | Some("-V") => println!("sourceright {}", env!("CARGO_PKG_VERSION")),
        Some("--help") | Some("-h") | None => print_help(),
        Some("init") => {
            let target = args
                .next()
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("."));
            let workspace = SourcerightWorkspace::for_document_or_dir(target);
            workspace.init().map_err(|error| error.to_string())?;
            println!("{}", workspace.root.display());
        }
        Some("validate-csl") => {
            let Some(path) = args.next() else {
                return Err("validate-csl requires a path to references.csl.json".to_string());
            };
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
        Some("mcp") => {
            eprintln!("MCP server mode is planned but not implemented yet.");
            return Err("MCP server mode is not implemented".to_string());
        }
        Some(command) => {
            return Err(format!(
                "unknown command: {command}\nrun `sourceright --help` for available commands"
            ));
        }
    }

    Ok(())
}

fn print_help() {
    println!(
        "sourceright\n\nUsage:\n  sourceright --help\n  sourceright --version\n  sourceright init [document-or-directory]\n  sourceright validate-csl <references.csl.json>\n  sourceright mcp\n\nSourceright is reference verification infrastructure."
    );
}
