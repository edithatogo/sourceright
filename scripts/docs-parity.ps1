<#
.SYNOPSIS
    Checks content parity between archival docs/src/ Markdown files
    and the Astro docs-site/src/content/docs/ surface.
.DESCRIPTION
    Lists all .md files in docs/src/ (excluding SUMMARY.md), checks each
    has a counterpart in docs-site/src/content/docs/, and reports any
    missing files. Uses the same mapping as tests/docs_site_parity.rs.
.EXAMPLE
    pwsh -File scripts/docs-parity.ps1
#>

$ErrorActionPreference = "Stop"
$RepoRoot = Split-Path -Parent $PSScriptRoot

$ParityMap = @{
    "index.md"                          = "docs-site/src/content/docs/index.md"
    "installation.md"                   = "docs-site/src/content/docs/guides/installation.md"
    "workflow.md"                       = "docs-site/src/content/docs/guides/workflow.md"
    "quickstart.md"                     = "docs-site/src/content/docs/guides/quickstart.md"
    "feature-contract-matrix.md"        = "docs-site/src/content/docs/feature-contract-matrix.md"
    "design.md"                         = "docs-site/src/content/docs/design.md"
    "author-preflight-workflow.md"      = "docs-site/src/content/docs/guides/author-preflight-workflow.md"
    "editorial-triage-workflow.md"      = "docs-site/src/content/docs/guides/editorial-triage-workflow.md"
    "university-repository-workflow.md" = "docs-site/src/content/docs/guides/university-repository-workflow.md"
    "legal-citation-mode-workflow.md"   = "docs-site/src/content/docs/guides/legal-citation-mode-workflow.md"
    "legal-citation-audit-mcp.md"       = "docs-site/src/content/docs/guides/legal-citation-audit-mcp.md"
    "legacy-audit.md"                   = "docs-site/src/content/docs/legacy-audit.md"
    "cli.md"                            = "docs-site/src/content/docs/guides/cli.md"
    "mcp.md"                            = "docs-site/src/content/docs/guides/mcp.md"
    "mcp-server-plan.md"                = "docs-site/src/content/docs/mcp-server-plan.md"
    "providers.md"                      = "docs-site/src/content/docs/reference/providers.md"
    "plugins.md"                        = "docs-site/src/content/docs/plugins.md"
    "plugin-registry.md"                = "docs-site/src/content/docs/plugin-registry.md"
    "plugin-authoring.md"               = "docs-site/src/content/docs/plugin-authoring.md"
    "csl-model.md"                      = "docs-site/src/content/docs/csl-model.md"
    "verification-sidecar.md"           = "docs-site/src/content/docs/verification-sidecar.md"
    "schema-contracts.md"               = "docs-site/src/content/docs/schema-contracts.md"
    "artifact-schema-guide.md"          = "docs-site/src/content/docs/guides/artifact-schema-guide.md"
    "reporting.md"                      = "docs-site/src/content/docs/reference/reporting.md"
    "exports.md"                            = "docs-site/src/content/docs/exports.md"
    "benchmarks.md"                         = "docs-site/src/content/docs/guides/benchmarks.md"
    "citation-manager-integrations.md"      = "docs-site/src/content/docs/citation-manager-integrations.md"
    "zotero-plugin-install.md"              = "docs-site/src/content/docs/guides/zotero-plugin-install.md"
    "host-packaging.md"                     = "docs-site/src/content/docs/guides/host-packaging.md"
    "journal-integrations.md"               = "docs-site/src/content/docs/journal-integrations.md"
    "policy-engine.md"                      = "docs-site/src/content/docs/policy-engine.md"
    "style-and-recency.md"                  = "docs-site/src/content/docs/style-and-recency.md"
    "publishing.md"                         = "docs-site/src/content/docs/guides/publishing.md"
    "security-automation.md"                = "docs-site/src/content/docs/guides/security-automation.md"
    "release-runbook.md"                    = "docs-site/src/content/docs/guides/release-runbook.md"
    "release-status.md"                     = "docs-site/src/content/docs/release-status.md"
    "release-notes.md"                      = "docs-site/src/content/docs/guides/release-notes.md"
    "faq.md"                                = "docs-site/src/content/docs/guides/faq.md"
    "pilot-invitation.md"                   = "docs-site/src/content/docs/guides/pilot-invitation.md"
    "coverage-reporting.md"                 = "docs-site/src/content/docs/guides/coverage-reporting.md"
    "coverage-status.md"                    = "docs-site/src/content/docs/guides/coverage-status.md"
    "devsecops-automation-upgrade.md"        = "docs-site/src/content/docs/guides/devsecops-automation-upgrade.md"
    "operations-status.md"                  = "docs-site/src/content/docs/guides/operations-status.md"
    "docs-cutover.md"                       = "docs-site/src/content/docs/guides/docs-cutover.md"
    "live-provider-configuration.md"        = "docs-site/src/content/docs/guides/live-provider-configuration.md"
    "limitations.md"                        = "docs-site/src/content/docs/guides/limitations.md"
    "manual-review.md"                      = "docs-site/src/content/docs/manual-review.md"
    "legal-roadmap.md"                      = "docs-site/src/content/docs/legal-roadmap.md"
    "claim-provenance-roadmap.md"           = "docs-site/src/content/docs/claim-provenance-roadmap.md"
    "contributing.md"                       = "docs-site/src/content/docs/guides/contributing.md"

}


