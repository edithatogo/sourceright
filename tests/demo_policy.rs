use std::fs;
use std::process::Command;

use serde_json::Value;

fn read(path: &str) -> String {
    fs::read_to_string(path).expect("fixture should be present")
}

fn read_json(path: &str) -> Value {
    serde_json::from_str(&read(path)).expect("fixture should be valid JSON")
}

fn command_exists(command: &str) -> bool {
    Command::new(command)
        .arg("--version")
        .output()
        .is_ok_and(|output| output.status.success())
}

#[test]
fn demo_samples_are_present_and_schema_versioned() {
    for root in ["github_pages_demo/sample", "streamlit_app/sample_workspace"] {
        let reference_report = read_json(&format!("{root}/reference-report.json"));
        let journal_screen = read_json(&format!("{root}/journal-screening.json"));

        assert_eq!(
            reference_report["schema_version"],
            "sourceright.reference_report.v1"
        );
        assert_eq!(reference_report["report_type"], "reference_integrity");
        assert_eq!(
            journal_screen["schema_version"],
            "sourceright.journal_screening.v1"
        );
        assert_eq!(
            journal_screen["reference_report"]["schema_version"],
            "sourceright.reference_report.v1"
        );
    }

    assert_eq!(
        read_json("github_pages_demo/sample/reference-report.json"),
        read_json("streamlit_app/sample_workspace/reference-report.json")
    );
    assert_eq!(
        read_json("github_pages_demo/sample/journal-screening.json"),
        read_json("streamlit_app/sample_workspace/journal-screening.json")
    );
}

#[test]
fn demos_remain_sample_data_only_and_do_not_call_live_services() {
    let static_readme = read("github_pages_demo/README.md");
    let static_html = read("github_pages_demo/index.html");
    let static_js = read("github_pages_demo/app.js");
    let streamlit_readme = read("streamlit_app/README.md");
    let streamlit_app = read("streamlit_app/app.py");

    for content in [&static_readme, &streamlit_readme] {
        assert!(content.contains("sample"));
        assert!(content.contains("not a live verification service"));
    }

    for content in [&static_html, &static_js, &streamlit_app] {
        let lower = content.to_ascii_lowercase();
        assert!(lower.contains("synthetic sample data only"));
        assert!(lower.contains("does not call live providers"));
        assert!(!lower.contains("fetch(\"http"));
        assert!(!lower.contains("fetch('http"));
        assert!(!lower.contains("requests."));
        assert!(!lower.contains("urllib."));
        assert!(!lower.contains("httpx."));
    }

    assert!(static_js.contains("sample/reference-report.json"));
    assert!(static_js.contains("sample/journal-screening.json"));
    assert!(streamlit_app.contains("sample_workspace"));
}

#[test]
fn static_demo_render_smoke_passes_when_node_is_available() {
    if !command_exists("node") {
        eprintln!("skipping static demo render smoke because node is not available");
        return;
    }

    let output = Command::new("node")
        .arg("github_pages_demo/render-smoke.mjs")
        .output()
        .expect("run static demo render smoke");

    assert!(
        output.status.success(),
        "static demo render smoke failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn streamlit_demo_model_smoke_passes_when_python_is_available() {
    if !command_exists("python") {
        eprintln!("skipping Streamlit demo model smoke because python is not available");
        return;
    }

    let output = Command::new("python")
        .args(["-m", "unittest", "streamlit_app.test_demo_model"])
        .output()
        .expect("run Streamlit demo model smoke");

    assert!(
        output.status.success(),
        "Streamlit demo model smoke failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
