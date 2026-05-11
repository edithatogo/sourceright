param(
    [string]$Surface = "release",
    [string]$Outcome = "published",
    [string]$Tag = $env:GITHUB_REF_NAME,
    [string]$OutputPath = "release-status.md"
)

$runUrl = if ($env:GITHUB_SERVER_URL -and $env:GITHUB_REPOSITORY -and $env:GITHUB_RUN_ID) {
    "$($env:GITHUB_SERVER_URL)/$($env:GITHUB_REPOSITORY)/actions/runs/$($env:GITHUB_RUN_ID)"
} else {
    "local-run"
}

$releaseUrl = if ($env:GITHUB_SERVER_URL -and $env:GITHUB_REPOSITORY -and $Tag) {
    "$($env:GITHUB_SERVER_URL)/$($env:GITHUB_REPOSITORY)/releases/tag/$Tag"
} else {
    "unavailable"
}

$tagValue = if ($Tag) { $Tag } else { "local" }
$imageRef = "ghcr.io/edithatogo/sourceright-mcp:$tagValue"

$lines = @(
    "# Release status",
    "",
    "- Surface: $Surface",
    "- Outcome: $Outcome",
    "- Tag: $tagValue",
    "- Workflow run: $runUrl",
    "- GitHub release: $releaseUrl",
    "- OCI image: $imageRef",
    "- Crate publish: tag-triggered publish-crate workflow",
    "- MCP registry publish: tag-triggered publish-mcp-registry workflow",
    "- Evidence: checksums, attestations, and clean-tree validation"
)

Set-Content -LiteralPath $OutputPath -Value $lines -Encoding utf8
