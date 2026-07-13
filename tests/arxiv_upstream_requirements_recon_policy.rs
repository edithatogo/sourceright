use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

#[test]
fn track_78_arxiv_upstream_requirements_recon_is_completed() {
    let metadata = read("conductor/tracks/78-arxiv-upstream-requirements-recon/metadata.json");
    assert!(
        metadata.contains("\"status\": \"completed\""),
        "track 78 should be completed"
    );

    for artifact in [
        "requirements-evidence.md",
        "requirements-matrix.md",
        "test-matrix.md",
        "review.md",
    ] {
        let path = format!("conductor/tracks/78-arxiv-upstream-requirements-recon/{artifact}");
        let content = read(&path);
        assert!(!content.is_empty(), "{path} should not be empty");
    }

    let matrix =
        read("conductor/tracks/78-arxiv-upstream-requirements-recon/requirements-matrix.md");
    assert!(matrix.contains("arXiv/submit-ce"));
    assert!(matrix.contains("arXiv/arxiv-submission-core"));
    assert!(matrix.contains("issue-first"));

    let review = read("conductor/tracks/78-arxiv-upstream-requirements-recon/review.md");
    assert!(review.contains("contracted"));
    assert!(review.contains("No upstream write"));

    let packet = read("conductor/submission-packets/arxiv-upstream.md");
    assert!(packet.contains("Track 78 requirements matrix complete"));

    let inventory = read("conductor/submission-requirements.json");
    assert!(inventory.contains("\"id\": \"arxiv-submit-ce\""));
    assert!(inventory.contains("\"id\": \"arxiv-submission-core\""));
    assert!(inventory.contains("\"status\": \"searched\""));

    let ledger = read("conductor/evidence-ledger.json");
    assert!(ledger.contains("78-arxiv-upstream-requirements-recon"));
    assert!(ledger.contains("\"evidence_level\": \"contracted\""));
}
