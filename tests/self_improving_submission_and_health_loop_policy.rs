use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

#[test]
fn track_82_self_improving_submission_and_health_loop_is_completed() {
    let metadata =
        read("conductor/tracks/82-self-improving-submission-and-health-loop/metadata.json");
    assert!(
        metadata.contains("\"status\": \"completed\""),
        "track 82 should be completed"
    );

    for artifact in [
        "requirements-evidence.md",
        "health-loop-2026-06-09.md",
        "review.md",
    ] {
        let path =
            format!("conductor/tracks/82-self-improving-submission-and-health-loop/{artifact}");
        let content = read(&path);
        assert!(!content.is_empty(), "{path} should not be empty");
    }

    let health = read(
        "conductor/tracks/82-self-improving-submission-and-health-loop/health-loop-2026-06-09.md",
    );
    assert!(health.contains("9.5"));
    assert!(health.contains("verify-submission-readiness.ps1"));

    let review = read("conductor/tracks/82-self-improving-submission-and-health-loop/review.md");
    assert!(review.contains("fixture-backed"));
    assert!(review.contains("No external submissions"));

    let agent_workflow = read("conductor/submission-packets/agent-workflow.md");
    assert!(agent_workflow.contains("Tracks 73-81"));
    assert!(agent_workflow.contains("stable host package"));

    let inventory = read("conductor/submission-requirements.json");
    assert!(inventory.contains("\"repo_health_target\": 9.5"));
    assert!(inventory.contains("verify-submission-readiness.ps1"));

    let tracks = read("conductor/tracks.md");
    assert!(tracks.contains("| 82 self-improving submission and health loop | completed |"));

    let ledger = read("conductor/evidence-ledger.json");
    assert!(ledger.contains("82-self-improving-submission-and-health-loop"));
}
