param(
    [string]$InventoryPath = "$PSScriptRoot\..\conductor\submission-inventory.json"
)

$ErrorActionPreference = 'Stop'

# --- Constants ---
$RequiredSurfaceCount = 10
$MinimumEvidenceLevel = "contracted"
$KnownEvidenceLevels = @("contracted", "fixture-backed", "submitted", "verified")

# --- Result accumulator ---
$result = @{
    pass         = $true
    health_score = 0.0
    errors       = [System.Collections.Generic.List[string]]::new()
    warnings     = [System.Collections.Generic.List[string]]::new()
}

function Add-Error {
    param([string]$Message)
    $result.pass = $false
    $result.errors.Add($Message)
}

function Add-Warning {
    param([string]$Message)
    $result.warnings.Add($Message)
}

# --- Resolve inventory path ---
$resolvedPath = Resolve-Path $InventoryPath -ErrorAction Stop
Write-Verbose "Reading inventory from: $resolvedPath"

# --- Read and parse inventory ---
try {
    $raw = Get-Content -LiteralPath $resolvedPath -Raw -Encoding utf8
    $inventory = $raw | ConvertFrom-Json
} catch {
    Add-Error "Failed to read or parse submission-inventory.json: $_"
    $result.health_score = 0.0
    $result | ConvertTo-Json -Depth 10
    exit 1
}

# --- 1. Schema: top-level required fields ---
$requiredTopFields = @("schema", "generated_at", "surfaces", "health_score", "health_target", "surface_count")
foreach ($field in $requiredTopFields) {
    if (-not ($inventory.PSObject.Properties.Name -contains $field)) {
        Add-Error "Missing required top-level field: '$field'"
    }
}

# --- 2. Surface count ---
$surfaceCount = $inventory.surfaces.Count
if ($surfaceCount -ne $RequiredSurfaceCount) {
    Add-Error "Expected $RequiredSurfaceCount surfaces, found $surfaceCount"
}

# --- 3. Surface-level validation ---
$computedHealthSum = 0.0
$validSurfaceCount = 0

foreach ($surface in $inventory.surfaces) {
    $trackId = $surface.track_id

    # Required fields
    $requiredSurfaceFields = @("track_id", "name", "category", "readiness", "blockers", "last_updated")
    foreach ($field in $requiredSurfaceFields) {
        if (-not ($surface.PSObject.Properties.Name -contains $field)) {
            Add-Error "Surface '$trackId' missing required field: '$field'"
        }
    }

    # Readiness sub-fields
    if ($surface.readiness) {
        $requiredReadinessFields = @("gates_passed", "total_gates", "evidence_level", "health_contribution")
        foreach ($field in $requiredReadinessFields) {
            if (-not ($surface.readiness.PSObject.Properties.Name -contains $field)) {
                Add-Error "Surface '$trackId' readiness missing field: '$field'"
            }
        }

        # Validate total_gates > 0
        if ($surface.readiness.total_gates -le 0) {
            Add-Error "Surface '$trackId' has total_gates <= 0"
        }

        # Validate health_contribution in [0,1]
        $hc = $surface.readiness.health_contribution
        if ($hc -lt 0.0 -or $hc -gt 1.0) {
            Add-Warning "Surface '$trackId' health_contribution $hc outside [0,1]"
        }

        # Validate evidence level
        $evLevel = $surface.readiness.evidence_level
        if ($KnownEvidenceLevels -notcontains $evLevel) {
            Add-Warning "Surface '$trackId' has unknown evidence level '$evLevel'"
        }

        # Accumulate for health score computation
        $computedHealthSum += $hc * 10.0
        $validSurfaceCount++
    }
}

# --- 4. Check computed health score matches inventory ---
$computedHealthScore = [math]::Round(($computedHealthSum / [math]::Max($validSurfaceCount, 1)), 2)
$storedHealthScore = [math]::Round($inventory.health_score, 2)

if ($computedHealthScore -ne $storedHealthScore) {
    Add-Warning "Computed health score ($computedHealthScore) differs from stored ($storedHealthScore)"
}

$result.health_score = $computedHealthScore

# --- 5. Output and exit ---
$output = $result | ConvertTo-Json -Depth 10
Write-Output $output

if (-not $result.pass) {
    exit 1
}

exit 0
