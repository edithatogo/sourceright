param(
    [string]$WorkDir = "C:\tmp\sourceright-ojs-smoke",
    [string]$PluginVersion = "0.1.0",
    [switch]$FetchPkpContainers
)

$ErrorActionPreference = "Stop"

$dockerVersion = (& docker --version) -join "`n"
$composeVersion = (& docker compose version) -join "`n"

New-Item -ItemType Directory -Force -Path $WorkDir | Out-Null
$resolvedWorkDir = (Resolve-Path -LiteralPath $WorkDir).Path
$containersDir = Join-Path $resolvedWorkDir "containers"
$packageOutputDir = Join-Path $resolvedWorkDir "packages"

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$packageScript = Join-Path $repoRoot "scripts/build-ojs-plugin-package.ps1"
$packageResultJson = & powershell -NoProfile -ExecutionPolicy Bypass -File $packageScript -Version $PluginVersion -OutputDir $packageOutputDir
$packageResult = $packageResultJson | ConvertFrom-Json

if ($FetchPkpContainers) {
    if (!(Test-Path -LiteralPath $containersDir)) {
        & git clone https://github.com/pkp/containers.git $containersDir
    }
}

$planPath = Join-Path $resolvedWorkDir "sourceright-ojs-docker-smoke-plan.md"
$archivePath = $packageResult.archive
$checksumPath = $packageResult.sha256

$plan = @"
# Sourceright OJS Docker Smoke Plan

Generated from: $repoRoot

## Local Preflight

- Docker: $dockerVersion
- Docker Compose: $composeVersion
- Plugin archive: $archivePath
- SHA-256 sidecar: $checksumPath
- Expected OJS plugin path: plugins/generic/sourceright

## PKP Container Setup

Use PKP's current container repository:

    git clone https://github.com/pkp/containers.git "$containersDir"
    Set-Location "$containersDir"

Edit .env for a disposable OJS run. Prefer PKP_TOOL=ojs,
IMAGE_SOURCE=docker-io, and a pinned OJS/PHP image tag rather than latest.

Then start OJS:

    docker compose up -d

Finish the browser-based OJS installer at the host/port defined by `.env`.

## Install Sourceright Plugin Into The Container

Copy or mount the unpacked archive so this directory exists inside the OJS
container:

/var/www/html/plugins/generic/sourceright

Inside the OJS container, register the plugin metadata:

    php lib/pkp/tools/installPluginVersion.php plugins/generic/sourceright/version.xml

Then enable the plugin in OJS as an administrator under Website/Plugins.

## Sourceright CLI

The OJS web-server user must be able to execute sourceright.

For a container smoke, either:

- mount a release binary into the OJS container and configure
  sourcerightCliPath; or
- install from crates.io inside a derived test image.

Verify inside the container:

    sourceright --version
    sourceright journal-screen --help

## Evidence To Capture

- OJS version, PHP version, and PKP container tag.
- Plugin archive SHA-256.
- Output of installPluginVersion.php.
- Administrator enablement note or screenshot.
- OJS error log excerpt showing no Sourceright plugin fatal errors.
- One sourceright journal-screen --platform ojs transcript against a
  disposable Sourceright workspace.

## Boundary

This plan is an opt-in smoke harness. It does not prove PKP Plugin Gallery
acceptance, and it does not make OJS compatibility claims until a completed
transcript is committed as evidence.
"@

$plan | Set-Content -LiteralPath $planPath -Encoding UTF8

[pscustomobject]@{
    docker = $dockerVersion
    compose = $composeVersion
    pluginArchive = $archivePath
    sha256 = $checksumPath
    workDir = $resolvedWorkDir
    plan = $planPath
    pkpContainers = if (Test-Path -LiteralPath $containersDir) { $containersDir } else { $null }
} | ConvertTo-Json
