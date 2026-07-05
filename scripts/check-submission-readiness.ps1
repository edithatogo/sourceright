param(
    [string]$InventoryPath = "$PSScriptRoot\..\conductor\submission-inventory.json"
)

$ErrorActionPreference = 'Stop'

$RequiredSurfaceCount = 10
$MinimumEvidenceLevel = "contracted"
$KnownEvidenceLevels = @("contracted", "fixture-backed", "submitted", "verified")

$result = @{
    pass          = $true
    schema_valid  = $true
    surface_count = 0
    health_score  = 0.0
    errors        = [System.Collections.Generic.List[string]]::new()
    warnings      = [System.Collections.Generic.List[string]]::new()
}

function Add-Error {
    param([string]$Message)
    $result.pass = $false
    $result.errors.Add($Message)
}

function Add-SchemaError {
    param([string]$Message)
    $result.schema_valid = $false
    Add-Error $Message
}

function Add-Warning {
    param([string]$Message)
    $result.warnings.Add($Message)
}

try {
    $resolvedPath = Resolve-Path $InventoryPath -ErrorAction Stop
    Write-Verbose "Reading inventory from: $resolvedPath"
    $raw = Get-Content -LiteralPath $resolvedPath -Raw -Encoding utf8
    $inventory = $raw | ConvertFrom-Json
} catch {
    Add-SchemaError "Failed to read or parse submission-inventory.json: $_"
    $result.health_score = 0.0
    $result | ConvertTo-Json -Depth 10
    exit 1
}

$requiredTopFields = @("schema", "generated_at", "surfaces", "health_score", "health_target", "surface_count")
foreach ($property in $inventory.PSObject.Properties.Name) {
    if ($requiredTopFields -notcontains $property) {
        Add-SchemaError "Unexpected top-level field: '$property'"
    }
}
foreach ($field in $requiredTopFields) {
    if (-not ($inventory.PSObject.Properties.Name -contains $field)) {
        Add-SchemaError "Missing required top-level field: '$field'"
    }
}

if (-not ($inventory.surfaces -is [array])) {
    Add-SchemaError "Top-level field 'surfaces' must be an array"
}

$surfaceCount = $inventory.surfaces.Count
$result.surface_count = $surfaceCount
if ($surfaceCount -ne $RequiredSurfaceCount) {
    Add-SchemaError "Expected $RequiredSurfaceCount surfaces, found $surfaceCount"
}
if ($inventory.surface_count -ne $surfaceCount) {
    Add-SchemaError "Stored surface_count ($($inventory.surface_count)) does not match actual surface count ($surfaceCount)"
}

$computedHealthSum = 0.0
$validSurfaceCount = 0
$seenTrackIds = [System.Collections.Generic.HashSet[string]]::new()

foreach ($surface in $inventory.surfaces) {
    $trackId = $surface.track_id
    if ([string]::IsNullOrWhiteSpace($trackId)) {
        Add-SchemaError "Surface missing track_id"
        continue
    }
    if (-not $seenTrackIds.Add($trackId)) {
        Add-SchemaError "Duplicate surface track_id: '$trackId'"
    }

    $requiredSurfaceFields = @("track_id", "name", "category", "readiness", "blockers", "last_updated")
    foreach ($property in $surface.PSObject.Properties.Name) {
        if ($requiredSurfaceFields -notcontains $property) {
            Add-SchemaError "Surface '$trackId' has unexpected field: '$property'"
        }
    }
    foreach ($field in $requiredSurfaceFields) {
        if (-not ($surface.PSObject.Properties.Name -contains $field)) {
            Add-SchemaError "Surface '$trackId' missing required field: '$field'"
        }
    }

    $trackPath = Join-Path $PSScriptRoot "..\conductor\tracks\$trackId"
    if (-not (Test-Path -LiteralPath $trackPath -PathType Container)) {
        Add-SchemaError "Surface '$trackId' does not map to an on-disk conductor track"
    } else {
        foreach ($requiredTrackFile in @("metadata.json", "plan.md", "test-matrix.md")) {
            $trackFilePath = Join-Path $trackPath $requiredTrackFile
            if (-not (Test-Path -LiteralPath $trackFilePath -PathType Leaf)) {
                Add-SchemaError "Surface '$trackId' missing required track file: '$requiredTrackFile'"
            }
        }
    }

    if ($null -eq $surface.readiness) {
        Add-SchemaError "Surface '$trackId' missing readiness object"
        continue
    }

    $requiredReadinessFields = @("gates_passed", "total_gates", "evidence_level", "health_contribution")
    foreach ($property in $surface.readiness.PSObject.Properties.Name) {
        if ($requiredReadinessFields -notcontains $property) {
            Add-SchemaError "Surface '$trackId' readiness has unexpected field: '$property'"
        }
    }
    foreach ($field in $requiredReadinessFields) {
        if (-not ($surface.readiness.PSObject.Properties.Name -contains $field)) {
            Add-SchemaError "Surface '$trackId' readiness missing field: '$field'"
        }
    }

    if ($surface.readiness.total_gates -le 0) {
        Add-Error "Surface '$trackId' has total_gates <= 0"
    }
    if ($surface.readiness.gates_passed -gt $surface.readiness.total_gates) {
        Add-Error "Surface '$trackId' has gates_passed greater than total_gates"
    }

    $hc = [double]$surface.readiness.health_contribution
    if ($hc -lt 0.0 -or $hc -gt 1.0) {
        Add-Error "Surface '$trackId' health_contribution $hc outside [0,1]"
    }

    if ($surface.readiness.total_gates -gt 0) {
        $expectedHealthContribution = [math]::Round(($surface.readiness.gates_passed / $surface.readiness.total_gates), 4)
        $actualHealthContribution = [math]::Round($hc, 4)
        if ([math]::Abs($expectedHealthContribution - $actualHealthContribution) -gt 0.0001) {
            Add-Error "Surface '$trackId' health_contribution $actualHealthContribution does not match gates_passed / total_gates ($expectedHealthContribution)"
        }
    }

    $evLevel = $surface.readiness.evidence_level
    if ($KnownEvidenceLevels -notcontains $evLevel) {
        Add-Error "Surface '$trackId' has unknown evidence level '$evLevel'"
    } elseif ([array]::IndexOf($KnownEvidenceLevels, $evLevel) -lt [array]::IndexOf($KnownEvidenceLevels, $MinimumEvidenceLevel)) {
        Add-Error "Surface '$trackId' evidence level '$evLevel' is below minimum '$MinimumEvidenceLevel'"
    }

    $computedHealthSum += $hc * 10.0
    $validSurfaceCount++
}

$computedHealthScore = [math]::Round(($computedHealthSum / [math]::Max($validSurfaceCount, 1)), 2)
$storedHealthScore = [math]::Round($inventory.health_score, 2)
if ($computedHealthScore -ne $storedHealthScore) {
    Add-Warning "Computed health score ($computedHealthScore) differs from stored ($storedHealthScore)"
}

$result.health_score = $computedHealthScore
$result | ConvertTo-Json -Depth 10

if (-not $result.pass) {
    exit 1
}
exit 0
