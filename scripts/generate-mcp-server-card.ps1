<#
.SYNOPSIS
    Regenerates the checked-in MCP server card from the live Rust MCP surface.
.DESCRIPTION
    Writes:
      - mcp/server-card.json
      - docs-site/src/data/mcp-server-card.json

    Smithery URL publish scans `/.well-known/mcp/server-card.json` on the published
    homepage (https://edithatogo.github.io/sourceright/).
.EXAMPLE
    pwsh -File scripts/generate-mcp-server-card.ps1
.EXAMPLE
    pwsh -File scripts/generate-mcp-server-card.ps1 -BinaryPath C:\tmp\sourceright-target-live\x86_64-pc-windows-gnu\release\sourceright.exe
#>
param(
    [string]$BinaryPath = ""
)

$ErrorActionPreference = "Stop"
$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")

if ([string]::IsNullOrWhiteSpace($BinaryPath)) {
    $BinaryPath = Join-Path $repoRoot "target/release/sourceright.exe"
    if (-not (Test-Path -LiteralPath $BinaryPath)) {
        $BinaryPath = Join-Path $repoRoot "target/release/sourceright"
    }
}

$binary = Resolve-Path -LiteralPath $BinaryPath
$cardJson = & $binary mcp server-card --json
if ($LASTEXITCODE -ne 0) {
    throw "sourceright mcp server-card failed with exit code $LASTEXITCODE"
}

$null = $cardJson | ConvertFrom-Json

$repoCardPath = Join-Path $repoRoot "mcp/server-card.json"
$docsDataDir = Join-Path $repoRoot "docs-site/src/data"
$docsDataPath = Join-Path $docsDataDir "mcp-server-card.json"

New-Item -ItemType Directory -Force -Path (Split-Path -Parent $repoCardPath) | Out-Null
New-Item -ItemType Directory -Force -Path $docsDataDir | Out-Null

$prettyCard = & $binary mcp server-card
if ($LASTEXITCODE -ne 0) {
    throw "sourceright mcp server-card failed with exit code $LASTEXITCODE"
}

Set-Content -LiteralPath $repoCardPath -Value $prettyCard -Encoding utf8NoBOM
Set-Content -LiteralPath $docsDataPath -Value $prettyCard -Encoding utf8NoBOM

Write-Host "Wrote MCP server card to:"
Write-Host "  $repoCardPath"
Write-Host "  $docsDataPath"
Write-Host "  (docs-site prerender route: src/pages/.well-known/mcp/server-card.json.ts)"
