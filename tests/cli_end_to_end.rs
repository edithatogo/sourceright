use std::process::Command;

fn binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_sourceright"))
}

fn output_text(output: &std::process::Output) -> String {
    String::from_utf8(output.stdout.clone()).expect("stdout must be utf-8")
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
