param(
    [string]$EvidencePath = "conductor/submission-packets/live-evidence.json",
    [string]$SchemaPath = "schemas/sourceright.live-submission-evidence.schema.json",
    [switch]$AllowTemplate
)

$ErrorActionPreference = "Stop"

if (-not (Test-Path -LiteralPath $SchemaPath)) {
    throw "Missing live submission evidence schema: $SchemaPath"
}

$schema = Get-Content -Raw -LiteralPath $SchemaPath | ConvertFrom-Json
if ($schema.'$id' -ne "sourceright.live-submission-evidence.v1") {
    throw "Unexpected live submission evidence schema id: $($schema.'$id')"
}

if (-not (Test-Path -LiteralPath $EvidencePath)) {
    Write-Host "live-submission-evidence: no evidence file found at $EvidencePath"
    Write-Host "live-submission-evidence: copy conductor/submission-packets/live-evidence.template.json after live submission evidence exists"
    exit 0
}

$evidence = Get-Content -Raw -LiteralPath $EvidencePath | ConvertFrom-Json

$allowedTopLevel = @('$schema', 'schema_version', 'generated_for', 'evidence')
foreach ($property in $evidence.PSObject.Properties.Name) {
    if ($allowedTopLevel -notcontains $property) {
        throw "Live evidence has additional top-level property: $property"
    }
}

if ($evidence.'$schema' -ne "sourceright.live-submission-evidence.v1") {
    throw "Unexpected live evidence `$schema: $($evidence.'$schema')"
}
if ($evidence.schema_version -ne "sourceright.live_submission_evidence.v1") {
    throw "Unexpected live evidence schema_version: $($evidence.schema_version)"
}

$allowedSurfaces = @(
    "smithery",
    "glama",
    "ojs-pkp",
    "arxiv-submit-ce",
    "arxiv-submission-core",
    "vscode-open-vsx",
    "claude-cowork",
    "codex-app",
    "github-copilot",
    "gemini-cli-extensions",
    "qwen-cli-extensions",
    "opencode",
    "cline"
)
$allowedPackets = @(
    "mcp-directories",
    "journal-platforms",
    "arxiv-upstream",
    "ai-client-extensions",
    "vscode-open-vsx"
)
$allowedStatuses = @("verified", "submitted", "accepted", "rejected", "deferred")
$allowedTypes = @(
    "public_listing",
    "api_result",
    "issue_url",
    "pull_request_url",
    "gallery_pr",
    "live_smoke_log",
    "submission_receipt"
)
$allowedGates = @("submission_ready", "submitted", "publicly_accepted")

$seen = New-Object System.Collections.Generic.HashSet[string]
foreach ($item in @($evidence.evidence)) {
    $allowedFields = @(
        "surface_id",
        "packet_id",
        "status",
        "url",
        "recorded_at",
        "evidence_type",
        "claim_boundary",
        "next_gate"
    )
    foreach ($property in $item.PSObject.Properties.Name) {
        if ($allowedFields -notcontains $property) {
            throw "Live evidence item $($item.surface_id) has additional property: $property"
        }
    }
    foreach ($field in $allowedFields) {
        if ([string]::IsNullOrWhiteSpace([string]$item.$field)) {
            throw "Live evidence item is missing $field"
        }
    }
    if ($allowedSurfaces -notcontains $item.surface_id) {
        throw "Unsupported live evidence surface: $($item.surface_id)"
    }
    if ($allowedPackets -notcontains $item.packet_id) {
        throw "Unsupported live evidence packet: $($item.packet_id)"
    }
    if ($allowedStatuses -notcontains $item.status) {
        throw "Unsupported live evidence status for $($item.surface_id): $($item.status)"
    }
    if ($allowedTypes -notcontains $item.evidence_type) {
        throw "Unsupported live evidence type for $($item.surface_id): $($item.evidence_type)"
    }
    if ($allowedGates -notcontains $item.next_gate) {
        throw "Unsupported live evidence next gate for $($item.surface_id): $($item.next_gate)"
    }
    if ($item.recorded_at -notmatch '^\d{4}-\d{2}-\d{2}$') {
        throw "Live evidence recorded_at must be YYYY-MM-DD for $($item.surface_id)"
    }
    if (-not $AllowTemplate -and $item.url -match 'REPLACE_WITH') {
        throw "Live evidence still has placeholder URL for $($item.surface_id)"
    }
    if ($item.next_gate -eq "publicly_accepted" -and $item.status -ne "accepted") {
        throw "Live evidence for $($item.surface_id) cannot target publicly_accepted unless status is accepted"
    }
    if ($item.status -eq "accepted" -and $item.evidence_type -notin @("public_listing", "api_result", "pull_request_url")) {
        throw "Accepted evidence for $($item.surface_id) needs public listing, API result, or accepted/merged PR evidence"
    }
    [void]$seen.Add($item.surface_id)
}

Write-Host "live-submission-evidence: $($seen.Count) surfaces with live evidence checked"
