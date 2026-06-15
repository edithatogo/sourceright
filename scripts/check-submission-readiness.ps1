param(
    [string]$InventoryPath = (Join-Path $PSScriptRoot '..' 'conductor' 'submission-inventory.json')
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
