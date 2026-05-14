param(
    [string]$Version = "0.1.0",
    [string]$OutputDir = "dist/ojs"
)

$ErrorActionPreference = "Stop"

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$sourceDir = Join-Path $repoRoot "plugins/ojs/sourceright"
if ([System.IO.Path]::IsPathRooted($OutputDir)) {
    $outDir = $OutputDir
} else {
    $outDir = Join-Path $repoRoot $OutputDir
}
$stageRoot = Join-Path ([System.IO.Path]::GetTempPath()) ("sourceright-ojs-stage-" + [guid]::NewGuid().ToString("N"))
$stagePlugin = Join-Path $stageRoot "sourceright"
$archiveName = "sourceright-ojs-generic-plugin-$Version.tar.gz"
$archivePath = Join-Path $outDir $archiveName
$checksumPath = "$archivePath.sha256"

if (!(Test-Path -LiteralPath $sourceDir)) {
    throw "Missing OJS plugin source directory: $sourceDir"
}

$tar = Get-Command tar -ErrorAction SilentlyContinue
if ($null -eq $tar) {
    throw "The 'tar' command is required to build the OJS plugin archive."
}

New-Item -ItemType Directory -Force -Path $outDir | Out-Null
New-Item -ItemType Directory -Force -Path $stagePlugin | Out-Null

Copy-Item -Path (Join-Path $sourceDir "*") -Destination $stagePlugin -Recurse -Force

if (Test-Path -LiteralPath $archivePath) {
    Remove-Item -LiteralPath $archivePath -Force
}

Push-Location $stageRoot
try {
    & tar -czf $archivePath "sourceright"
} finally {
    Pop-Location
}

$hash = (Get-FileHash -Algorithm SHA256 -LiteralPath $archivePath).Hash.ToLowerInvariant()
"$hash  $archiveName" | Set-Content -LiteralPath $checksumPath -Encoding UTF8

[pscustomobject]@{
    archive = $archivePath
    sha256 = $checksumPath
    pluginInstallPath = "plugins/generic/sourceright"
} | ConvertTo-Json

try {
    Remove-Item -LiteralPath $stageRoot -Recurse -Force
} catch {
    Write-Warning "Built archive, but could not remove staging directory: $stageRoot"
}
