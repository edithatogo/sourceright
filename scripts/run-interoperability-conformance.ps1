[CmdletBinding()]
param(
    [string]$CanonicalPath = "fixtures/interoperability/bibtex-basic.expected-csl.json",
    [string]$InputPath = "fixtures/interoperability/bibtex-basic.bib",
    [string]$OutputPath = "conductor/.tmp/citation-js-interoperability-report.md"
)

$ErrorActionPreference = "Stop"
$runnerRoot = Join-Path $PSScriptRoot "..\interop-runners"
if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
    throw "Node.js is required for the interoperability runner; install Node.js 20+ and retry."
}

Push-Location $runnerRoot
try {
    npm ci --ignore-scripts
    New-Item -ItemType Directory -Force .tmp | Out-Null
    node citation-js-runner.mjs "..\$InputPath" .tmp\citation-js.json
}
finally {
    Pop-Location
}

$outputDirectory = Split-Path -Parent $OutputPath
if ($outputDirectory) { New-Item -ItemType Directory -Force $outputDirectory | Out-Null }
$env:CARGO_TARGET_DIR = Join-Path ([System.IO.Path]::GetTempPath()) "sourceright-target"
$env:RUSTUP_TOOLCHAIN = "stable"
$env:CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER = $null
$oraclePath = Join-Path $runnerRoot ".tmp\citation-js.json"
cargo run --locked --bin interoperability-diff -- $CanonicalPath $oraclePath citation-js $OutputPath
