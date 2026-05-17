use std::fs;

#[test]
fn zotero_live_smoke_workflow_is_manual_and_secret_gated() {
    let workflow =
        fs::read_to_string(".github/workflows/zotero-live-smoke.yml").expect("read workflow");

    assert!(workflow.contains("workflow_dispatch"));
    assert!(workflow.contains("zotero-live-smoke"));
    assert!(workflow.contains("SOURCERIGHT_ZOTERO_LIVE_SMOKE"));
    assert!(workflow.contains("SOURCERIGHT_ZOTERO_API_KEY"));
    assert!(workflow.contains("SOURCERIGHT_ZOTERO_LIBRARY_ID"));
    assert!(workflow.contains("cargo test zotero --lib"));
    assert!(workflow.contains("--ignored --nocapture"));
    assert!(!workflow.contains("pull_request"));
}

#[test]
fn zotero_docs_do_not_claim_xpi_submission_or_desktop_test() {
    let docs = fs::read_to_string("docs/src/zotero-plugin-install.md").expect("read docs");
    let decision =
        fs::read_to_string("conductor/tracks/58-mature-zotero-plugin/packaging-decision.md")
            .expect("read packaging decision");

    for marker in [
        "there is no Zotero Plugin Gallery submission to make",
        "official plugin directory is planned",
        "Because Sourceright does not ship",
        "not write to Zotero",
        "future `.xpi` plugin track",
    ] {
        assert!(docs.contains(marker), "missing docs marker: {marker}");
    }

    assert!(decision.contains("not applicable"));
    assert!(decision.contains("Desktop loading tests"));
    assert!(decision.contains("until a real `.xpi` package exists"));
}
