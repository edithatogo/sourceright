param(
    [int]$CoverageMinimum = 85,
    [string]$ReportPath = "coverage-report.txt"
)

$ErrorActionPreference = 'Stop'

$lines = @(
    "Coverage floor: $CoverageMinimum",
    "Coverage metric: branch",
    "Primary gate: cargo llvm-cov --locked --all-targets --summary-only --branch --fail-under-branches $CoverageMinimum",
    "Hook parity: pwsh -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1 -CoverageMinimum $CoverageMinimum"
)

$lines | Set-Content -LiteralPath $ReportPath -Encoding UTF8
Write-Host "Wrote coverage summary to $ReportPath"
