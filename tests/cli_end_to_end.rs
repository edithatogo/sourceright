use std::fs;
use std::path::Path;
use std::process::{Command, Output};

use tempfile::TempDir;

fn binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_sourceright"))
}

fn output_text(output: &std::process::Output) -> String {
    String::from_utf8(output.stdout.clone()).expect("stdout must be utf-8")
}

fn run_in_dir(args: &[&str], cwd: &Path) -> Output {
    binary()
        .current_dir(cwd)
        .args(args)
        .output()
        .expect("run command")
}

fn write(path: impl AsRef<Path>, contents: &str) {
    fs::write(path, contents).expect("write fixture");
}

#[test]
fn cli_version_help_and_status_work_end_to_end() {
    let version = binary().arg("--version").output().expect("run version");
    assert!(version.status.success());
    assert!(output_text(&version).contains("sourceright"));

    let help = binary().arg("--help").output().expect("run help");
    assert!(help.status.success());
    assert!(output_text(&help).contains("sourceright"));

    let mcp_status = binary()
        .args(["mcp", "status", "--json"])
        .output()
        .expect("run mcp status");
    assert!(mcp_status.status.success());
    assert!(output_text(&mcp_status).contains("\"server_mode\":\"stdio\""));
}

#[test]
fn cli_bench_smoke_runs_end_to_end() {
    let bench = binary()
        .args([
            "bench",
            "--json",
            "--manifest",
            "sourceright-bench/tasks.yaml",
        ])
        .output()
        .expect("run bench");

    assert!(bench.status.success());
    assert!(output_text(&bench).contains("\"schema_version\":\"sourceright.benchmark_run.v1\""));
}

#[test]
fn cli_surface_sweep_covers_the_major_dispatch_paths() {
    let tempdir = TempDir::new().expect("create tempdir");
    let root = tempdir.path();

    let init = run_in_dir(&["init"], root);
    assert!(init.status.success());

    let workspace = root.join(".sourceright");
    let references = workspace.join("references.csl.json");
    let verification = workspace.join("references.verification.json");
    let review_queue = workspace.join("review-queue.jsonl");
    let exports = workspace.join("exports");
    let manuscript = root.join("manuscript.txt");
    let legal_text = root.join("legal.txt");
    let provenance_text = root.join("provenance.txt");
    let remote_fixture = root.join("remote.json");

    write(
        &references,
        r#"[{"id":"smith-2024","type":"article-journal","title":"Trial","author":[{"family":"Smith"}],"DOI":"10.1000/example"}]"#,
    );
    write(
        &verification,
        r#"{"schema_version":"sourceright.verification.v1","references":{"smith-2024":{"review_status":"queued"}}}"#,
    );
    write(
        &review_queue,
        r#"{"id":"smith-2024","review_status":"queued"}"#,
    );
    write(&manuscript, "Smith (2024) cites the trial.");
    write(&legal_text, "Smith v Jones 2024 NSWSC 1.");
    write(
        &provenance_text,
        "The draft states that Smith (2024) supports the claim.",
    );
    write(&remote_fixture, "[]");

    let validate = run_in_dir(
        &["validate-csl", "--json", &references.display().to_string()],
        root,
    );
    assert!(validate.status.success());
    assert!(output_text(&validate).contains("\"ok\":true"));

    let report = run_in_dir(&["report", "--json", ".sourceright"], root);
    assert!(report.status.success());
    assert!(
        output_text(&report).contains("\"schema_version\":\"sourceright.reference_report.v1\"")
    );

    let conflicts = run_in_dir(&["conflicts", ".sourceright"], root);
    assert!(conflicts.status.success());
    assert!(output_text(&conflicts).contains("smith-2024"));

    let citations = run_in_dir(
        &[
            "citations",
            &manuscript.display().to_string(),
            ".sourceright",
        ],
        root,
    );
    assert!(citations.status.success());
    assert!(output_text(&citations).contains("Smith"));

    let review_queue_output = run_in_dir(&["review", "queue", ".sourceright"], root);
    assert!(review_queue_output.status.success());
    assert!(output_text(&review_queue_output).contains("smith-2024"));

    let review_partitions = run_in_dir(
        &["review", "partitions", "--size", "2", ".sourceright"],
        root,
    );
    assert!(review_partitions.status.success());
    assert!(output_text(&review_partitions).contains("smith-2024"));

    let journal = run_in_dir(
        &[
            "journal-screen",
            "--platform",
            "ojs",
            "--submission-id",
            "SUB-1",
            "--manuscript",
            "manuscript.docx",
            ".sourceright",
        ],
        root,
    );
    assert!(journal.status.success());
    assert!(output_text(&journal).contains("\"submission_id\":\"SUB-1\""));

    let legal = run_in_dir(&["legal", &legal_text.display().to_string()], root);
    assert!(legal.status.success());
    assert!(output_text(&legal).contains("legal_citations"));

    let provenance = run_in_dir(
        &["provenance", &provenance_text.display().to_string()],
        root,
    );
    assert!(provenance.status.success());
    assert!(output_text(&provenance).contains("claim"));

    let policy_output = run_in_dir(&["policy", &references.display().to_string()], root);
    assert!(policy_output.status.success());
    assert!(output_text(&policy_output).contains("sourceright.policy_report.v1"));

    let export_preview = run_in_dir(
        &["export", "--preview", "--format", "ris", ".sourceright"],
        root,
    );
    assert!(export_preview.status.success());
    assert!(output_text(&export_preview).contains("sourceright.export_manifest.v1"));

    let export_all = run_in_dir(&["export", "--all", ".sourceright"], root);
    assert!(export_all.status.success());
    assert!(exports.join("references.ris").exists());

    let plugins = run_in_dir(&["plugins", "validate", "--json"], root);
    assert!(plugins.status.success());
    assert!(output_text(&plugins).contains("sourceright.plugin_registry_report.v1"));

    let bench = run_in_dir(
        &[
            "bench",
            "--json",
            "--manifest",
            "sourceright-bench/tasks.yaml",
        ],
        root,
    );
    assert!(bench.status.success());
    assert!(output_text(&bench).contains("sourceright.benchmark_run.v1"));

    let citation_sync = run_in_dir(&["citation-sync", "--remote-fixture", "remote.json"], root);
    assert!(citation_sync.status.success());
    assert!(output_text(&citation_sync).contains("sourceright.citation_sync.v1"));

    let mcp_status = run_in_dir(&["mcp", "status", "--json"], root);
    assert!(mcp_status.status.success());
    assert!(output_text(&mcp_status).contains("\"server_mode\":\"stdio\""));

    let mcp_tools = run_in_dir(&["mcp", "tools", "--json"], root);
    assert!(mcp_tools.status.success());
    assert!(output_text(&mcp_tools).contains("references.report"));

    let mcp_resources = run_in_dir(&["mcp", "resources", "--json"], root);
    assert!(mcp_resources.status.success());
    assert!(output_text(&mcp_resources).contains("sourceright://plugins/registry"));

    let mcp_prompts = run_in_dir(&["mcp", "prompts", "--json"], root);
    assert!(mcp_prompts.status.success());
    assert!(output_text(&mcp_prompts).contains("provider_conflict_explanation"));

    let unknown = run_in_dir(&["does-not-exist"], root);
    assert!(!unknown.status.success());
    assert!(
        String::from_utf8(unknown.stderr)
            .expect("stderr utf-8")
            .contains("unknown command")
    );
}
