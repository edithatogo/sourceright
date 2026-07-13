[CmdletBinding()]
param(
    [string]$OutputDirectory = "conductor/.tmp/interoperability"
)

$ErrorActionPreference = "Stop"
$nodeRoot = "C:\Users\60217257\scoop\apps\nodejs\current"
$env:Path = "$nodeRoot;$nodeRoot\bin;$env:Path"
$runnerRoot = Join-Path $PSScriptRoot "..\interop-runners"
$fixtures = @(
    @{
        Name = "bibtex-basic"
        Input = "fixtures/interoperability/bibtex-basic.bib"
        Expected = "fixtures/interoperability/bibtex-basic.expected-csl.json"
    },
    @{
        Name = "bibtex-adversarial"
        Input = "fixtures/interoperability/bibtex-adversarial.bib"
        Expected = "fixtures/interoperability/bibtex-adversarial.expected-csl.json"
    },
    @{
        Name = "ris-basic"
        Input = "fixtures/interoperability/ris-basic.ris"
        Expected = "fixtures/interoperability/ris-basic.expected-csl.json"
    }
)

New-Item -ItemType Directory -Force $OutputDirectory | Out-Null
Push-Location $runnerRoot
try {
    npm ci --ignore-scripts
    New-Item -ItemType Directory -Force .tmp | Out-Null
    foreach ($fixture in $fixtures) {
        $oracle = ".tmp\$($fixture.Name)-citation-js.json"
        node citation-js-runner.mjs "..\$($fixture.Input)" $oracle
        if ($LASTEXITCODE -ne 0) {
            throw "Citation.js runner failed for $($fixture.Name)."
        }

        if ($fixture.Name -like "bibtex-*") {
            node bibtex-parser-runner.mjs "..\$($fixture.Input)" ".tmp\$($fixture.Name)-bibtex-parser.json"
            if ($LASTEXITCODE -ne 0) {
                throw "BibTeX parser runner failed for $($fixture.Name)."
            }
        }

        $report = Join-Path (Resolve-Path "..\$OutputDirectory") "$($fixture.Name).md"
        $env:CARGO_TARGET_DIR = "C:\tmp\sourceright-target"
        $env:RUSTUP_TOOLCHAIN = "stable-x86_64-pc-windows-gnu"
        $env:CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER = "gcc"
        cargo run --locked --bin interoperability-diff -- "..\$($fixture.Expected)" $oracle citation-js $report
        if ($LASTEXITCODE -ne 0) {
            throw "Interoperability comparison failed for $($fixture.Name)."
        }
    }
}
finally {
    Pop-Location
}

Write-Host "Interoperability matrix passed for $($fixtures.Count) fixtures."
