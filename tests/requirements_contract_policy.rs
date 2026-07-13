use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).expect("fixture should be present")
}

#[test]
fn feature_contract_matrix_is_canonical_and_granular() {
    let matrix = read("docs/src/feature-contract-matrix.md");

    assert!(matrix.contains("public requirements document and repo contract"));
    assert!(matrix.contains("conductor/requirements.md"));
    for priority in ["Must", "Should", "Could", "Won't for now"] {
        assert!(
            matrix.contains(priority),
            "missing MoSCoW priority {priority}"
        );
    }
    for contract in [
        "references.csl.json",
        "references.verification.json",
        "review-queue.jsonl",
        "Provider evidence must not silently overwrite canonical CSL",
        "OJS",
        "MCP",
        "Zotero",
        "official MCP Registry",
        "Fixture-backed refresh cadence",
        "scripts/verify-release-surface-refresh.ps1",
    ] {
        assert!(
            matrix.contains(contract),
            "missing contract marker {contract}"
        );
    }
}

#[test]
fn design_doc_covers_architecture_with_mermaid_diagrams() {
    let design = read("docs/src/design.md");

    assert!(design.matches("```mermaid").count() >= 6);
    for section in [
        "Data Boundaries",
        "CLI And MCP Surfaces",
        "Providers And Plugins",
        "Journal Integration",
        "Citation Manager Sync",
        "Release And Registry Flow",
    ] {
        assert!(design.contains(section), "missing design section {section}");
    }
}

#[test]
fn docs_site_has_contract_and_design_parity() {
    let site_matrix = read("docs-site/src/content/docs/feature-contract-matrix.md");
    let site_design = read("docs-site/src/content/docs/design.md");

    assert!(site_matrix.contains("MoSCoW matrix"));
    assert!(site_matrix.contains("Default live external CI"));
    assert!(site_design.matches("```mermaid").count() >= 6);
    assert!(site_design.contains("Release and registry flow"));
}

#[test]
fn conductor_requirements_and_design_are_canonical_contracts() {
    let requirements = read("conductor/requirements.md");
    let design = read("conductor/design.md");
    let workflow = read("conductor/workflow.md");
    let product = read("conductor/product.md");

    for priority in ["Must", "Should", "Could", "Won't for now"] {
        assert!(
            requirements.contains(priority),
            "missing Conductor MoSCoW priority {priority}"
        );
    }
    for marker in [
        "Automatic Track Progression Contract",
        "Completion Evidence Levels",
        "Overclaim guard",
        "Provider evidence must not silently overwrite canonical CSL",
        "Claim/source/provenance work must not assert claim truth",
    ] {
        assert!(
            requirements.contains(marker),
            "missing Conductor requirements marker {marker}"
        );
    }
    assert!(
        design.matches("```mermaid").count() >= 6,
        "Conductor design must contain comprehensive Mermaid diagrams"
    );
    assert!(workflow.contains("conductor/requirements.md"));
    assert!(workflow.contains("conductor/evidence-ledger.json"));
    assert!(workflow.contains("$conductor-review"));
    assert!(product.contains("conductor/requirements.md"));
}

#[test]
fn release_maturity_control_layer_exists() {
    for path in [
        "conductor/evidence-ledger.json",
        "conductor/implementation-order.md",
        "conductor/plugin-compatibility-matrix.md",
        "conductor/release-channels.md",
        "conductor/secrets-and-live-tests.md",
        "conductor/deprecation-policy.md",
        "conductor/track-template.md",
        "conductor/adrs/0001-no-submodules-by-default.md",
        "conductor/adrs/0002-rust-core-thin-adapters.md",
        "conductor/adrs/0003-zotero-plugin-after-proof.md",
        "conductor/adrs/0004-ojs-plugin-after-platform-contract.md",
        "conductor/adrs/0005-no-google-scholar-scraping.md",
    ] {
        assert!(std::path::Path::new(path).exists(), "missing {path}");
    }

    let ledger = read("conductor/evidence-ledger.json");
    assert!(ledger.contains("sourceright.conductor.evidence-ledger.v1"));
    assert!(ledger.contains("publicly-accepted"));
    assert!(ledger.contains("Mature Zotero integration is planned."));

    let order = read("conductor/implementation-order.md");
    assert!(order.contains("Foundation First"));
    assert!(order.contains("Parallelization Rule"));
    assert!(order.contains("Track 70 refreshes release-surface evidence"));

    let release_channels = read("conductor/release-channels.md");
    assert!(
        release_channels.contains("release-surface refresh before publication wording changes")
    );
    assert!(release_channels.contains("56, 57, 70"));
    assert!(release_channels.contains("66, 69, 70"));

    let secrets = read("conductor/secrets-and-live-tests.md");
    assert!(secrets.contains("SOURCERIGHT_PROVIDER_LIVE"));
    assert!(secrets.contains("Redaction"));

    let adr = read("conductor/adrs/0005-no-google-scholar-scraping.md");
    assert!(adr.contains("No Google Scholar Scraping"));
}

