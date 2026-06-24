param(
    [string]$InventoryPath = "conductor/submission-requirements.json",
    [string]$PacketManifestPath = "conductor/submission-packets/manifest.json",
    [double]$MinimumHealth = 9.5
)

$ErrorActionPreference = 'Stop'

if (-not (Test-Path -LiteralPath $InventoryPath)) {
    throw "Missing submission requirements inventory: $InventoryPath"
}

$inventory = Get-Content -Raw -LiteralPath $InventoryPath | ConvertFrom-Json

if ([string]::IsNullOrWhiteSpace($inventory.'$schema')) {
    throw "Submission requirements inventory is missing `$schema"
}

$schemaPath = Join-Path (Split-Path -Parent $InventoryPath) $inventory.'$schema'
$schemaPath = [System.IO.Path]::GetFullPath($schemaPath)
if (-not (Test-Path -LiteralPath $schemaPath)) {
    throw "Missing submission requirements schema: $schemaPath"
}

$schema = Get-Content -Raw -LiteralPath $schemaPath | ConvertFrom-Json
if ($schema.'$id' -ne $inventory.'$schema') {
    throw "Submission requirements schema id $($schema.'$id') does not match inventory `$schema $($inventory.'$schema')"
}

$allowedTopLevel = @('$schema', 'schema_version', 'repo_health_target', 'generated_for', 'claim_rule', 'approval_rule', 'evidence_gates', 'self_improving_controls', 'surfaces')
foreach ($property in $inventory.PSObject.Properties.Name) {
    if ($allowedTopLevel -notcontains $property) {
        throw "Submission requirements inventory has additional top-level property: $property"
    }
}

$allowedControls = @('readiness_script', 'ci_workflow', 'policy_test', 'packet_manifest', 'health_target', 'agents_and_skills')
foreach ($property in $inventory.self_improving_controls.PSObject.Properties.Name) {
    if ($allowedControls -notcontains $property) {
        throw "Submission requirements self_improving_controls has additional property: $property"
    }
}

if ($inventory.schema_version -ne "sourceright.submission_requirements.v1") {
    throw "Unexpected submission requirements schema: $($inventory.schema_version)"
}

if ([double]$inventory.repo_health_target -lt $MinimumHealth) {
    throw "Repo health target $($inventory.repo_health_target) is below required $MinimumHealth"
}

if ([string]::IsNullOrWhiteSpace($inventory.self_improving_controls.packet_manifest)) {
    throw "Submission requirements inventory is missing self_improving_controls.packet_manifest"
}

if ($inventory.self_improving_controls.packet_manifest -ne $PacketManifestPath) {
    throw "Packet manifest path mismatch: inventory has $($inventory.self_improving_controls.packet_manifest), verifier expected $PacketManifestPath"
}

$requiredGates = @(
    "requirements_searched",
    "contracted",
    "hardened_local_package",
    "submission_ready",
    "submitted",
    "publicly_accepted"
)

$requiredSurfaces = @(
    "official-mcp-registry",
    "smithery",
    "glama",
    "zotero",
    "endnote",
    "ojs-pkp",
    "arxiv-submit-ce",
    "arxiv-submission-core",
    "claude-cowork",
    "codex-app",
    "github-copilot",
    "gemini-cli-extensions",
    "qwen-cli-extensions",
    "vscode-open-vsx",
    "opencode",
    "cline"
)

