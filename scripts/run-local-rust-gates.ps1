[CmdletBinding()]
param(
    [string]$TargetDirectory = "C:\tmp\sourceright-target"
)

$ErrorActionPreference = "Stop"
$env:CARGO_TARGET_DIR = $TargetDirectory
$env:RUSTUP_TOOLCHAIN = "stable-x86_64-pc-windows-gnu"
$env:CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER = "gcc"

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
Write-Host "Local GNU Rust gates passed using $TargetDirectory."
