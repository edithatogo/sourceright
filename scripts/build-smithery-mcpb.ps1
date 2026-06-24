param(
    [Parameter(Mandatory = $true)]
    [string]$BinaryPath,

    [string]$OutputPath = "",

    [ValidateSet("linux", "win32", "darwin")]
    [string]$Platform = "linux"
)

$ErrorActionPreference = "Stop"

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
$binary = Resolve-Path -LiteralPath $BinaryPath
$templatePath = Join-Path $repoRoot "smithery/mcpb/manifest.template.json"
$cargoTomlPath = Join-Path $repoRoot "Cargo.toml"

if (-not (Test-Path -LiteralPath $templatePath)) {
    throw "Missing Smithery MCPB manifest template: $templatePath"
}

$cargoToml = Get-Content -Raw -LiteralPath $cargoTomlPath
if ($cargoToml -notmatch '(?m)^version\s*=\s*"([^"]+)"') {
    throw "Could not parse package version from Cargo.toml"
}
$version = $Matches[1]

if ([string]::IsNullOrWhiteSpace($OutputPath)) {
    $OutputPath = Join-Path $repoRoot "dist/sourceright-smithery-$version-$Platform.mcpb"
}

if ([System.IO.Path]::IsPathRooted($OutputPath)) {
    $output = [System.IO.Path]::GetFullPath($OutputPath)
} else {
    $output = [System.IO.Path]::GetFullPath((Join-Path (Get-Location) $OutputPath))
}
$distDir = Split-Path -Parent $output
New-Item -ItemType Directory -Force -Path $distDir | Out-Null

$stagingRoot = Join-Path $repoRoot (".tmp/smithery-mcpb-$Platform-" + [guid]::NewGuid().ToString("N"))
New-Item -ItemType Directory -Force -Path (Join-Path $stagingRoot "bin") | Out-Null

$binaryName = if ($Platform -eq "win32") { "sourceright.exe" } else { "sourceright" }
Copy-Item -LiteralPath $binary -Destination (Join-Path $stagingRoot "bin/$binaryName") -Force

$serverCardPath = Join-Path $repoRoot "mcp/server-card.json"
if (-not (Test-Path -LiteralPath $serverCardPath)) {
    throw "Missing MCP server card for Smithery metadata: $serverCardPath"
}

$serverCard = Get-Content -Raw -LiteralPath $serverCardPath | ConvertFrom-Json
$manifest = Get-Content -Raw -LiteralPath $templatePath | ConvertFrom-Json
$manifest.version = $version
$manifest.compatibility.platforms = @($Platform)
$manifest.server.entry_point = "bin/$binaryName"
$manifest.server.mcp_config.command = '${__dirname}' + "/bin/$binaryName"
$manifest | Add-Member -NotePropertyName tools -NotePropertyValue $serverCard.tools -Force
$manifest | Add-Member -NotePropertyName resources -NotePropertyValue $serverCard.resources -Force
$manifest | Add-Member -NotePropertyName prompts -NotePropertyValue $serverCard.prompts -Force
$manifest | ConvertTo-Json -Depth 30 | Set-Content -LiteralPath (Join-Path $stagingRoot "manifest.json") -Encoding UTF8

Copy-Item -LiteralPath (Join-Path $repoRoot "README.md") -Destination (Join-Path $stagingRoot "README.md") -Force
Copy-Item -LiteralPath (Join-Path $repoRoot "LICENSE-MIT") -Destination (Join-Path $stagingRoot "LICENSE-MIT") -Force
Copy-Item -LiteralPath (Join-Path $repoRoot "LICENSE-APACHE") -Destination (Join-Path $stagingRoot "LICENSE-APACHE") -Force

if (Test-Path -LiteralPath $output) {
    Remove-Item -LiteralPath $output -Force
}

$zipPath = if ([System.IO.Path]::GetExtension($output) -eq ".zip") {
    $output
} else {
    Join-Path $distDir ("smithery-" + [guid]::NewGuid().ToString("N") + ".zip")
}
Compress-Archive -Path (Join-Path $stagingRoot "*") -DestinationPath $zipPath -Force
if (-not (Test-Path -LiteralPath $zipPath)) {
    throw "Compress-Archive did not create expected archive: $zipPath"
}
if ($zipPath -ne $output) {
    Copy-Item -LiteralPath $zipPath -Destination $output -Force
    try {
        Remove-Item -LiteralPath $zipPath -Force
    } catch {
        Write-Warning "Temporary zip could not be removed: $zipPath"
    }
}

$manifestPath = Join-Path $stagingRoot "manifest.json"
$generated = Get-Content -Raw -LiteralPath $manifestPath | ConvertFrom-Json
if ($generated.manifest_version -ne "0.3") {
    throw "Generated MCPB manifest has unexpected manifest_version"
}
if ($generated.server.mcp_config.args[0] -ne "mcp") {
    throw "Generated MCPB manifest does not start Sourceright in MCP mode"
}
if (-not $generated.tools -or @($generated.tools).Count -lt 1) {
    throw "Generated MCPB manifest is missing embedded Smithery tool metadata"
}

[pscustomobject]@{
    schema_version = "sourceright.smithery_mcpb_build.v1"
    output = $output
    platform = $Platform
    version = $version
    manifest = $manifestPath
} | ConvertTo-Json -Depth 5