$surfaceById = @{}
foreach ($surface in $inventory.surfaces) {
    if ([string]::IsNullOrWhiteSpace($surface.id)) {
        throw "A submission surface is missing id"
    }
    if ($surfaceById.ContainsKey($surface.id)) {
        throw "Duplicate submission surface id: $($surface.id)"
    }
    $surfaceById[$surface.id] = $surface

    $allowedSurfaceFields = @(
        "id",
        "name",
        "category",
        "owning_tracks",
        "current_state",
        "required_artifact",
        "submission_target",
        "requirements_sources",
        "package_contract",
        "gates",
        "approval_required",
        "external_submission_allowed",
        "blockers",
        "maintenance_notes"
    )
    foreach ($property in $surface.PSObject.Properties.Name) {
        if ($allowedSurfaceFields -notcontains $property) {
            throw "Surface $($surface.id) has additional property: $property"
        }
    }

    foreach ($field in @("name", "category", "current_state", "required_artifact", "submission_target", "package_contract")) {
        if ([string]::IsNullOrWhiteSpace($surface.$field)) {
            throw "Surface $($surface.id) is missing $field"
        }
    }

    if (-not $surface.owning_tracks -or $surface.owning_tracks.Count -eq 0) {
        throw "Surface $($surface.id) has no owning tracks"
    }

    if (-not $surface.requirements_sources -or $surface.requirements_sources.Count -eq 0) {
        throw "Surface $($surface.id) has no requirements sources"
    }

    foreach ($source in $surface.requirements_sources) {
        $allowedSourceFields = @("label", "kind", "url_or_path", "status", "retrieved_at")
        foreach ($property in $source.PSObject.Properties.Name) {
            if ($allowedSourceFields -notcontains $property) {
                throw "Surface $($surface.id) requirements source has additional property: $property"
            }
        }
        foreach ($field in @("label", "kind", "url_or_path", "status")) {
            if ([string]::IsNullOrWhiteSpace($source.$field)) {
                throw "Surface $($surface.id) has an incomplete requirements source"
            }
        }
        if ($source.status -match 'to-search|to-refresh') {
            throw "Surface $($surface.id) has unsearched requirements source: $($source.label)"
        }
        if ([string]::IsNullOrWhiteSpace($source.retrieved_at)) {
            throw "Surface $($surface.id) requirements source is missing retrieved_at: $($source.label)"
        }
    }

    foreach ($property in $surface.gates.PSObject.Properties.Name) {
        if ($requiredGates -notcontains $property) {
            throw "Surface $($surface.id) gates has additional property: $property"
        }
    }

    foreach ($gate in $requiredGates) {
        if ($null -eq $surface.gates.$gate) {
            throw "Surface $($surface.id) is missing gate $gate"
        }
    }

    if ($surface.external_submission_allowed -and -not $surface.approval_required) {
        throw "Surface $($surface.id) allows external submission without approval"
    }

    for ($i = 1; $i -lt $requiredGates.Count; $i++) {
        $gate = $requiredGates[$i]
        $priorGate = $requiredGates[$i - 1]
        if ($surface.gates.$gate -and -not $surface.gates.$priorGate) {
            throw "Surface $($surface.id) cannot set $gate before $priorGate"
        }
    }

    $blockers = @($surface.blockers)
    if (($surface.gates.submission_ready -or $surface.gates.submitted -or $surface.gates.publicly_accepted) -and $blockers.Count -gt 0) {
        throw "Surface $($surface.id) cannot claim readiness or later gates while blockers remain"
    }
}

foreach ($id in $requiredSurfaces) {
    if (-not $surfaceById.ContainsKey($id)) {
        throw "Missing required submission surface: $id"
    }
}

if (-not (Test-Path -LiteralPath $PacketManifestPath)) {
    throw "Missing submission packet manifest: $PacketManifestPath"
}

$packetManifest = Get-Content -Raw -LiteralPath $PacketManifestPath | ConvertFrom-Json
if ([string]::IsNullOrWhiteSpace($packetManifest.'$schema')) {
    throw "Submission packet manifest is missing `$schema"
}
$packetSchemaPath = "schemas/sourceright.submission-packets.schema.json"
if (-not (Test-Path -LiteralPath $packetSchemaPath)) {
    throw "Missing submission packet manifest schema: $packetSchemaPath"
}
$packetSchema = Get-Content -Raw -LiteralPath $packetSchemaPath | ConvertFrom-Json
if ($packetSchema.'$id' -ne $packetManifest.'$schema') {
    throw "Submission packet schema id $($packetSchema.'$id') does not match manifest `$schema $($packetManifest.'$schema')"
}

$allowedPacketManifestFields = @('$schema', 'schema_version', 'generated_for', 'approval_rule', 'packets')
foreach ($property in $packetManifest.PSObject.Properties.Name) {
    if ($allowedPacketManifestFields -notcontains $property) {
        throw "Submission packet manifest has additional top-level property: $property"
    }
}

if ($packetManifest.schema_version -ne "sourceright.submission_packets.v1") {
    throw "Unexpected submission packet manifest schema: $($packetManifest.schema_version)"
}
if ($packetManifest.approval_rule -notmatch "explicit approval") {
    throw "Submission packet manifest must keep explicit approval in the approval rule"
}

$requiredPackets = @(
    "mcp-directories",
    "citation-managers",
    "journal-platforms",
    "arxiv-upstream",
    "ai-client-extensions",
    "vscode-open-vsx",
    "agent-workflow"
)

$allowedPacketStatuses = @("blocked", "active-control", "ready-local", "submitted", "accepted")

