param(
    [switch]$DryRun
)

$ErrorActionPreference = "Stop"
$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")

function Try-Command($label, $scriptBlock) {
    Write-Host "`n--- $label ---"
    if ($DryRun) {
        Write-Host "[dry-run] skipped"
        return @{ status = "dry_run" }
    }
    try {
        & $scriptBlock
        return @{ status = "ok" }
    } catch {
        return @{ status = "failed"; error = $_.Exception.Message }
    }
}

$results = @{}

$results.vscode_marketplace = Try-Command "VS Code Marketplace (vsce)" {
    if (-not $env:VSCE_PAT) { throw "VSCE_PAT not set" }
    Push-Location (Join-Path $repoRoot "extensions/vscode-sourceright")
    npx --yes @vscode/vsce publish -p $env:VSCE_PAT
    Pop-Location
}

$results.open_vsx = Try-Command "Open VSX (ovsx)" {
    if (-not $env:OVSX_PAT) { throw "OVSX_PAT not set" }
    $vsix = Join-Path $repoRoot "dist/edithatogo.sourceright-0.1.20.vsix"
    npx --yes ovsx publish $vsix -p $env:OVSX_PAT
}

$results.cline_marketplace = Try-Command "Cline MCP Marketplace issue" {
    gh issue create `
        --repo cline/mcp-marketplace `
        --title "Sourceright MCP server" `
        --body-file (Join-Path $repoRoot "conductor/tracks/90-cline-mcp-marketplace-submission-and-acceptance/cline-marketplace-issue-body.md")
}

$results | ConvertTo-Json -Depth 4
