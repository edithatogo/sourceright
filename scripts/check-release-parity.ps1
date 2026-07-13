[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
$release = Get-Content .github/workflows/release.yml -Raw
$dryRun = Get-Content .github/workflows/release-dry-run.yml -Raw

foreach ($needle in @(
    "cargo build --release --locked",
    "cargo package --locked",
    "cargo metadata --locked --format-version 1",
    "cargo deny check advisories bans sources",
    "cargo publish --dry-run --locked"
)) {
    if ($release -notmatch [regex]::Escape($needle)) { throw "release.yml is missing parity control: $needle" }
    if ($dryRun -notmatch [regex]::Escape($needle)) { throw "release-dry-run.yml is missing parity control: $needle" }
}

foreach ($workflow in @($release, $dryRun)) {
    if ($workflow -notmatch "actions/attest-build-provenance@") { throw "release path is missing build provenance attestation" }
}

Write-Host "Release parity contract passed."
