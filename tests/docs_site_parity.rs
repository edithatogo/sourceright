use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

fn read(path: &str) -> String {
    fs::read_to_string(path).expect("fixture should be present")
}

fn docs_site_map() -> BTreeMap<&'static str, &'static str> {
    BTreeMap::from([
        ("index.md", "docs-site/src/content/docs/index.md"),
        (
            "installation.md",
            "docs-site/src/content/docs/guides/installation.md",
        ),
        (
            "workflow.md",
            "docs-site/src/content/docs/guides/workflow.md",
        ),
        (
            "quickstart.md",
            "docs-site/src/content/docs/guides/quickstart.md",
        ),
        (
            "feature-contract-matrix.md",
            "docs-site/src/content/docs/feature-contract-matrix.md",
        ),
        ("design.md", "docs-site/src/content/docs/design.md"),
        (
            "author-preflight-workflow.md",
            "docs-site/src/content/docs/guides/author-preflight-workflow.md",
        ),
        (
            "editorial-triage-workflow.md",
            "docs-site/src/content/docs/guides/editorial-triage-workflow.md",
        ),
        (
            "university-repository-workflow.md",
            "docs-site/src/content/docs/guides/university-repository-workflow.md",
        ),
        (
            "legal-citation-mode-workflow.md",
            "docs-site/src/content/docs/guides/legal-citation-mode-workflow.md",
        ),
        (
            "legal-citation-audit-mcp.md",
            "docs-site/src/content/docs/guides/legal-citation-audit-mcp.md",
        ),
        (
            "legacy-audit.md",
            "docs-site/src/content/docs/legacy-audit.md",
        ),
        ("cli.md", "docs-site/src/content/docs/guides/cli.md"),
        ("mcp.md", "docs-site/src/content/docs/guides/mcp.md"),
        (
            "mcp-server-plan.md",
            "docs-site/src/content/docs/mcp-server-plan.md",
        ),
        (
            "providers.md",
            "docs-site/src/content/docs/reference/providers.md",
        ),
        ("plugins.md", "docs-site/src/content/docs/plugins.md"),
        (
            "plugin-registry.md",
            "docs-site/src/content/docs/plugin-registry.md",
        ),
        (
            "plugin-authoring.md",
            "docs-site/src/content/docs/plugin-authoring.md",
        ),
        ("csl-model.md", "docs-site/src/content/docs/csl-model.md"),
        (
            "verification-sidecar.md",
            "docs-site/src/content/docs/verification-sidecar.md",
        ),
        (
            "schema-contracts.md",
            "docs-site/src/content/docs/schema-contracts.md",
        ),
        (
            "artifact-schema-guide.md",
            "docs-site/src/content/docs/guides/artifact-schema-guide.md",
        ),
        (
            "reporting.md",
            "docs-site/src/content/docs/reference/reporting.md",
        ),
        ("exports.md", "docs-site/src/content/docs/exports.md"),
        (
            "benchmarks.md",
            "docs-site/src/content/docs/guides/benchmarks.md",
        ),
        (
            "citation-manager-integrations.md",
            "docs-site/src/content/docs/citation-manager-integrations.md",
        ),
        (
            "future-scientific-publishing.md",
            "docs-site/src/content/docs/future-scientific-publishing.md",
        ),
        (
            "zotero-plugin-install.md",
            "docs-site/src/content/docs/guides/zotero-plugin-install.md",
        ),
        (
            "host-packaging.md",
            "docs-site/src/content/docs/guides/host-packaging.md",
        ),
        (
            "submission-contracts.md",
            "docs-site/src/content/docs/submission-contracts.md",
        ),
        (
            "journal-integrations.md",
            "docs-site/src/content/docs/journal-integrations.md",
        ),
        (
            "policy-engine.md",
            "docs-site/src/content/docs/policy-engine.md",
        ),
        (
            "style-and-recency.md",
            "docs-site/src/content/docs/style-and-recency.md",
        ),
        (
            "publishing.md",
            "docs-site/src/content/docs/guides/publishing.md",
        ),
        (
            "security-automation.md",
            "docs-site/src/content/docs/guides/security-automation.md",
        ),
        (
            "security-and-quality-gates.md",
            "docs-site/src/content/docs/security-and-quality-gates.md",
        ),
        (
            "release-runbook.md",
            "docs-site/src/content/docs/guides/release-runbook.md",
        ),
        (
            "release-status.md",
            "docs-site/src/content/docs/release-status.md",
        ),
        (
            "release-surface-refresh.md",
            "docs-site/src/content/docs/guides/release-surface-refresh.md",
        ),
        (
            "release-notes.md",
            "docs-site/src/content/docs/guides/release-notes.md",
        ),
        ("faq.md", "docs-site/src/content/docs/guides/faq.md"),
        (
            "pilot-invitation.md",
            "docs-site/src/content/docs/guides/pilot-invitation.md",
        ),
        (
            "coverage-reporting.md",
            "docs-site/src/content/docs/guides/coverage-reporting.md",
        ),
        (
            "coverage-status.md",
            "docs-site/src/content/docs/guides/coverage-status.md",
        ),
        (
            "devsecops-automation-upgrade.md",
            "docs-site/src/content/docs/guides/devsecops-automation-upgrade.md",
        ),
        (
            "operations-status.md",
            "docs-site/src/content/docs/guides/operations-status.md",
        ),
        (
            "docs-cutover.md",
            "docs-site/src/content/docs/guides/docs-cutover.md",
        ),
        (
            "live-provider-configuration.md",
            "docs-site/src/content/docs/guides/live-provider-configuration.md",
        ),
        (
            "limitations.md",
            "docs-site/src/content/docs/guides/limitations.md",
        ),
        (
            "manual-review.md",
            "docs-site/src/content/docs/manual-review.md",
        ),
        (
            "legal-roadmap.md",
            "docs-site/src/content/docs/legal-roadmap.md",
        ),
        (
            "claim-provenance-roadmap.md",
            "docs-site/src/content/docs/claim-provenance-roadmap.md",
        ),
        (
            "submission-contracts.md",
            "docs-site/src/content/docs/submission-contracts.md",
        ),
        (
            "citeweft-grobid-extraction.md",
            "docs-site/src/content/docs/citeweft-grobid-extraction.md",
        ),
        (
            "native-layout-ir.md",
            "docs-site/src/content/docs/native-layout-ir.md",
        ),
        (
            "native-reference-model.md",
            "docs-site/src/content/docs/native-reference-model.md",
        ),
        (
            "native-entity-model.md",
            "docs-site/src/content/docs/native-entity-model.md",
        ),
        (
            "hybrid-routing.md",
            "docs-site/src/content/docs/hybrid-routing.md",
        ),
        (
            "citeweft-repository-extraction.md",
            "docs-site/src/content/docs/citeweft-repository-extraction.md",
        ),
        (
            "contributing.md",
            "docs-site/src/content/docs/guides/contributing.md",
        ),
    ])
}

