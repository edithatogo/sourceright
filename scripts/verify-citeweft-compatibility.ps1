[CmdletBinding()]
param(
    [string]$CiteWeftRoot = "C:\tmp\citeweft"
)

$ErrorActionPreference = "Stop"
$pairs = @(
    @{ Source = "src/citeweft.rs"; Destination = "src/citeweft.rs" },
    @{ Source = "src/entity_model.rs"; Destination = "src/entity_model.rs" },
    @{ Source = "src/layout.rs"; Destination = "src/layout.rs" },
    @{ Source = "src/reference_model.rs"; Destination = "src/reference_model.rs" },
    @{ Source = "src/routing.rs"; Destination = "src/routing.rs" }
)

foreach ($pair in $pairs) {
    $source = Join-Path (Get-Location) $pair.Source
    $destination = Join-Path $CiteWeftRoot $pair.Destination
    if (-not (Test-Path -LiteralPath $source -PathType Leaf)) { throw "Missing Sourceright file: $($pair.Source)" }
    if (-not (Test-Path -LiteralPath $destination -PathType Leaf)) { throw "Missing CiteWeft file: $($pair.Destination)" }
    $sourceHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $source).Hash
    $destinationHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $destination).Hash
    if ($sourceHash -ne $destinationHash) {
        throw "Neutral module mismatch: $($pair.Source) ($sourceHash != $destinationHash)"
    }
}

Write-Output "CiteWeft neutral module compatibility hashes passed for $($pairs.Count) modules."