#[test]
fn optional_future_roadmap_is_visible_but_not_claimed() {
    let requirements = read("conductor/requirements.md");

    assert!(requirements.contains("Optional Future Roadmap Requirements"));
    assert!(requirements.contains("do not block the mature stable"));
    assert!(requirements.contains("release. They should not be described as implemented"));
    for optional in [
        "Hosted team workspace",
        "Web UI for review queues",
        "Browser extension for manuscript systems",
        "Word/LibreOffice add-ins",
        "VS Code / IDE extension",
        "Full HTTP MCP service",
        "Organization policy packs",
        "Public benchmark corpus",
        "ML-assisted matching",
        "OCR/layout engine integration",
        "Multilingual citation workflows",
        "Legal-provider expansion",
        "Research integrity signals",
        "Institutional repository deposit workflow",
        "LLM-agent review assistants",
        "Enterprise audit exports",
    ] {
        assert!(
            requirements.contains(optional),
            "missing optional roadmap item {optional}"
        );
    }
    assert!(requirements.contains("Do not claim hosted MCP from local stdio support."));
    assert!(
        requirements
            .contains("Do not claim SOTA or public benchmark leadership before this exists.")
    );
    assert!(
        requirements
            .contains("Do not let agents silently change canonical CSL or external systems.")
    );
}

#[test]
fn remaining_work_tracks_are_granular_and_review_gated() {
    let tracks = read("conductor/tracks.md");
    for track in [
        "42 GitHub automation and alert operations",
        "43 Publication registry completion",
        "44 Branch triage and stale-work closure",
        "45 External proof suites",
        "46 Plugin and provider roadmap delivery",
        "47 Contract evidence and overclaim gates",
        "48 Public API provider adapters",
        "49 Licensed BYO-key provider adapters",
        "50 Repository record provider adapters",
        "51 Citation-manager adapter proof",
        "52 Non-provider pipeline plugins",
        "53 CourtListener legal provider",
        "54 Demo public surface proof",
        "55 Benchmark robustness contract",
        "56 MCP registry release binding",
        "57 Smithery distribution readiness",
        "58 Mature Zotero plugin",
        "59 Other citation-manager integrations",
        "60 Mature OJS plugin",
        "61 Streamlit app publication and hardening",
        "62 Expanded normaliser/provider catalogue",
        "63 Plugin packaging and supply-chain maturity",
        "64 GitHub-side governance additions",
    ] {
        assert!(
            tracks.contains(track),
            "missing remaining-work track {track}"
        );
    }

    for slug in [
        "42-github-automation-and-alert-operations",
        "43-publication-registry-completion",
        "44-branch-triage-and-stale-work-closure",
        "45-external-proof-suites",
        "46-plugin-and-provider-roadmap-delivery",
        "47-contract-evidence-and-overclaim-gates",
        "48-public-api-provider-adapters",
        "49-licensed-byo-key-provider-adapters",
        "50-repository-record-provider-adapters",
        "51-citation-manager-adapter-proof",
        "52-non-provider-pipeline-plugins",
        "53-courtlistener-legal-provider",
        "54-demo-public-surface-proof",
        "55-benchmark-robustness-contract",
        "56-mcp-registry-release-binding",
        "57-smithery-distribution-readiness",
        "58-mature-zotero-plugin",
        "59-other-citation-manager-integrations",
        "60-mature-ojs-plugin",
        "61-streamlit-app-publication-and-hardening",
        "62-expanded-normaliser-provider-catalogue",
        "63-plugin-packaging-and-supply-chain-maturity",
        "64-github-side-governance-additions",
    ] {
        for file in ["metadata.json", "spec.md", "plan.md", "test-matrix.md"] {
            let path = format!("conductor/tracks/{slug}/{file}");
            assert!(
                std::path::Path::new(&path).exists(),
                "missing track file {path}"
            );
        }
        let plan = read(&format!("conductor/tracks/{slug}/plan.md"));
        let test_matrix = read(&format!("conductor/tracks/{slug}/test-matrix.md"));
        assert!(
            plan.contains("$conductor-review") || test_matrix.contains("$conductor-review"),
            "track {slug} must require conductor review"
        );
        assert!(
            plan.contains("Apply") || plan.contains("apply"),
            "track {slug} must describe automatic local fix application"
        );
    }
}

