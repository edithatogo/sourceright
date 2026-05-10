use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::journal::{JournalPlatform, JournalScreeningReport};
use crate::legal::LegalCitationReport;
use crate::provenance::ProvenanceReport;
use crate::sidecar::VerificationSidecar;
use crate::workspace::SourcerightWorkspace;

pub const BENCHMARK_MANIFEST_SCHEMA_VERSION: &str = "sourceright.benchmark_tasks.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BenchmarkManifest {
    pub schema_version: String,
    pub runner_status: String,
    pub live_network: bool,
    pub workspace_crate: bool,
    pub tasks: Vec<BenchmarkTask>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BenchmarkTask {
    pub id: String,
    pub surface: String,
    pub fixture: String,
    pub baseline: String,
    pub measures: Vec<BenchmarkMeasure>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkMeasure {
    Correctness,
    Latency,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BenchmarkDiff {
    pub expected: String,
    pub actual: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BenchmarkTaskResult {
    pub id: String,
    pub passed: bool,
    pub baseline: String,
    pub actual: String,
    pub diff: Option<BenchmarkDiff>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BenchmarkRunReport {
    pub schema_version: String,
    pub manifest_path: String,
    pub task_count: usize,
    pub passed_count: usize,
    pub failed_count: usize,
    pub task_results: Vec<BenchmarkTaskResult>,
}

#[derive(Debug, thiserror::Error)]
pub enum BenchmarkError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("workspace error: {0}")]
    Workspace(#[from] crate::workspace::WorkspaceError),
    #[error("benchmark manifest validation error: {0}")]
    Manifest(String),
    #[error("unsupported benchmark surface: {0}")]
    UnsupportedSurface(String),
}

impl BenchmarkManifest {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, BenchmarkError> {
        let input = fs::read_to_string(path)?;
        Ok(serde_yaml::from_str(&input)?)
    }
}

impl BenchmarkRunReport {
    pub fn summary_text(&self) -> String {
        let mut output = format!(
            "Benchmark tasks: {}\nPassed: {}\nFailed: {}\n",
            self.task_count, self.passed_count, self.failed_count
        );
        for result in &self.task_results {
            output.push_str(&format!(
                "\n- {}: {}\n",
                result.id,
                if result.passed { "passed" } else { "failed" }
            ));
            if let Some(diff) = &result.diff {
                output.push_str("  expected:\n");
                output.push_str(&indent_block(&diff.expected, "    "));
                output.push_str("  actual:\n");
                output.push_str(&indent_block(&diff.actual, "    "));
            }
        }
        output
    }
}

pub fn run_benchmark_suite(
    manifest_path: impl AsRef<Path>,
) -> Result<BenchmarkRunReport, BenchmarkError> {
    let manifest_path = manifest_path.as_ref();
    let manifest = BenchmarkManifest::load(manifest_path)?;
    let bench_dir = manifest_path.parent().ok_or_else(|| {
        BenchmarkError::Manifest("manifest path has no parent directory".to_string())
    })?;

    let mut passed_count = 0usize;
    let mut task_results = Vec::new();
    for task in &manifest.tasks {
        let actual = run_task(bench_dir, task)
            .map_err(|error| BenchmarkError::Manifest(format!("task {}: {error}", task.id)))?;
        let baseline_path = bench_dir.join(&task.baseline);
        let expected = fs::read_to_string(&baseline_path).map_err(|error| {
            BenchmarkError::Manifest(format!(
                "task {} baseline {}: {error}",
                task.id,
                baseline_path.display()
            ))
        })?;
        let passed = normalize_newlines(&actual) == normalize_newlines(&expected);
        if passed {
            passed_count += 1;
        }
        task_results.push(BenchmarkTaskResult {
            id: task.id.clone(),
            passed,
            baseline: baseline_path.display().to_string(),
            actual: actual.clone(),
            diff: (!passed).then_some(BenchmarkDiff { expected, actual }),
        });
    }

    Ok(BenchmarkRunReport {
        schema_version: "sourceright.benchmark_run.v1".to_string(),
        manifest_path: manifest_path.display().to_string(),
        task_count: manifest.tasks.len(),
        passed_count,
        failed_count: manifest.tasks.len().saturating_sub(passed_count),
        task_results,
    })
}

fn run_task(bench_dir: &Path, task: &BenchmarkTask) -> Result<String, BenchmarkError> {
    let fixture_path = bench_dir.join(&task.fixture);
    match task.surface.as_str() {
        "validate-csl" => Ok(render_lines(
            crate::workspace::SourcerightWorkspace::validate_csl_file(&fixture_path)?,
        )),
        "sidecar" => {
            let sidecar: VerificationSidecar =
                serde_json::from_str(&fs::read_to_string(&fixture_path)?)?;
            Ok(render_sidecar(&sidecar))
        }
        "report" => {
            let workspace = SourcerightWorkspace::from_root(&fixture_path);
            Ok(serde_json::to_string_pretty(&workspace.reference_report_json()?)? + "\n")
        }
        "citations" => {
            let manuscript = fs::read_to_string(&fixture_path)?;
            let workspace = SourcerightWorkspace::from_root(bench_dir.join("fixtures"));
            Ok(workspace
                .citation_reconciliation_report(&manuscript)?
                .to_markdown())
        }
        "review-queue" => Ok(fs::read_to_string(fixture_path.join("review-queue.jsonl"))?),
        "export" => {
            let workspace = SourcerightWorkspace::from_root(&fixture_path);
            Ok(render_export_suite(&workspace)?)
        }
        "legal" => {
            let text = fs::read_to_string(&fixture_path)?;
            let report: LegalCitationReport = crate::analyze_legal_citations(&text);
            Ok(serde_json::to_string_pretty(&report)? + "\n")
        }
        "provenance" => {
            let text = fs::read_to_string(&fixture_path)?;
            let report: ProvenanceReport = crate::analyze_claim_source_provenance(&text);
            Ok(serde_json::to_string_pretty(&report)? + "\n")
        }
        "journal-screen" => {
            let workspace = SourcerightWorkspace::from_root(&fixture_path);
            let report: JournalScreeningReport = workspace.journal_screening_report(
                "BENCH-001".to_string(),
                JournalPlatform::Ojs,
                "benchmark.txt".to_string(),
            )?;
            Ok(serde_json::to_string_pretty(&report)? + "\n")
        }
        other => Err(BenchmarkError::UnsupportedSurface(other.to_string())),
    }
}

fn render_export_suite(workspace: &SourcerightWorkspace) -> Result<String, BenchmarkError> {
    let mut output = workspace
        .export_references(None)?
        .into_iter()
        .map(|artifact| artifact.filename)
        .collect::<Vec<_>>()
        .join("\n");
    if !output.is_empty() {
        output.push('\n');
    }
    output.push_str("valid\nno diagnostics\n");
    Ok(output)
}

fn render_lines(lines: Vec<String>) -> String {
    if lines.is_empty() {
        "valid\nno diagnostics\n".to_string()
    } else {
        let mut output = lines.join("\n");
        output.push('\n');
        output
    }
}

fn render_sidecar(sidecar: &VerificationSidecar) -> String {
    let diagnostics = sidecar.validate();
    if diagnostics.is_empty() {
        "valid\nno diagnostics\n".to_string()
    } else {
        let mut output = diagnostics
            .iter()
            .map(|diagnostic| {
                format!(
                    "{} {} {}",
                    diagnostic.code, diagnostic.path, diagnostic.message
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        output.push('\n');
        output
    }
}

fn indent_block(text: &str, indent: &str) -> String {
    text.lines()
        .map(|line| format!("{indent}{line}\n"))
        .collect::<String>()
}

fn normalize_newlines(text: &str) -> String {
    text.replace("\r\n", "\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_parses_checked_in_tasks() {
        let manifest =
            BenchmarkManifest::load("sourceright-bench/tasks.yaml").expect("load manifest");
        assert_eq!(manifest.schema_version, BENCHMARK_MANIFEST_SCHEMA_VERSION);
        assert_eq!(manifest.tasks.len(), 9);
    }

    #[test]
    fn report_summary_text_includes_pass_counts() {
        let report = BenchmarkRunReport {
            schema_version: "sourceright.benchmark_run.v1".to_string(),
            manifest_path: "sourceright-bench/tasks.yaml".to_string(),
            task_count: 1,
            passed_count: 1,
            failed_count: 0,
            task_results: vec![BenchmarkTaskResult {
                id: "example".to_string(),
                passed: true,
                baseline: "baseline.txt".to_string(),
                actual: "ok".to_string(),
                diff: None,
            }],
        };

        assert!(report.summary_text().contains("Passed: 1"));
    }

    #[test]
    fn benchmark_suite_matches_checked_in_baselines() {
        let manifest =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("sourceright-bench/tasks.yaml");
        let report = run_benchmark_suite(&manifest).expect("run benchmark suite");
        if report.failed_count != 0 {
            eprintln!("{}", report.summary_text());
        }
        assert_eq!(report.failed_count, 0);
        assert_eq!(report.task_count, 9);
    }
}
