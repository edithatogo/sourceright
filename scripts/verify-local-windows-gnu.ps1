$ErrorActionPreference = "Stop"

$Toolchain = "stable-x86_64-pc-windows-gnu"
$TargetDir = "C:\tmp\sourceright-target-local"

function Invoke-CheckedCommand {
    param(
        [scriptblock]$Command
    )

    & $Command
    if ($LASTEXITCODE -ne 0) {
        throw "command failed with exit code $LASTEXITCODE"
    }
}

Invoke-CheckedCommand { cargo fmt --all --check }
Invoke-CheckedCommand {
    & (Join-Path $PSScriptRoot 'verify-submission-readiness.ps1')
}
Invoke-CheckedCommand { cargo +$Toolchain clippy --locked --target-dir $TargetDir --all-targets -- -D warnings }
Invoke-CheckedCommand { cargo +$Toolchain test --locked --target-dir $TargetDir }
Invoke-CheckedCommand { cargo +$Toolchain check --locked --target-dir $TargetDir }
Invoke-CheckedCommand {
    cargo +$Toolchain run --locked --target-dir $TargetDir --bin sourceright -- plugins validate --json
}
Invoke-CheckedCommand {
    cargo +$Toolchain run --locked --target-dir $TargetDir --bin sourceright -- bench
}
Invoke-CheckedCommand {
    cargo +$Toolchain run --locked --target-dir $TargetDir --bin sourceright -- report --json examples/workspace
}

Write-Host "Windows GNU validation passed."