$packetById = @{}
foreach ($packet in $packetManifest.packets) {
    if ([string]::IsNullOrWhiteSpace($packet.id)) {
        throw "A submission packet is missing id"
    }
    if ($packetById.ContainsKey($packet.id)) {
        throw "Duplicate submission packet id: $($packet.id)"
    }
    $packetById[$packet.id] = $packet

    $allowedPacketFields = @(
        "id",
        "path",
        "owning_tracks",
        "surfaces",
        "status",
        "approval_required",
        "local_validation",
        "blockers"
    )
    foreach ($property in $packet.PSObject.Properties.Name) {
        if ($allowedPacketFields -notcontains $property) {
            throw "Packet $($packet.id) has additional property: $property"
        }
    }

    foreach ($field in @("path", "status")) {
        if ([string]::IsNullOrWhiteSpace($packet.$field)) {
            throw "Packet $($packet.id) is missing $field"
        }
    }
    if ($allowedPacketStatuses -notcontains $packet.status) {
        throw "Packet $($packet.id) has unsupported status: $($packet.status)"
    }
    if (-not $packet.owning_tracks -or @($packet.owning_tracks).Count -eq 0) {
        throw "Packet $($packet.id) has no owning tracks"
    }
    if (-not (Test-Path -LiteralPath $packet.path)) {
        throw "Packet $($packet.id) references missing path: $($packet.path)"
    }
    $packetText = Get-Content -Raw -LiteralPath $packet.path
    if ($packetText -notmatch "(?m)^## Blockers\s*$") {
        throw "Packet $($packet.id) markdown is missing a Blockers section"
    }
    if ($packet.id -ne "agent-workflow") {
        foreach ($section in @("Requirements Evidence", "Local Gates", "Draft", "Approval Gate")) {
            if ($packetText -notmatch "(?m)^## .*$section") {
                throw "Packet $($packet.id) markdown is missing section containing: $section"
            }
        }
    }
    if (-not $packet.approval_required) {
        throw "Packet $($packet.id) is not approval-gated"
    }
    if (-not $packet.local_validation -or @($packet.local_validation).Count -eq 0) {
        throw "Packet $($packet.id) has no local validation commands or checks"
    }
    if ($packet.status -eq "blocked" -and @($packet.blockers).Count -eq 0) {
        throw "Packet $($packet.id) is blocked but has no blockers"
    }
    foreach ($blocker in @($packet.blockers)) {
        if ($packetText -notlike "*$blocker*") {
            throw "Packet $($packet.id) markdown does not mention blocker: $blocker"
        }
    }
    if (@($packet.blockers).Count -eq 0 -and $packetText -notmatch "(?m)^None\.\s*$") {
        throw "Packet $($packet.id) markdown should record 'None.' when no blockers remain"
    }
    if (($packet.status -eq "submitted" -or $packet.status -eq "accepted") -and @($packet.blockers).Count -gt 0) {
        throw "Packet $($packet.id) cannot be $($packet.status) while blockers remain"
    }
    foreach ($surfaceId in @($packet.surfaces)) {
        if (-not $surfaceById.ContainsKey($surfaceId)) {
            throw "Packet $($packet.id) references unknown surface: $surfaceId"
        }
    }
}

foreach ($packetId in $requiredPackets) {
    if (-not $packetById.ContainsKey($packetId)) {
        throw "Missing required submission packet: $packetId"
    }
}

$packetSurfaceIds = @(
    $packetManifest.packets |
        ForEach-Object { @($_.surfaces) } |
        Where-Object { -not [string]::IsNullOrWhiteSpace($_) } |
        Sort-Object -Unique
)

foreach ($id in $requiredSurfaces) {
    if (-not ($packetSurfaceIds -contains $id)) {
        throw "Submission surface $id is not covered by any packet"
    }
}

$blocked = @($inventory.surfaces | Where-Object { @($_.blockers).Count -gt 0 } | ForEach-Object { $_.id })
$toSearch = @(
    $inventory.surfaces |
        ForEach-Object {
            $surface = $_
            @($surface.requirements_sources) |
                Where-Object { $_.status -match 'to-search|to-refresh' } |
                ForEach-Object { "$($surface.id):$($_.label)=$($_.status)" }
        }
)

Write-Host "submission-readiness: $($surfaceById.Count) surfaces checked; repo health target $($inventory.repo_health_target)"
Write-Host "submission-readiness packets checked: $($packetById.Count)"
Write-Host "submission-readiness blocked surfaces: $($blocked -join ', ')"
Write-Host "submission-readiness sources needing search or refresh: $($toSearch -join '; ')"
