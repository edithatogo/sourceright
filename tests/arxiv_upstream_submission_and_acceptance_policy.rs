use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

#[test]
fn track_81_arxiv_upstream_submission_and_acceptance_is_completed() {
    let metadata =
        read("conductor/tracks/81-arxiv-upstream-submission-and-acceptance/metadata.json");
    assert!(
        metadata.contains("\"status\": \"completed\""),
        "track 81 should be completed"
    );

    for artifact in [
        "requirements-evidence.md",
        "readiness-review-2026-06-09.md",
        "approval-gates.md",
        "submission-drafts.md",
        "review.md",
    ] {
        let path =
            format!("conductor/tracks/81-arxiv-upstream-submission-and-acceptance/{artifact}");
        let content = read(&path);
        assert!(!content.is_empty(), "{path} should not be empty");
    }

    let readiness = read(
        "conductor/tracks/81-arxiv-upstream-submission-and-acceptance/readiness-review-2026-06-09.md",
    );
    assert!(readiness.contains("Track 78"));
    assert!(readiness.contains("Track 79"));
    assert!(readiness.contains("Track 80"));
    assert!(readiness.contains("green"));

    let review = read("conductor/tracks/81-arxiv-upstream-submission-and-acceptance/review.md");
    assert!(review.contains("submitted"));
    assert!(review.contains("No upstream acceptance"));

    let drafts =
        read("conductor/tracks/81-arxiv-upstream-submission-and-acceptance/submission-drafts.md");
    assert!(drafts.contains("Rollback"));
    assert!(drafts.contains("issues/72"));
    assert!(drafts.contains("issues/88"));
    assert!(drafts.contains("Submitted"));

    let live_evidence = read("conductor/submission-packets/live-evidence.json");
    assert!(live_evidence.contains("arxiv-submit-ce"));
    assert!(live_evidence.contains("issues/72"));
    assert!(live_evidence.contains("issues/88"));

    let packet = read("conductor/submission-packets/arxiv-upstream.md");
    assert!(packet.contains("Track 81 readiness review complete"));

    let manifest = read("conductor/submission-packets/manifest.json");
    assert!(manifest.contains("\"id\": \"arxiv-upstream\""));
    assert!(manifest.contains("\"status\": \"submitted\""));

    let ledger = read("conductor/evidence-ledger.json");
    assert!(ledger.contains("81-arxiv-upstream-submission-and-acceptance"));
    assert!(ledger.contains("\"evidence_level\": \"submitted\""));
}
