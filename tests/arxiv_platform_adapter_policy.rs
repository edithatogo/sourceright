use std::fs;
use std::path::Path;

fn read(path: &str) -> String {
    fs::read_to_string(path).expect("fixture should be present")
}

#[test]
fn arxiv_platform_adapters_are_registered_separately_from_provider_arxiv() {
    let registry = read("plugins/registry.toml");
    let submit_manifest = read("plugins/manifests/journal.arxiv-submit-ce.toml");
    let legacy_manifest = read("plugins/manifests/journal.arxiv-submission-core.toml");
    let provider_manifest = read("plugins/manifests/provider.arxiv.toml");

    for marker in [
        "journal.arxiv-submit-ce",
        "plugins/manifests/journal.arxiv-submit-ce.toml",
        "journal.arxiv-submission-core",
        "plugins/manifests/journal.arxiv-submission-core.toml",
        "provider.arxiv",
    ] {
        assert!(
            registry.contains(marker),
            "missing registry marker {marker}"
        );
    }

    assert!(submit_manifest.contains("category = \"journal\""));
    assert!(submit_manifest.contains("submit-ce"));
    assert!(submit_manifest.contains("sourceright.journal_screening.v1"));
    assert!(legacy_manifest.contains("category = \"journal\""));
    assert!(legacy_manifest.contains("submission-core"));
    assert!(legacy_manifest.contains("sourceright.journal_screening.v1"));
    assert!(provider_manifest.contains("category = \"provider\""));
    assert!(provider_manifest.contains("sourceright.verification.v1"));
}

#[test]
fn arxiv_platform_contracts_are_in_shared_schema_and_docs() {
    let schema = read("schemas/sourceright.journal-screening.schema.json");
    let docs = read("docs/src/journal-integrations.md");
    let docs_site = read("docs-site/src/content/docs/journal-integrations.md");

    for platform in ["arxiv_submit_ce", "arxiv_submission_core"] {
        assert!(schema.contains(platform), "schema missing {platform}");
        assert!(docs.contains(platform), "docs missing {platform}");
        assert!(docs_site.contains(platform), "docs site missing {platform}");
    }

    for boundary in [
        "provider.arxiv",
        "journal.arxiv-submit-ce",
        "journal.arxiv-submission-core",
        "fixture-backed",
        "supported journal-screening platform labels with adapter contract fixtures",
    ] {
        assert!(docs.contains(boundary), "docs missing boundary {boundary}");
        assert!(
            docs_site.contains(boundary),
            "docs site missing boundary {boundary}"
        );
    }
    assert!(docs.contains("no platform adapter writes back to arXiv systems or canonical CSL"));
    assert!(docs_site.contains("canonical CSL writeback is claimed"));
}

#[test]
fn arxiv_fixtures_are_runtime_sidecars_and_mcp_manifest_documents_inputs() {
    let mcp_manifest = read("mcp/tools.v1.json");
    let runtime = read("src/mcp.rs");

    for fixture_path in [
        "fixtures/journal/arxiv-submit-ce-submission.json",
        "fixtures/journal/arxiv-submission-core-submission.json",
        "fixtures/journal/arxiv-submit-ce-variants.json",
        "fixtures/journal/arxiv-submission-core-variants.json",
    ] {
        let fixture: serde_json::Value =
            serde_json::from_str(&read(fixture_path)).expect("fixture JSON should parse");
        assert_eq!(
            fixture["verification_sidecar"]["schema_version"], "sourceright.verification.v1",
            "{fixture_path} should be directly consumable as a VerificationSidecar"
        );
        assert!(
            fixture["verification_sidecar"].get("schema").is_none(),
            "{fixture_path} should not use legacy sidecar schema key"
        );
    }

    for marker in [
        "submission_id",
        "manuscript_label",
        "platform_enum",
        "arxiv-submit-ce",
        "arxiv_submission_core",
    ] {
        assert!(
            mcp_manifest.contains(marker),
            "MCP manifest missing {marker}"
        );
    }

    for marker in [
        "submission_id",
        "manuscript_label",
        "\"enum\"",
        "arxiv-submit-ce",
        "arxiv_submission_core",
    ] {
        assert!(
            runtime.contains(marker),
            "MCP runtime schema missing {marker}"
        );
    }
}

