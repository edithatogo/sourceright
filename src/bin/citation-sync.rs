#![forbid(unsafe_code)]

use std::path::PathBuf;

fn main() {
    let mut apply = false;
    let mut preview = false;
    let mut audit_log_path = None;
    let mut remote_fixture_path = None;
    let mut workspace_root = None;
    let mut args = std::env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => {
                println!(
                    "sourceright citation-sync\n\nUsage:\n  citation-sync [--preview|--apply] [--remote-fixture <remote.json>] [--audit-log <audit.jsonl>] [.sourceright-directory]"
                );
                return;
            }
            "--apply" => {
                if preview {
                    eprintln!("citation-sync accepts only one of `--preview` or `--apply`");
                    std::process::exit(2);
                }
                apply = true;
            }
            "--preview" => {
                if apply {
                    eprintln!("citation-sync accepts only one of `--preview` or `--apply`");
                    std::process::exit(2);
                }
                preview = true;
            }
            "--audit-log" => {
                audit_log_path = Some(PathBuf::from(args.next().unwrap_or_else(|| {
                    eprintln!("citation-sync requires audit log path");
                    std::process::exit(2);
                })));
            }
            "--remote-fixture" => {
                remote_fixture_path = Some(PathBuf::from(args.next().unwrap_or_else(|| {
                    eprintln!("citation-sync requires remote fixture path");
                    std::process::exit(2);
                })));
            }
            _ if arg.starts_with('-') => {
                eprintln!("unexpected argument for `citation-sync`: {arg}");
                std::process::exit(2);
            }
            _ if workspace_root.is_none() => workspace_root = Some(PathBuf::from(arg)),
            _ => {
                eprintln!("unexpected argument for `citation-sync`: {arg}");
                std::process::exit(2);
            }
        }
    }

    let apply = apply && !preview;
    let workspace_root = workspace_root.unwrap_or_else(|| PathBuf::from(".sourceright"));

    let workspace = if apply {
        sourceright::SourcerightWorkspace::from_root(workspace_root)
    } else {
        sourceright::SourcerightWorkspace::from_root_or_parent(workspace_root)
    };
    let config = sourceright::CitationSyncConfig {
        preview: !apply,
        apply,
        audit_log_path,
        remote_fixture_path,
        zotero_api_url: std::env::var("SOURCERIGHT_ZOTERO_API_URL").ok(),
        zotero_api_key: std::env::var("SOURCERIGHT_ZOTERO_API_KEY").ok(),
        zotero_library_id: std::env::var("SOURCERIGHT_ZOTERO_LIBRARY_ID").ok(),
        zotero_library_type: std::env::var("SOURCERIGHT_ZOTERO_LIBRARY_TYPE").ok(),
    };

    match sourceright::run_citation_sync(&workspace, config) {
        Ok(report) => {
            println!(
                "{}",
                serde_json::to_string_pretty(&report).expect("serialize report")
            );
            if report.conflict_count > 0 {
                std::process::exit(1);
            }
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