#[test]
fn every_markdown_doc_has_a_docs_site_parity_page() {
    let parity = docs_site_map();
    let docs = fs::read_dir("docs/src").expect("docs/src should be readable");

    for entry in docs {
        let entry = entry.expect("docs/src entry");
        let path = entry.path();
        if path.file_name().and_then(|value| value.to_str()) == Some("SUMMARY.md") {
            continue;
        }
        if path.extension().and_then(|value| value.to_str()) != Some("md") {
            continue;
        }

        let file_name = path
            .file_name()
            .and_then(|value| value.to_str())
            .expect("file name");
        let target = parity
            .get(file_name)
            .unwrap_or_else(|| panic!("missing docs-site parity page for {file_name}"));
        assert!(
            Path::new(target).exists(),
            "missing docs-site page for {file_name}"
        );
    }
}

#[test]
fn docs_site_pages_still_reference_the_public_docs_site_contract() {
    let publishing = read("docs-site/src/content/docs/guides/publishing.md");
    let cutover = read("docs-site/src/content/docs/guides/docs-cutover.md");
    let release = read("docs-site/src/content/docs/guides/release-runbook.md");
    let guide_index = read("docs-site/src/content/docs/guides/index.md");
    let plugin_authoring = read("docs-site/src/content/docs/plugin-authoring.md");
    let providers = read("docs-site/src/content/docs/reference/providers.md");
    let mcp_plan = read("docs-site/src/content/docs/mcp-server-plan.md");
    let mcp_guide = read("docs-site/src/content/docs/guides/mcp.md");

    assert!(publishing.contains("Astro site"));
    assert!(cutover.contains("canonical public docs surface"));
    assert!(release.contains("attestation"));
    for marker in [
        "Release Runbook",
        "Release Surface Refresh",
        "Coverage Reporting",
        "Docs Cutover",
    ] {
        assert!(guide_index.contains(marker), "guide index missing {marker}");
    }
    for marker in [
        "Provider plugins must not overwrite canonical CSL fields silently",
        "Evidence-Ledger Requirements",
        "Sandbox Policy",
    ] {
        assert!(
            plugin_authoring.contains(marker),
            "plugin authoring missing {marker}"
        );
    }
    for marker in [
        "Implementation Order",
        "Fixture and Mocking Expectations",
        "Expanded Provider Catalogue",
        "Google Scholar Decision Log",
    ] {
        assert!(
            providers.contains(marker),
            "providers page missing {marker}"
        );
    }
    for marker in [
        "Current read-only tools",
        "Write-capable tools should remain dry-run by default",
        "require explicit apply",
    ] {
        assert!(
            mcp_plan.contains(marker),
            "MCP server plan missing {marker}"
        );
    }
    assert!(
        mcp_guide.contains("[mcp_servers.sourceright]"),
        "Codex MCP guide must use the TOML config shape"
    );
}