#[test]
fn arxiv_variant_fixtures_cover_current_and_legacy_negative_cases() {
    let submit: serde_json::Value =
        serde_json::from_str(&read("fixtures/journal/arxiv-submit-ce-variants.json"))
            .expect("submit-ce variant fixture JSON should parse");
    let legacy: serde_json::Value = serde_json::from_str(&read(
        "fixtures/journal/arxiv-submission-core-variants.json",
    ))
    .expect("submission-core variant fixture JSON should parse");

    assert_eq!(submit["submission"]["platform"], "arxiv_submit_ce");
    assert_eq!(legacy["submission"]["platform"], "arxiv_submission_core");

    let submit_cases = submit["variants"]
        .as_array()
        .expect("submit variants should be an array")
        .iter()
        .filter_map(|case| case["case"].as_str())
        .collect::<Vec<_>>();
    for required in ["complete", "warning", "rejected", "malformed"] {
        assert!(
            submit_cases.contains(&required),
            "submit-ce variants missing {required}"
        );
    }
    assert!(
        submit["variants"]
            .as_array()
            .expect("submit variants")
            .iter()
            .any(|case| case["expected_status"] == "blocked_for_extraction")
    );
    assert!(
        submit["variants"]
            .as_array()
            .expect("submit variants")
            .iter()
            .any(|case| case["expected_status"] == "screened_with_errors")
    );

    let legacy_cases = legacy["submission_events"]
        .as_array()
        .expect("legacy events should be an array")
        .iter()
        .filter_map(|case| case["case"].as_str())
        .collect::<Vec<_>>();
    for required in ["accepted", "held", "rejected", "malformed", "unknown-event"] {
        assert!(
            legacy_cases.contains(&required),
            "submission-core variants missing {required}"
        );
    }
    assert!(
        legacy["submission_events"]
            .as_array()
            .expect("legacy events")
            .iter()
            .any(|case| case["event_type"] == "legacy_custom_event"
                && case["expected_status"] == "screened_with_warnings")
    );
}

#[test]
fn arxiv_track_is_dependency_first_and_parallelized() {
    let metadata = read("conductor/tracks/71-arxiv-submission-platform-adapters/metadata.json");
    let spec = read("conductor/tracks/71-arxiv-submission-platform-adapters/spec.md");
    let plan = read("conductor/tracks/71-arxiv-submission-platform-adapters/plan.md");
    let matrix = read("conductor/tracks/71-arxiv-submission-platform-adapters/test-matrix.md");
    let proof = read("conductor/tracks/71-arxiv-submission-platform-adapters/arxiv-proof.md");
    let review = read("conductor/tracks/71-arxiv-submission-platform-adapters/review.md");
    let tracks = read("conductor/tracks.md");
    let order = read("conductor/implementation-order.md");
    let ledger = read("conductor/evidence-ledger.json");

    for path in [
        "fixtures/journal/arxiv-submit-ce-submission.json",
        "fixtures/journal/arxiv-submission-core-submission.json",
        "schemas/sourceright.arxiv-submission-fixture.schema.json",
        "plugins/manifests/journal.arxiv-submit-ce.toml",
        "plugins/manifests/journal.arxiv-submission-core.toml",
    ] {
        assert!(Path::new(path).exists(), "missing {path}");
    }

    let fixture_schema = read("schemas/sourceright.arxiv-submission-fixture.schema.json");
    assert!(fixture_schema.contains("Sourceright arXiv submission fixture"));
    assert!(fixture_schema.contains("arxiv_submit_ce"));
    assert!(fixture_schema.contains("arxiv_submission_core"));

    for dependency in [
        "16-journal-workflow-integrations",
        "45-external-proof-suites",
        "46-plugin-and-provider-roadmap-delivery",
        "47-contract-evidence-and-overclaim-gates",
        "48-public-api-provider-adapters",
        "60-mature-ojs-plugin",
        "63-plugin-packaging-and-supply-chain-maturity",
    ] {
        assert!(
            metadata.contains(dependency),
            "missing dependency {dependency}"
        );
    }

    for marker in [
        "Dependency slice",
        "Lane A",
        "Lane B",
        "Lane C",
        "Only Lane C edits shared contracts",
        "Address dependencies early",
        "$conductor-review",
    ] {
        assert!(
            spec.contains(marker) || plan.contains(marker),
            "missing parallelization marker {marker}"
        );
    }

    assert!(matrix.contains("Shared dependency contract"));
    assert!(matrix.contains("Default CI"));
    assert!(proof.contains("Do not claim upstream module acceptance"));
    assert!(review.contains("Local review passed"));
    assert!(review.contains("No upstream arXiv module acceptance"));
    assert!(tracks.contains("71 arXiv submission platform adapters"));
    assert!(order.contains("Track 71: arXiv submission platform adapters"));
    assert!(ledger.contains("71-arxiv-submission-platform-adapters"));
    assert!(ledger.contains("No upstream arXiv pull request or module acceptance is claimed"));
}