Write-Host "=== Docs Content Parity Check ===" -ForegroundColor Cyan
Write-Host "Archival source : docs/src/"
Write-Host "Astro site       : docs-site/src/content/docs/"
Write-Host ""

$missing = @()
$extras = @()
$ok = 0

$archivalDir = Join-Path $RepoRoot "docs/src"
$mdFiles = Get-ChildItem -Path $archivalDir -Filter "*.md" |
    Where-Object { $_.Name -ne "SUMMARY.md" }

foreach ($file in $mdFiles) {
    $fileName = $file.Name
    if (-not $ParityMap.ContainsKey($fileName)) {
        $missing += "NO MAPPING ENTRY for docs/src/$fileName"
        continue
    }
    $targetPath = Join-Path $RepoRoot $ParityMap[$fileName]
    if (-not (Test-Path $targetPath)) {
        $missing += "MISSING TARGET: $($ParityMap[$fileName]) (from docs/src/$fileName)"
        continue
    }
    $ok++
}

foreach ($key in $ParityMap.Keys) {
    $sourcePath = Join-Path $archivalDir $key
    if (-not (Test-Path $sourcePath)) {
        $extras += "ORPHAN MAPPING: docs/src/$key -> $($ParityMap[$key]) (source missing)"
    }
}

Write-Host " OK : $ok files with confirmed counterparts" -ForegroundColor Green

if ($extras.Count -gt 0) {
    Write-Host ""
    Write-Host " WARNINGS : $($extras.Count) orphaned mapping entries" -ForegroundColor Yellow
    foreach ($e in $extras) { Write-Host "   $e" -ForegroundColor Yellow }
}

if ($missing.Count -gt 0) {
    Write-Host ""
    Write-Host " FAILURES : $($missing.Count) missing files" -ForegroundColor Red
    foreach ($m in $missing) { Write-Host "   $m" -ForegroundColor Red }
}

Write-Host ""
Write-Host "=== Summary ===" -ForegroundColor Cyan
Write-Host "  Total archival .md files : $($mdFiles.Count)"
Write-Host "  Confirmed counterparts   : $ok"
Write-Host "  Missing / unmapped       : $($missing.Count)"
Write-Host "  Orphaned mappings        : $($extras.Count)"

if ($missing.Count -eq 0 -and $extras.Count -eq 0) {
    Write-Host ""
    Write-Host " PASS - All archival docs have Astro site counterparts." -ForegroundColor Green
    exit 0
} else {
    Write-Host ""
    Write-Host " FAIL - Parity gaps detected. See above for details." -ForegroundColor Red
    exit 1
}
