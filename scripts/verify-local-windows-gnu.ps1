function Invoke-CheckedCommand {
    param(
        [scriptblock]$Command
    )

    & $Command
    if ($LASTEXITCODE -ne 0) {
        throw "command failed with exit code $LASTEXITCODE"
    }
}

Invoke-CheckedCommand {
    powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify-submission-readiness.ps1
}
