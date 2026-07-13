[CmdletBinding()]
param(
    [string]$TargetDirectory = (Join-Path ([System.IO.Path]::GetTempPath()) "sourceright-target")
)

$ErrorActionPreference = "Stop"
$env:CARGO_TARGET_DIR = $TargetDirectory
$isWindows = $env:OS -eq "Windows_NT"
if ($isWindows -and (Get-Command gcc -ErrorAction SilentlyContinue)) {
    $env:RUSTUP_TOOLCHAIN = "stable-x86_64-pc-windows-gnu"
    $env:CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER = "gcc"
} else {
    $env:RUSTUP_TOOLCHAIN = "stable"
    Remove-Item Env:CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER -ErrorAction SilentlyContinue
}

New-Item -ItemType Directory -Force $TargetDirectory | Out-Null

function Invoke-CheckedNativeCommand {
    param(
        [Parameter(Mandatory)]
        [scriptblock]$Command,
        [Parameter(Mandatory)]
        [string]$Description
    )

    & $Command
    if ($LASTEXITCODE -ne 0) {
        throw "$Description failed with exit code $LASTEXITCODE."
    }
}

Invoke-CheckedNativeCommand { cargo fmt --all --check } "cargo fmt --all --check"
Invoke-CheckedNativeCommand { cargo test --locked } "cargo test --locked"
Invoke-CheckedNativeCommand { cargo clippy --locked --all-targets -- -D warnings } "cargo clippy --locked --all-targets -- -D warnings"
Invoke-CheckedNativeCommand { cargo check --locked } "cargo check --locked"
Write-Host "Cross-platform Rust gates passed using $TargetDirectory."
