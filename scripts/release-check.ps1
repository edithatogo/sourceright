param(
    [string]$ReleaseTag = ""
)

$ErrorActionPreference = 'Stop'

Write-Host "Release tag: $ReleaseTag"
Write-Host "Required checks:"
Write-Host " - cargo package --locked"
Write-Host " - cargo publish --dry-run --locked"
Write-Host " - cargo deny check advisories bans sources"
Write-Host " - cargo tree -d --locked --target x86_64-unknown-linux-gnu (allow known wit-bindgen split)"
Write-Host " - verify clean tree"
Write-Host " - validate release artifacts, checksums, and registry metadata"
