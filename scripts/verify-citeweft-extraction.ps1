param(
    [string]$ManifestPath = "conductor/citeweft-extraction-manifest.json"
)

$ErrorActionPreference = "Stop"
$manifest = Get-Content -Raw $ManifestPath | ConvertFrom-Json
$root = (Get-Location).Path
$core = @($manifest.standalone_core)
$adapters = @($manifest.sourceright_adapters)

$overlap = @($core | Where-Object { $adapters -contains $_ })
if ($overlap.Count -gt 0) {
    throw "core and adapter inventories overlap: $($overlap -join ', ')"
}
if (@($manifest.schema_contracts).Count -eq 0) {
    throw "standalone schema contract inventory is empty"
}
foreach ($gate in @('destination_approval', 'history_preservation', 'independent_ci_security', 'package_release', 'downstream_compatibility', 'rollback_issue_migration')) {
    if ($manifest.external_gates.$gate -ne 'open') {
        throw "external gate must remain explicitly open in local inventory: $gate"
    }
}

foreach ($path in $manifest.standalone_core) {
    $resolved = Join-Path $root $path
    if (-not (Test-Path -LiteralPath $resolved -PathType Leaf)) {
        throw "standalone core path is missing: $path"
    }
    $content = Get-Content -Raw $resolved
    foreach ($forbidden in $manifest.forbidden_core_imports) {
        if ($content.Contains($forbidden)) {
            throw "forbidden standalone-core import `$forbidden` in $path"
        }
    }
}

foreach ($path in $manifest.sourceright_adapters) {
    if (-not (Test-Path -LiteralPath (Join-Path $root $path) -PathType Leaf)) {
        throw "adapter path is missing: $path"
    }
}

if ($manifest.publication_claim -ne "not_permitted_from_local_inventory") {
    throw "local extraction inventory must not make a publication claim"
}

Write-Output "CiteWeft extraction independence checks passed for $($manifest.standalone_core.Count) core modules."
