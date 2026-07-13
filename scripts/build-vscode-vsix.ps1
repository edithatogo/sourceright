param(
    [string]$ExtensionDir = "extensions/vscode-sourceright",
    [string]$OutputDir = "dist"
)

$ErrorActionPreference = "Stop"

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
$extensionPath = [System.IO.Path]::GetFullPath((Join-Path $repoRoot $ExtensionDir))
$outputPath = [System.IO.Path]::GetFullPath((Join-Path $repoRoot $OutputDir))

if (-not (Test-Path -LiteralPath (Join-Path $extensionPath "package.json"))) {
    throw "Missing VS Code extension package.json in $extensionPath"
}

$package = Get-Content -Raw -LiteralPath (Join-Path $extensionPath "package.json") | ConvertFrom-Json
foreach ($field in @("name", "displayName", "version", "publisher", "main", "engines")) {
    if ($null -eq $package.$field -or [string]::IsNullOrWhiteSpace([string]$package.$field)) {
        throw "VS Code extension package.json is missing $field"
    }
}
if ($package.capabilities.untrustedWorkspaces.supported -ne $true) {
    throw "VS Code extension must declare Workspace Trust behavior"
}

New-Item -ItemType Directory -Force -Path $outputPath | Out-Null
$vsix = Join-Path $outputPath "$($package.publisher).$($package.name)-$($package.version).vsix"
if (Test-Path -LiteralPath $vsix) {
    Remove-Item -LiteralPath $vsix -Force
}

$stagingDir = Join-Path $repoRoot (".tmp/vscode-vsix-" + [guid]::NewGuid().ToString("N"))
New-Item -ItemType Directory -Force -Path $stagingDir | Out-Null
$extensionStagingDir = Join-Path $stagingDir "extension"
New-Item -ItemType Directory -Force -Path $extensionStagingDir | Out-Null
Copy-Item -Path (Join-Path $extensionPath "*") -Destination $extensionStagingDir -Recurse -Force
$zip = Join-Path $stagingDir "$($package.publisher).$($package.name)-$($package.version).zip"

Compress-Archive -Path (Join-Path $stagingDir "extension") -DestinationPath $zip -Force
Copy-Item -LiteralPath $zip -Destination $vsix -Force
try {
    Remove-Item -LiteralPath $stagingDir -Recurse -Force
} catch {
    Write-Warning "Temporary VSIX staging directory could not be removed: $stagingDir"
}

[pscustomobject]@{
    schema_version = "sourceright.vscode_vsix_build.v1"
    output = $vsix
    name = $package.name
    publisher = $package.publisher
    version = $package.version
    workspace_trust_supported = $package.capabilities.untrustedWorkspaces.supported
} | ConvertTo-Json -Depth 5
