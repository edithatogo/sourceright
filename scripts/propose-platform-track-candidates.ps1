param(
    [string]$RegistryPath = "conductor/platform-registry.json",
    [switch]$CandidatesOnly
)

$ErrorActionPreference = "Stop"

$result = [ordered]@{
    pass                  = $true
    schema_version        = $null
    candidate_track_count = 0
    errors                = [System.Collections.Generic.List[string]]::new()
    candidate_tracks      = [System.Collections.Generic.List[object]]::new()
}

function Add-Error {
    param([string]$Message)
    $result.pass = $false
    $result.errors.Add($Message)
}

function Require-Property {
    param(
        [object]$Object,
        [string]$Property,
        [string]$Context
    )
    if (-not ($Object.PSObject.Properties.Name -contains $Property)) {
        Add-Error "$Context missing required property '$Property'"
        return $false
    }
    return $true
}

try {
    $registry = Get-Content -LiteralPath $RegistryPath -Raw -Encoding utf8 | ConvertFrom-Json
} catch {
    Add-Error "Failed to read or parse platform registry: $_"
    $result | ConvertTo-Json -Depth 20
    exit 1
}

$result.schema_version = $registry.schema_version

foreach ($field in @("schema_version", "`$schema", "policy", "candidate_track_template", "platforms")) {
    Require-Property $registry $field "registry" | Out-Null
}

if ($registry.policy.ad_hoc_notes_allowed -ne $false) {
    Add-Error "platform opportunities must not fall back to ad hoc notes"
}
if ($registry.policy.candidate_track_required -ne $true) {
    Add-Error "candidate tracks must be required for every platform opportunity"
}
if ($registry.policy.human_review_required -ne $true) {
    Add-Error "human review must remain required"
}
if ($registry.policy.auto_open_tracks -ne $false) {
    Add-Error "registry must not auto-open tracks"
}

$requiredCandidateFields = @($registry.candidate_track_template.required_fields)
if ($requiredCandidateFields.Count -eq 0) {
    Add-Error "candidate_track_template.required_fields must not be empty"
}

$seenPlatforms = [System.Collections.Generic.HashSet[string]]::new()
$seenSlugs = [System.Collections.Generic.HashSet[string]]::new()

foreach ($platform in $registry.platforms) {
    foreach ($field in @("platform_id", "name", "family", "owner_track", "status", "evidence_sources", "capabilities", "blockers", "confidence", "approval_state", "candidate_track")) {
        Require-Property $platform $field "platform" | Out-Null
    }

    if (-not $seenPlatforms.Add([string]$platform.platform_id)) {
        Add-Error "duplicate platform_id '$($platform.platform_id)'"
    }

    $ownerTrackPath = Join-Path "conductor/tracks" $platform.owner_track
    if (-not (Test-Path -LiteralPath $ownerTrackPath -PathType Container)) {
        Add-Error "platform '$($platform.platform_id)' references missing owner_track '$($platform.owner_track)'"
    }

    if ($platform.approval_state -ne "needs-human-review") {
        Add-Error "platform '$($platform.platform_id)' must keep approval_state needs-human-review"
    }
    if ([double]$platform.confidence -lt 0.0 -or [double]$platform.confidence -gt 1.0) {
        Add-Error "platform '$($platform.platform_id)' confidence must be between 0 and 1"
    }
    if ($platform.evidence_sources.Count -eq 0) {
        Add-Error "platform '$($platform.platform_id)' must contain at least one evidence source"
    }
    if ($platform.blockers.Count -eq 0) {
        Add-Error "platform '$($platform.platform_id)' must contain blockers"
    }

    $candidate = $platform.candidate_track
    foreach ($field in $requiredCandidateFields) {
        Require-Property $candidate $field "candidate_track '$($platform.platform_id)'" | Out-Null
    }

    if (-not ([string]$candidate.track_slug).StartsWith("candidate-")) {
        Add-Error "candidate track for '$($platform.platform_id)' must use candidate- slug"
    }
    if (-not $seenSlugs.Add([string]$candidate.track_slug)) {
        Add-Error "duplicate candidate track slug '$($candidate.track_slug)'"
    }
    if ($candidate.proposed_status -ne $registry.candidate_track_template.default_status) {
        Add-Error "candidate track '$($candidate.track_slug)' must use default candidate status"
    }
    if ($candidate.human_review_required -ne $true) {
        Add-Error "candidate track '$($candidate.track_slug)' must require human review"
    }
    if ($candidate.auto_open -ne $false) {
        Add-Error "candidate track '$($candidate.track_slug)' must not auto-open"
    }
    if ($candidate.scope.Count -eq 0 -or $candidate.acceptance_gates.Count -eq 0) {
        Add-Error "candidate track '$($candidate.track_slug)' must include scope and acceptance gates"
    }

    $result.candidate_tracks.Add([ordered]@{
        platform_id           = $platform.platform_id
        platform_name         = $platform.name
        owner_track           = $platform.owner_track
        track_slug            = $candidate.track_slug
        title                 = $candidate.title
        proposed_status       = $candidate.proposed_status
        trigger               = $candidate.trigger
        scope                 = @($candidate.scope)
        acceptance_gates      = @($candidate.acceptance_gates)
        human_review_required = $candidate.human_review_required
        auto_open             = $candidate.auto_open
        approval_state        = $platform.approval_state
        confidence            = $platform.confidence
        blockers              = @($platform.blockers)
    }) | Out-Null
}

$result.candidate_track_count = $result.candidate_tracks.Count

if ($CandidatesOnly) {
    $result.candidate_tracks | ConvertTo-Json -Depth 20
} else {
    $result | ConvertTo-Json -Depth 20
}

if (-not $result.pass) {
    exit 1
}
