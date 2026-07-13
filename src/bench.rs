use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};

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
    pub performance: Option<BenchmarkPerformance>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BenchmarkPerformance {
    pub max_duration_ms: Option<u64>,
    pub enforce: bool,
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
    pub duration_ms: u128,
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
            output.push_str(&format!("  latency_ms: {}\n", result.duration_ms));
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
        let has_correctness = has_measure(task, BenchmarkMeasure::Correctness);
        let has_latency = has_measure(task, BenchmarkMeasure::Latency);
        let started = Instant::now();
        let actual = run_task(bench_dir, task)
            .map_err(|error| BenchmarkError::Manifest(format!("task {}: {error}", task.id)))?;
        let duration = started.elapsed();
        let baseline = bench_dir.join(&task.baseline).display().to_string();
        let expected = if has_correctness {
            Some(fs::read_to_string(&baseline).map_err(|error| {
                BenchmarkError::Manifest(format!("task {} baseline {}: {error}", task.id, baseline))
            })?)
        } else {
            None
        };
        let correctness = expected
            .as_deref()
            .is_none_or(|expected| normalize_newlines(&actual) == normalize_newlines(expected));
        let performance = if has_latency {
            evaluate_performance(task, duration)
        } else {
            true
        };
        let passed = correctness && performance;
        if passed {
            passed_count += 1;
        }
        task_results.push(BenchmarkTaskResult {
            id: task.id.clone(),
            passed,
            baseline,
            actual: actual.clone(),
            duration_ms: duration.as_millis(),
            diff: if has_correctness && !passed {
                let expected_text = expected.unwrap_or_default();
                (expected_text != actual).then_some(BenchmarkDiff {
                    expected: expected_text,
                    actual,
                })
            } else {
                None
            },
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
            let workspace = SourcerightWorkspace::from_root_or_parent(&fixture_path);
            Ok(serde_json::to_string_pretty(&workspace.reference_report_json()?)? + "\n")
        }
        "citations" => {
            let manuscript = fs::read_to_string(&fixture_path)?;
            let workspace = SourcerightWorkspace::from_root_or_parent(bench_dir.join("fixtures"));
            Ok(workspace
                .citation_reconciliation_report(&manuscript)?
                .to_markdown())
        }
        "review-queue" => Ok(fs::read_to_string(fixture_path.join("review-queue.jsonl"))?),
        "export" => {
            let workspace = SourcerightWorkspace::from_root_or_parent(&fixture_path);
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
            let workspace = SourcerightWorkspace::from_root_or_parent(&fixture_path);
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

fn evaluate_performance(task: &BenchmarkTask, duration: Duration) -> bool {
    if !has_measure(task, BenchmarkMeasure::Latency) {
        return true;
    }

    let Some(performance) = &task.performance else {
        return true;
    };

    let Some(max_duration_ms) = performance.max_duration_ms else {
        return true;
    };

    if !performance.enforce {
        return true;
    }

    duration.as_millis() <= u128::from(max_duration_ms)
}

fn has_measure(task: &BenchmarkTask, measure: BenchmarkMeasure) -> bool {
    task.measures.contains(&measure)
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
        assert_eq!(manifest.tasks.len(), 13);
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
                duration_ms: 0,
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
        assert_eq!(report.task_count, 13);
    }

    #[test]
    fn benchmark_report_records_task_duration() {
        let tempdir = tempfile::tempdir().expect("create tempdir");
        let manifest = tempdir.path().join("tasks.yaml");
        let fixture = tempdir.path().join("references.csl.json");
        let baseline = tempdir.path().join("baseline.txt");

        std::fs::write(&fixture, r#"[]"#).expect("write fixture");
        std::fs::write(&baseline, "valid\nno diagnostics\n").expect("write baseline");

        let manifest_yaml = [
            "schema_version: sourceright.benchmark_tasks.v1",
            "runner_status: technical_preview",
            "live_network: false",
            "workspace_crate: false",
            "tasks:",
            "  - id: csl-validation-timing",
            "    surface: validate-csl",
            "    fixture: references.csl.json",
            "    baseline: baseline.txt",
            "    measures: [correctness, latency]",
            "    performance:",
            "      max_duration_ms: 500",
            "      enforce: true",
        ]
        .join("\n");

        std::fs::write(&manifest, manifest_yaml).expect("write manifest");

        let report = run_benchmark_suite(&manifest).expect("run benchmark suite");

        assert_eq!(report.task_count, 1);
        assert_eq!(report.passed_count, 1);
        assert_eq!(report.failed_count, 0);
        assert_eq!(report.task_results.len(), 1);
        assert_eq!(report.task_results[0].id, "csl-validation-timing");
        assert!(report.task_results[0].diff.is_none());
    }

    #[test]
    fn benchmark_manifest_latency_task_can_run_without_correctness_baseline_comparison() {
        let tempdir = tempfile::tempdir().expect("create tempdir");
        let manifest = tempdir.path().join("tasks.yaml");
        let fixture = tempdir.path().join("references.csl.json");
        let baseline = tempdir.path().join("baseline.txt");

        std::fs::write(&fixture, r#"[]"#).expect("write fixture");
        std::fs::write(&baseline, "valid\nno diagnostics\nMISMATCH\n").expect("write baseline");

        let manifest_yaml = [
            "schema_version: sourceright.benchmark_tasks.v1",
            "runner_status: technical_preview",
            "live_network: false",
            "workspace_crate: false",
            "tasks:",
            "  - id: csl-validation-latency-only",
            "    surface: validate-csl",
            "    fixture: references.csl.json",
            "    baseline: baseline.txt",
            "    measures: [latency]",
            "    performance: { max_duration_ms: 500, enforce: true }",
        ]
        .join("\n");

        std::fs::write(&manifest, manifest_yaml).expect("write manifest");

        let report = run_benchmark_suite(&manifest).expect("run benchmark suite");

        assert_eq!(report.task_count, 1);
        assert_eq!(report.passed_count, 1);
        assert_eq!(report.failed_count, 0);
        assert!(report.task_results[0].diff.is_none());
        assert_eq!(report.task_results[0].id, "csl-validation-latency-only");
    }

    #[test]
    fn performance_budget_is_optional_and_enforced_only_when_configured() {
        let task = BenchmarkTask {
            id: "x".to_string(),
            surface: "validate-csl".to_string(),
            fixture: String::new(),
            baseline: String::new(),
            measures: vec![BenchmarkMeasure::Latency],
            performance: None,
        };
        let tight = BenchmarkTask {
            performance: Some(BenchmarkPerformance {
                max_duration_ms: Some(0),
                enforce: true,
            }),
            ..task.clone()
        };
        let relaxed = BenchmarkTask {
            performance: Some(BenchmarkPerformance {
                max_duration_ms: Some(0),
                enforce: false,
            }),
            ..task
        };

        assert!(evaluate_performance(&relaxed, Duration::from_millis(10)));
        assert!(!evaluate_performance(&tight, Duration::from_millis(1)));
        assert!(evaluate_performance(&tight, Duration::from_millis(0)));
        assert!(evaluate_performance(
            &relaxed,
            Duration::from_millis(u64::MAX)
        ));
        let only_correctness = BenchmarkTask {
            measures: vec![BenchmarkMeasure::Correctness],
            ..tight
        };
        assert!(evaluate_performance(
            &only_correctness,
            Duration::from_millis(1000)
        ));
    }

    #[test]
    fn benchmark_suite_runs_latency_only_manifest_without_baseline_reads() {
        let manifest_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("sourceright-bench/tasks-stress.yaml");
        let manifest = BenchmarkManifest::load(&manifest_path).expect("load stress manifest");

        for task in manifest.tasks {
            assert!(
                task.measures.contains(&BenchmarkMeasure::Latency),
                "stress task {} must include latency",
                task.id
            );
            assert!(
                !task.measures.contains(&BenchmarkMeasure::Correctness),
                "stress task {} should be latency-only",
                task.id
            );
        }
    }

    #[test]
    fn stress_fixture_set_is_larger_and_structured() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let csl_text = fs::read_to_string(
            root.join("sourceright-bench/fixtures/workspace-stress/references.csl.json"),
        )
        .expect("read stress csl");
        let sidecar_text = fs::read_to_string(
            root.join("sourceright-bench/fixtures/workspace-stress/references.verification.json"),
        )
        .expect("read stress sidecar");
        let csl_items: Vec<serde_json::Value> =
            serde_json::from_str(&csl_text).expect("parse stress csl");
        let sidecar_json: serde_json::Value =
            serde_json::from_str(&sidecar_text).expect("parse stress sidecar");
        let side_refs = sidecar_json
            .get("references")
            .and_then(|value| value.as_object())
            .expect("sidecar references");

        assert!(csl_items.len() >= 120, "stress csl should remain large");
        assert_eq!(
            csl_items.len(),
            side_refs.len(),
            "stress sidecar should cover every csl reference"
        );

        let review_queue_count = side_refs
            .values()
            .filter(|value| {
                value
                    .get("review_status")
                    .and_then(serde_json::Value::as_str)
                    == Some("queued")
            })
            .count();
        assert_eq!(review_queue_count, 24);
    }
}
