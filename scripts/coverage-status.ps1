param(
    [int]$CoverageMinimum = 85,
    [string]$CoverageMetric = "branch",
    [string]$Outcome = "passed",
    [string]$OutputPath = "coverage-status.md"
)

$runUrl = if ($env:GITHUB_SERVER_URL -and $env:GITHUB_REPOSITORY -and $env:GITHUB_RUN_ID) {
    "$($env:GITHUB_SERVER_URL)/$($env:GITHUB_REPOSITORY)/actions/runs/$($env:GITHUB_RUN_ID)"
} else {
    "local-run"
}

$lines = @(
    "# Coverage status",
    "",
    "- Outcome: $Outcome",
    "- Coverage metric: $CoverageMetric",
    "- Coverage floor: $CoverageMinimum",
    "- Workflow run: $runUrl",
    "- Supported runner: ubuntu-latest",
    "- Tooling: cargo llvm-cov",
    "- Evidence: summary artifact and CI gate"
)

Set-Content -LiteralPath $OutputPath -Value $lines -Encoding utf8
