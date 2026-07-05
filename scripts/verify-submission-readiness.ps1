# $schema: submission-readiness-v1
param(
    [string]$InventoryPath = "conductor/submission-requirements.json"
)

$ErrorActionPreference = "Stop"

if (-not (Test-Path $InventoryPath)) {
    throw "missing inventory: $InventoryPath"
}

$inventory = Get-Content $InventoryPath -Raw | ConvertFrom-Json

function Test-GateOrder {
    param(
        [object]$Surface
    )

    $gates = $Surface.gates
    $order = @(
        "requirements_searched",
        "contracted",
        "hardened_local_package",
        "submission_ready",
        "submitted",
        "publicly_accepted"
    )

    $priorGate = $null
    foreach ($gate in $order) {
        if ($gates.$gate -and $priorGate -and -not $gates.$priorGate) {
            throw "cannot set $gate before $priorGate"
        }
        if ($gates.$gate) {
            $priorGate = $gate
        }
    }
}

$blockedSurfaces = @()
$sourcesNeedingSearchOrRefresh = @()

foreach ($surface in $inventory.surfaces) {
    if (-not $surface.approval_required) {
        throw "approval_required must be true"
    }
    if ($surface.external_submission_allowed) {
        throw "external_submission_allowed must remain false"
    }
    if ($surface.blockers.Count -gt 0) {
        $blockedSurfaces += $surface.id
    }
    foreach ($source in $surface.requirements_sources) {
        if ($source.status -ne "searched") {
            $sourcesNeedingSearchOrRefresh += $surface.id
        }
    }
    Test-GateOrder -Surface $surface
}

Write-Host "repo_health_target=$($inventory.repo_health_target)"
Write-Host "blocked surfaces: $($blockedSurfaces -join ', ')"
Write-Host "sources needing search or refresh: $($sourcesNeedingSearchOrRefresh -join ', ')"
Write-Host "external_submission_allowed=false"
Write-Host "approval_required=true"