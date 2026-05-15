param(
    [string]$OutputDir = "C:\tmp\sourceright-ojs-lint\packages",
    [switch]$RequirePhp
)

$ErrorActionPreference = "Stop"

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
Push-Location $repoRoot
try {
    $results = [ordered]@{}

    New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null
    $packageJson = & powershell -NoProfile -ExecutionPolicy Bypass -File scripts/build-ojs-plugin-package.ps1 -OutputDir $OutputDir
    $results.package = $packageJson | ConvertFrom-Json

    $archiveEntries = (& tar -tzf $results.package.archive) -join "`n"
    foreach ($entry in @(
        "sourceright/index.php",
        "sourceright/SourcerightPlugin.php",
        "sourceright/classes/SourcerightCliRunner.php",
        "sourceright/version.xml",
        "sourceright/plugin.xml",
        "sourceright/locale/en_US/locale.po"
    )) {
        if (!$archiveEntries.Contains($entry)) {
            throw "OJS package archive is missing $entry"
        }
    }
    $results.archiveEntries = "ok"

    $xmlFiles = @(
        "plugins/ojs/sourceright/version.xml",
        "plugins/ojs/sourceright/plugin.xml"
    )
    $xmllint = Get-Command xmllint -ErrorAction SilentlyContinue
    if ($null -ne $xmllint) {
        foreach ($xml in $xmlFiles) {
            & xmllint --noout $xml
            if ($LASTEXITCODE -ne 0) {
                throw "xmllint failed for $xml"
            }
        }
        $results.xml = "xmllint"
    } else {
        foreach ($xml in $xmlFiles) {
            [xml](Get-Content -Raw -LiteralPath $xml) | Out-Null
        }
        $results.xml = "powershell-xml"
    }

    $php = Get-Command php -ErrorAction SilentlyContinue
    if ($null -ne $php) {
        foreach ($phpFile in @(
            "plugins/ojs/sourceright/index.php",
            "plugins/ojs/sourceright/SourcerightPlugin.php",
            "plugins/ojs/sourceright/classes/SourcerightCliRunner.php"
        )) {
            & php -l $phpFile
            if ($LASTEXITCODE -ne 0) {
                throw "php -l failed for $phpFile"
            }
        }
        $results.php = "linted"
    } elseif ($RequirePhp) {
        throw "PHP is required but was not found on PATH."
    } else {
        $results.php = "skipped: php not on PATH"
    }

    $env:CARGO_TARGET_DIR = "C:\tmp\sourceright-ojs-lint-target"
    & cargo +stable-x86_64-pc-windows-gnu test --test ojs_plugin_packaging_policy
    if ($LASTEXITCODE -ne 0) {
        throw "cargo test --test ojs_plugin_packaging_policy failed"
    }
    $results.rustPolicy = "passed"

    [pscustomobject]$results | ConvertTo-Json
} finally {
    Pop-Location
}
