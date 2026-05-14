# docs-parity.ps1 - Verify all archival docs have Astro site counterparts

$ErrorActionPreference = 'Stop'

$archivalDir = 'docs/src'
$siteDir = 'docs-site/src/content/docs'
$summaryFile = 'SUMMARY.md'

$archivalFiles = Get-ChildItem -Path $archivalDir -Filter *.md -Recurse |
    Where-Object { $_.Name -ne $summaryFile }

$missing = @()
foreach ($file in $archivalFiles) {
    $relativePath = $file.FullName.Substring((Get-Location).Path.Length + 1)
    $sitePath = $relativePath -replace [regex]::Escape($archivalDir), $siteDir
    if (-not (Test-Path $sitePath)) {
        $missing += $relativePath
        Write-Warning "MISSING: $relativePath has no Astro site counterpart"
    }
}

Write-Host "Checked $($archivalFiles.Count) files"
Write-Host "Missing: $($missing.Count)"

if ($missing.Count -gt 0) {
    Write-Host "Missing files:" -ForegroundColor Red
    $missing | ForEach-Object { Write-Host "  $_" }
    exit 1
} else {
    Write-Host "All pages have Astro site counterparts" -ForegroundColor Green
    exit 0
}