#[test]
fn forbidden_claims_are_not_made_without_evidence() {
    let docs = [
        "README.md",
        "docs/src/limitations.md",
        "docs/src/release-notes.md",
        "docs/src/feature-contract-matrix.md",
        "docs/src/design.md",
        "conductor/requirements.md",
        "conductor/design.md",
    ];

    let forbidden_patterns = [
        (
            "production-ready institutional platform",
            "only fixture-backed tracks exist",
        ),
        (
            "examiner-grade final verifier",
            "excluded until tracks prove it",
        ),
        (
            "SOTA benchmarked performance",
            "fixture-backed regression only",
        ),
        (
            "state-of-the-art benchmarked performance",
            "fixture-backed regression only",
        ),
        ("AI detector", "excluded by policy"),
        (
            "AI authorship detection",
            "citation errors never treated as AI",
        ),
        (
            "legal filing compliance",
            "legal citation mode excludes filing compliance",
        ),
        (
            "legal filing compliance system",
            "legal citation mode excludes filing",
        ),
    ];

    for path in docs {
        if !std::path::Path::new(path).exists() {
            continue;
        }
        let content = std::fs::read_to_string(path).unwrap_or_default();
        for (forbidden, _context) in &forbidden_patterns {
            if content.contains(forbidden) {
                // The forbidden phrase may appear in a "Do not claim" / "Forbidden" context.
                // Accept it only if accompanied by explicit guard language.
                let guards = [
                    "Do not claim",
                    "Forbidden",
                    "must never",
                    "blocked until",
                    "explicitly excluded",
                    "Won't for now",
                    "explicitly excludes",
                    "not treated as proof",
                ];
                let has_guard = guards.iter().any(|g| content.contains(g));
                assert!(
                    has_guard,
                    "Forbidden claim '{forbidden}' found in {path} without guard language"
                );
            }
        }
    }
}

#[test]
fn evidence_ledger_track_47_exists() {
    let ledger = std::fs::read_to_string("conductor/evidence-ledger.json")
        .expect("evidence-ledger.json should exist");
    assert!(
        ledger.contains("47-contract-evidence-and-overclaim-gates"),
        "evidence-ledger.json must contain a track 47 entry"
    );
    assert!(
        ledger.contains("fixture-backed") || ledger.contains("contracted"),
        "track 47 must have at least contracted evidence level"
    );
}

#[test]
fn limitations_doc_has_forbidden_claims_table() {
    let limitations =
        std::fs::read_to_string("docs/src/limitations.md").expect("limitations.md should exist");
    assert!(limitations.contains("Forbidden claims"));
    assert!(limitations.contains("Production-ready institutional platform"));
    assert!(limitations.contains("Examiner-grade final verifier"));
    assert!(limitations.contains("SOTA benchmarked performance"));
    assert!(limitations.contains("AI detector"));
    assert!(limitations.contains("Legal filing compliance system"));
    assert!(limitations.contains("Live provider verified"));
    assert!(limitations.contains("Registry accepted"));
}

#[test]
fn docs_site_limitations_parity() {
    let src = std::fs::read_to_string("docs/src/limitations.md")
        .expect("docs/src/limitations.md should exist");
    let site = std::fs::read_to_string("docs-site/src/content/docs/limitations.md")
        .expect("docs-site limitations should exist");
    assert!(site.contains("Forbidden claims"));
    assert!(site.contains("Production-ready institutional platform"));
    assert!(src.contains("Forbidden claims"));
    assert!(src.contains("Examiner-grade final verifier"));
}

#[test]
fn docs_site_release_notes_parity() {
    let src = std::fs::read_to_string("docs/src/release-notes.md")
        .expect("docs/src/release-notes.md should exist");
    let site = std::fs::read_to_string("docs-site/src/content/docs/release-notes.md")
        .expect("docs-site release-notes should exist");
    assert!(src.contains("Release wording"));
    assert!(site.contains("Release wording"));
    assert!(src.contains("technical preview"));
    assert!(site.contains("technical preview"));
}

