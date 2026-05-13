param(
    [string]$Repo = "C:\Users\60217257\OneDrive - Flinders\repos\sourceright",
    [string]$Zip = "C:\Users\60217257\OneDrive - NSW Health Department\Downloads\sourceright_market_readiness_codex_plan_v6.zip"
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

if (-not (Test-Path $Repo)) {
    throw "Repo path does not exist: $Repo"
}
if (-not (Test-Path $Zip)) {
    throw "Zip path does not exist: $Zip"
}

Set-Location $Repo
New-Item -ItemType Directory -Force -Path ".codex-plan" | Out-Null
New-Item -ItemType Directory -Force -Path ".codex-plan\v6" | Out-Null

$destZip = ".codex-plan\sourceright_market_readiness_codex_plan_v6.zip"
Copy-Item -Path $Zip -Destination $destZip -Force

Expand-Archive -Path $destZip -DestinationPath ".codex-plan\v6" -Force

$template = ".codex-plan\v6\sourceright_market_readiness_codex_plan_v6\AGENTS.md.template"
if (-not (Test-Path $template)) {
    throw "Could not find AGENTS.md.template at $template"
}

if (-not (Test-Path "AGENTS.md")) {
    Copy-Item $template "AGENTS.md"
    Write-Host "Created AGENTS.md from template."
} else {
    Copy-Item "AGENTS.md" "AGENTS.md.backup" -Force
    Write-Host "AGENTS.md already exists. Backed it up to AGENTS.md.backup. Please merge template guidance manually."
}

Write-Host "Plan unpacked under .codex-plan\v6"
Write-Host "Next: run codex from repo root and paste USE_THIS_PROMPT.md or codex/prompts/00-bootstrap-audit.md"
