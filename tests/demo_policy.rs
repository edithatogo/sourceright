use std::fs;

use serde_json::Value;

fn read(path: &str) -> String {
    fs::read_to_string(path).expect("fixture should be present")
}

fn read_json(path: &str) -> Value {
    serde_json::from_str(&read(path)).expect("fixture should be valid JSON")
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
        assert!(!lower.contains("https://"));
        assert!(!lower.contains("http://"));
    }

    assert!(static_js.contains("sample/reference-report.json"));
    assert!(static_js.contains("sample/journal-screening.json"));
    assert!(streamlit_app.contains("sample_workspace"));
}
