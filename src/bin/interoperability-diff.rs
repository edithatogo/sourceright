use std::{env, fs, process::ExitCode};

use sourceright::compare_csl_json;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 || args.len() > 5 {
        eprintln!(
            "usage: interoperability-diff <canonical.json> <oracle.json> [source] [markdown-output]"
        );
        return ExitCode::from(2);
    }
    let canonical = match read_json(&args[1]) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("canonical input: {error}");
            return ExitCode::from(2);
        }
    };
    let oracle = match read_json(&args[2]) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("oracle input: {error}");
            return ExitCode::from(2);
        }
    };
    let source = args
        .get(3)
        .cloned()
        .unwrap_or_else(|| "unspecified".to_string());
    let report = compare_csl_json(&canonical, &oracle, source);
    if let Some(path) = args.get(4)
        && let Err(error) = fs::write(path, report.markdown())
    {
        eprintln!("markdown output: {error}");
        return ExitCode::from(2);
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&report).expect("report serializes")
    );
    if report.is_equivalent() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

fn read_json(path: &str) -> Result<serde_json::Value, String> {
    let text = fs::read_to_string(path).map_err(|error| error.to_string())?;
    serde_json::from_str(&text).map_err(|error| error.to_string())
}
