param(
    [string]$ReleaseTag = ""
)

$ErrorActionPreference = 'Stop'

if ([string]::IsNullOrWhiteSpace($ReleaseTag)) {
    $ReleaseTag = "v0.1.20"
}

Write-Host "Release tag: $ReleaseTag"
Write-Host "Running release-surface evidence refresh checks..."
& (Join-Path $PSScriptRoot 'verify-release-surface-refresh.ps1') -ReleaseTag $ReleaseTag

Write-Host "Release checks executed by this helper:"
Write-Host " - powershell -NoProfile -ExecutionPolicy Bypass -File scripts\verify-release-surface-refresh.ps1 -ReleaseTag $ReleaseTag"
Write-Host "Manual release follow-up checklist:"
Write-Host " - cargo package --locked"
Write-Host " - cargo publish --dry-run --locked"
Write-Host " - cargo deny check advisories bans sources"
Write-Host " - cargo tree -d --locked --target x86_64-unknown-linux-gnu (secondary duplicate scan; deny.toml skips known wit-bindgen split)"
Write-Host " - verify clean tree"
Write-Host " - validate release artifacts, checksums, and registry metadata"
