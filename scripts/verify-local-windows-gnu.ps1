param(
    [string]$TargetDir = "C:\tmp\sourceright-target-local",
    [switch]$SkipBench,
    [switch]$SkipReportSmoke
)

$ErrorActionPreference = 'Stop'

$toolchain = 'stable-x86_64-pc-windows-gnu'

function Invoke-CheckedCommand {
    param(
        [Parameter(Mandatory = $true)]
        [scriptblock]$Command,
        [Parameter(Mandatory = $true)]
        [string]$Name
    )

    & $Command
    if ($LASTEXITCODE -ne 0) {
        throw "$Name failed with exit code $LASTEXITCODE"
    }
}

if (-not (Get-Command rustup -ErrorAction SilentlyContinue)) {
    throw 'rustup is required to select the Windows GNU Rust toolchain.'
}

$toolchains = (& rustup toolchain list) -join "`n"
if ($toolchains -notmatch [regex]::Escape($toolchain)) {
    throw "Missing Rust toolchain '$toolchain'. Install it with: rustup toolchain install $toolchain"
}

$targetParent = Split-Path -Parent $TargetDir
if ($targetParent -and -not (Test-Path -LiteralPath $targetParent)) {
    New-Item -ItemType Directory -Path $targetParent | Out-Null
}

Invoke-CheckedCommand -Name "cargo fmt" -Command {
    & cargo "+$toolchain" fmt --check
}
Invoke-CheckedCommand -Name "cargo clippy" -Command {
    & cargo "+$toolchain" clippy --locked --target-dir $TargetDir --all-targets -- -D warnings
}
Invoke-CheckedCommand -Name "cargo test" -Command {
    & cargo "+$toolchain" test --locked --target-dir $TargetDir
}
Invoke-CheckedCommand -Name "cargo check" -Command {
    & cargo "+$toolchain" check --locked --target-dir $TargetDir
}
Invoke-CheckedCommand -Name "plugins validate" -Command {
    & cargo "+$toolchain" run --locked --target-dir $TargetDir --bin sourceright -- plugins validate --json
}
Invoke-CheckedCommand -Name "submission readiness" -Command {
    & powershell -NoProfile -ExecutionPolicy Bypass -File scripts\verify-submission-readiness.ps1
}

if (-not $SkipBench) {
    Invoke-CheckedCommand -Name "bench smoke" -Command {
        & cargo "+$toolchain" run --locked --target-dir $TargetDir --bin sourceright -- bench
    }
}

if (-not $SkipReportSmoke) {
    Invoke-CheckedCommand -Name "report smoke" -Command {
        & cargo "+$toolchain" run --locked --target-dir $TargetDir --bin sourceright -- report --json examples/workspace
    }
}
