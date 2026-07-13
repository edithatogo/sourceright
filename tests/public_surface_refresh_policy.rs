use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

#[test]
fn release_surface_refresh_track_is_registered() {
    let tracks = read("conductor/tracks.md");
    let metadata = read("conductor/tracks/70-release-surface-refresh-cadence/metadata.json");
    let plan = read("conductor/tracks/70-release-surface-refresh-cadence/plan.md");
    let test_matrix = read("conductor/tracks/70-release-surface-refresh-cadence/test-matrix.md");
    let script = read("scripts/verify-release-surface-refresh.ps1");
    let release_check = read("scripts/release-check.ps1");
    let release_dry_run = read(".github/workflows/release-dry-run.yml");
    let evidence_ledger = read("conductor/evidence-ledger.json");

    assert!(tracks.contains("70 Release surface refresh cadence"));
    assert!(metadata.contains("\"status\": \"completed\""));
    assert!(metadata.contains("docs/src/release-surface-refresh.md"));
    assert!(metadata.contains("scripts/verify-release-surface-refresh.ps1"));
    assert!(plan.contains("$conductor-review"));
    assert!(plan.contains("scripts\\verify-release-surface-refresh.ps1"));
    assert!(test_matrix.contains("Prepared/deferred surfaces stay bounded"));
    assert!(test_matrix.contains("Local refresh verification"));
    assert!(script.contains("Prepared metadata is not public marketplace acceptance"));
    assert!(script.contains("Assert-ReleaseStatusParity"));
    assert!(script.contains("Assert-TrackEvidenceParity"));
    assert!(script.contains("release status row has no mapped evidence source"));
    assert!(script.contains("GHCR MCP image"));
    assert!(script.contains("Claude Desktop client config"));
    assert!(script.contains("GitHub Copilot coding-agent prep"));
    assert!(script.contains("Chocolatey"));
    assert!(script.contains("Release surface refresh evidence checks passed."));
    assert!(release_check.contains("verify-release-surface-refresh.ps1"));
    assert!(release_check.contains("Running release-surface evidence refresh checks"));
    assert!(release_dry_run.contains("Release surface evidence"));
    assert!(release_dry_run.contains("./scripts/verify-release-surface-refresh.ps1"));
    assert!(evidence_ledger.contains("70-release-surface-refresh-cadence"));
    assert!(evidence_ledger.contains("No prepared or deferred release surface is promoted"));
    assert!(evidence_ledger.contains("Live registry checks remain opt-in"));
}

#[test]
fn refresh_guide_defines_promotion_rules_and_claim_boundaries() {
    let guide = read("docs/src/release-surface-refresh.md");
    let site_guide = read("docs-site/src/content/docs/guides/release-surface-refresh.md");

    for required in [
        "public URL, version or artifact id, verification date, and install metadata",
        "Prepared metadata can move to `accepted` only when the public listing is visible and installable",
        "Local config examples, package templates, source skeletons, or registry-ready metadata are not enough",
        "scripts/verify-release-surface-refresh.ps1",
        "technical preview",
        "pilot-ready",
        "fixture-backed regression benchmark",
        "deterministic benchmark scaffold",
    ] {
        assert!(
            guide.contains(required),
            "missing source guide marker: {required}"
        );
        assert!(
            site_guide.contains(required),
            "missing docs-site guide marker: {required}"
        );
    }

    for guarded in [
        "Glama listing",
        "Smithery listing",
        "Zotero package or Plugin Gallery evidence",
        "OJS/PKP Plugin Gallery evidence",
        "VS Code Marketplace or Open VSX package evidence",
        "Microsoft AppSource or sideloaded Word add-in proof",
        "LibreOffice `.oxt` proof",
        "Homebrew, Scoop, winget, npm, and PyPI wrapper decisions",
    ] {
        assert!(
            guide.contains(guarded),
            "missing watch-list marker: {guarded}"
        );
        assert!(
            site_guide.contains(guarded),
            "missing docs-site watch-list marker: {guarded}"
        );
    }
}

#[test]
fn release_status_pages_link_to_refresh_cadence() {
    let summary = read("docs/src/SUMMARY.md");
    let release_status = read("docs/src/release-status.md");
    let site_release_status = read("docs-site/src/content/docs/release-status.md");
    let requirements = read("conductor/requirements.md");

    assert!(summary.contains("[Release Surface Refresh](release-surface-refresh.md)"));
    assert!(release_status.contains("Release Surface Refresh"));
    assert!(site_release_status.contains("guides/release-surface-refresh"));
    assert!(release_status.contains("latest verified public release surface is `v0.1.20`"));
    assert!(site_release_status.contains("latest verified public release surface is `v0.1.20`"));
    for row in [
        "| Official MCP Registry | accepted | 0.1.20 |",
        "| GHCR MCP image | prepared | 0.1.20 |",
        "| GitHub Copilot coding-agent prep | prepared |",
        "| VS Code Marketplace / Open VSX | prepared |",
        "| Homebrew | deferred |",
        "| Chocolatey | n/a |",
    ] {
        assert!(
            release_status.contains(row),
            "source release status missing {row}"
        );
        assert!(
            site_release_status.contains(row),
            "docs-site release status missing {row}"
        );
    }
    assert!(requirements.contains("69, 70"));
    assert!(requirements.contains("repeatable refresh cadence"));
}
