[CmdletBinding()]
param(
    [string]$CanonicalPath = "fixtures/interoperability/bibtex-basic.expected-csl.json",
    [string]$InputPath = "fixtures/interoperability/bibtex-basic.bib",
    [string]$OutputPath = "conductor/.tmp/citation-js-interoperability-report.md"
)

$ErrorActionPreference = "Stop"
$runnerRoot = Join-Path $PSScriptRoot "..\interop-runners"
$nodeRoot = "C:\Users\60217257\scoop\apps\nodejs\current"
$env:Path = "$nodeRoot;$nodeRoot\bin;$env:Path"

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
$env:CARGO_TARGET_DIR = "C:\tmp\sourceright-target"
$env:RUSTUP_TOOLCHAIN = "stable-x86_64-pc-windows-gnu"
$env:CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER = "gcc"
$oraclePath = Join-Path $runnerRoot ".tmp\citation-js.json"
cargo run --locked --bin interoperability-diff -- $CanonicalPath $oraclePath citation-js $OutputPath