#[test]
fn conductor_design_has_anti_overclaim_diagram() {
    let design =
        std::fs::read_to_string("conductor/design.md").expect("conductor/design.md should exist");
    assert!(design.contains("Anti-Overclaim Gate"));
    assert!(design.matches("```mermaid").count() >= 7);
}

#[test]
fn conductor_requirements_has_track_47_overclaim_guards() {
    let requirements = std::fs::read_to_string("conductor/requirements.md")
        .expect("conductor/requirements.md should exist");
    assert!(requirements.contains("Automatic final verification"));
    assert!(requirements.contains("AI detector"));
    assert!(
        requirements.contains("track 47")
            || requirements.contains("Track 47")
            || requirements.contains("47 |")
    );
    assert!(requirements.contains("Use \"technical preview\" and \"structured triage\" wording."));
    assert!(requirements.contains("Never equate citation errors with AI authorship."));
}

#[test]
fn external_proofs_keep_mcp_write_surfaces_out_of_read_only_contracts() {
    let mcp_proof = read("conductor/tracks/45-external-proof-suites/mcp-transcript-proof.md");
    let citation_proof =
        read("conductor/tracks/45-external-proof-suites/citation-manager-proof.md");

    assert!(mcp_proof.contains("implemented_apply_gated_write_surfaces"));
    assert!(mcp_proof.contains("exports.write apply=true"));
    assert!(
        !mcp_proof
            .contains("  - sourceright plugins [validate] [--json]\n  - sourceright export --all"),
        "export writes must not be listed in the read-only MCP proof surface"
    );
    assert!(citation_proof.contains("Citation-Sync Is Not A Read-Only MCP Surface"));
    assert!(citation_proof.contains("contains(\"citation-sync\") | not"));
    assert!(
        !citation_proof.contains("select(contains(\"citation-sync\"))"),
        "citation-sync must not be advertised as a read-only MCP surface"
    );
}

#[test]
fn mcp_read_only_docs_do_not_include_apply_gated_or_cli_sync_surfaces() {
    for path in [
        "docs/src/mcp.md",
        "docs/src/mcp-server-plan.md",
        "docs-site/src/content/docs/mcp-server-plan.md",
    ] {
        let content = read(path);
        let read_only_start = content
            .find("Read-only tools:")
            .or_else(|| content.find("Current read-only tools"))
            .expect("read-only section should be present");
        let read_only_tail = &content[read_only_start..];
        let read_only_section = read_only_tail
            .split("Write-capable")
            .next()
            .expect("read-only section should precede write-capable section");

        assert!(
            read_only_section.contains("exports.preview")
                || read_only_section.contains("preview export artifacts"),
            "{path} read-only MCP docs should include export preview, not export writes"
        );
        assert!(
            !read_only_section.contains("exports.write"),
            "{path} must not list exports.write in read-only MCP docs"
        );
        assert!(
            !read_only_section.contains("citation-sync"),
            "{path} must not advertise citation-sync as a read-only MCP surface"
        );
        assert!(
            !read_only_section.contains("import-decisions"),
            "{path} must not advertise review import-decisions as read-only"
        );
    }
}

#[test]
fn plugin_registry_has_owned_remaining_work_contract() {
    let registry = read("plugins/registry.toml");
    let track = read("conductor/tracks/46-plugin-and-provider-roadmap-delivery/spec.md");

    for plugin in [
        "provider.dimensions",
        "provider.scopus",
        "provider.web-of-science",
        "provider.unpaywall",
        "provider.opencitations",
        "provider.arxiv",
        "provider.europepmc",
        "provider.repository-records",
        "citation-manager.zotero",
        "citation-manager.endnote",
        "journal.ojs",
        "legal.courtlistener",
        "matcher.local-bibliographic",
        "recency.retractions",
        "relevance.claim-source",
        "extraction.docx-pdf",
        "demo.github-pages",
    ] {
        assert!(
            registry.contains(plugin),
            "missing plugin registry entry {plugin}"
        );
    }
    for family in [
        "Public API providers",
        "Licensed BYO-key providers",
        "Adapters",
        "Local logic",
        "Planned plugins stay visibly planned",
    ] {
        assert!(
            track.contains(family),
            "plugin delivery track missing {family}"
        );
    }
}
