param(
    [string]$ReleaseTag = ""
)

$ErrorActionPreference = 'Stop'

Write-Host "Release tag: $ReleaseTag"
Write-Host "Required checks:"
Write-Host " - cargo package --locked"
Write-Host " - cargo publish --dry-run --locked"
Write-Host " - cargo deny check advisories bans sources duplicates"
Write-Host " - verify clean tree"
Write-Host " - validate release artifacts, checksums, and registry metadata"
