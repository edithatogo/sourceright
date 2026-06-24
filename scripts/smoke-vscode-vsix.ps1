param(
    [string]$VsixPath = "dist/edithatogo.sourceright-0.1.20.vsix",
    [string]$CodeCommand = "code.cmd",
    [string]$SmokeRoot = "C:\tmp\sourceright-vscode-vsix-smoke"
)

$ErrorActionPreference = "Stop"

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..")
$vsix = [System.IO.Path]::GetFullPath((Join-Path $repoRoot $VsixPath))
if (-not (Test-Path -LiteralPath $vsix)) {
    throw "Missing VSIX package: $vsix"
}

$smokeRootFull = [System.IO.Path]::GetFullPath($SmokeRoot)
if (-not $smokeRootFull.StartsWith("C:\tmp\", [System.StringComparison]::OrdinalIgnoreCase)) {
    throw "SmokeRoot must stay under C:\tmp for isolated extension-host smoke: $smokeRootFull"
}

if (Test-Path -LiteralPath $smokeRootFull) {
    Remove-Item -LiteralPath $smokeRootFull -Recurse -Force
}

$extensionsDir = Join-Path $smokeRootFull "extensions"
$userDataDir = Join-Path $smokeRootFull "user-data"
New-Item -ItemType Directory -Force -Path $extensionsDir, $userDataDir | Out-Null

function Invoke-Code {
    param([string[]]$Arguments)
    $output = & $CodeCommand @Arguments 2>&1
    $exit = $LASTEXITCODE
    if ($exit -ne 0) {
        throw "VS Code command failed ($exit): $CodeCommand $($Arguments -join ' ')`n$output"
    }
    $output
}

Invoke-Code @(
    "--user-data-dir", $userDataDir,
    "--extensions-dir", $extensionsDir,
    "--install-extension", $vsix,
    "--force"
) | Out-Null

$installed = Invoke-Code @(
    "--user-data-dir", $userDataDir,
    "--extensions-dir", $extensionsDir,
    "--list-extensions"
)
if (-not ($installed -contains "edithatogo.sourceright")) {
    throw "VSIX install smoke did not list edithatogo.sourceright"
}

Invoke-Code @(
    "--user-data-dir", $userDataDir,
    "--extensions-dir", $extensionsDir,
    "--uninstall-extension", "edithatogo.sourceright",
    "--force"
) | Out-Null

$remaining = Invoke-Code @(
    "--user-data-dir", $userDataDir,
    "--extensions-dir", $extensionsDir,
    "--list-extensions"
)
if ($remaining -contains "edithatogo.sourceright") {
    throw "VSIX uninstall smoke still lists edithatogo.sourceright"
}

[pscustomobject]@{
    schema_version = "sourceright.vscode_vsix_smoke.v1"
    vsix = $vsix
    code_command = $CodeCommand
    extensions_dir = $extensionsDir
    user_data_dir = $userDataDir
    installed_extension = "edithatogo.sourceright"
    install_smoke = "passed"
    uninstall_smoke = "passed"
} | ConvertTo-Json -Depth 5
