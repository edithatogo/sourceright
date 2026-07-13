use std::fs;

#[test]
fn workflow_harness_is_a_required_quality_sensor() {
    let workflow = fs::read_to_string(".github/workflows/quality.yml").unwrap();
    let harness = fs::read_to_string("scripts/check-workflow-harness.ps1").unwrap();

    assert!(workflow.contains("./scripts/check-workflow-harness.ps1"));
    assert!(harness.contains("^[0-9a-f]{40}$"));
    assert!(harness.contains("persist-credentials"));
    assert!(harness.contains("top-level permissions"));
    assert!(harness.contains("top-level concurrency"));
    assert!(harness.contains("timeout-minutes"));
}
