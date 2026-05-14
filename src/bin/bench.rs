#![forbid(unsafe_code)]

use std::path::PathBuf;

fn main() {
    let mut json = false;
    let mut manifest = None;
    let mut args = std::env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => {
                println!(
                    "sourceright bench\n\nUsage:\n  bench [--json] [--manifest <tasks.yaml>]\n  bench [--json] <tasks.yaml>"
                );
                return;
            }
            "--json" => json = true,
            "--manifest" => {
                manifest = Some(PathBuf::from(args.next().unwrap_or_else(|| {
                    eprintln!("bench requires benchmark manifest path");
                    std::process::exit(2);
                })));
            }
            _ if arg.starts_with('-') => {
                eprintln!("unexpected argument for `bench`: {arg}");
                std::process::exit(2);
            }
            _ if manifest.is_none() => manifest = Some(PathBuf::from(arg)),
            _ => {
                eprintln!("unexpected argument for `bench`: {arg}");
                std::process::exit(2);
            }
        }
    }

    let manifest = manifest.unwrap_or_else(|| PathBuf::from("sourceright-bench/tasks.yaml"));

    match sourceright::run_benchmark_suite(&manifest) {
        Ok(report) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string(&report).expect("serialize benchmark report")
                );
            } else {
                print!("{}", report.summary_text());
            }
            if report.failed_count > 0 {
                std::process::exit(1);
            }
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
