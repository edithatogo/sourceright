param(
    [string]$BinaryPath = "",
    [string]$Platform = "win32"
)

$ErrorActionPreference = "Stop"
$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")

if ([string]::IsNullOrWhiteSpace($BinaryPath)) {
    $BinaryPath = Join-Path $env:USERPROFILE ".cargo\bin\sourceright.exe"
}

Write-Host "=== VSIX ==="
& (Join-Path $PSScriptRoot "build-vscode-vsix.ps1") | Out-Null

Write-Host "=== Smithery / Claude MCPB ($Platform) ==="
$mcpbOut = Join-Path $repoRoot "dist/sourceright-claude-desktop-0.1.20-$Platform.mcpb"
& (Join-Path $PSScriptRoot "build-smithery-mcpb.ps1") `
    -BinaryPath $BinaryPath `
    -Platform $Platform `
    -OutputPath $mcpbOut | Out-Null

Write-Host "=== Host package manifest ==="
@{
    schema = "sourceright.host_packages_build.v1"
    vsix = Join-Path $repoRoot "dist/edithatogo.sourceright-0.1.20.vsix"
    claude_mcpb = $mcpbOut
    gemini_extension = Join-Path $repoRoot "extensions/gemini-sourceright"
    qwen_extension = Join-Path $repoRoot "extensions/qwen-sourceright"
    opencode_extension = Join-Path $repoRoot "extensions/opencode-sourceright"
    codex_package = Join-Path $repoRoot "packages/codex-sourceright-mcp"
    copilot_package = Join-Path $repoRoot "packages/copilot-sourceright"
    cline_install = Join-Path $repoRoot "mcp/llms-install.md"
} | ConvertTo-Json -Depth 4
