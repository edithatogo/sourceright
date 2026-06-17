param(
    [string]$EvidencePath = "conductor/submission-packets/live-evidence.json"
)

$ErrorActionPreference = "Stop"

Write-Host "sourceright.live_submission_evidence.v1"

if (-not (Test-Path $EvidencePath)) {
    Write-Host "REPLACE_WITH evidence file at $EvidencePath"
    return
}

$evidence = Get-Content $EvidencePath -Raw | ConvertFrom-Json
if ($evidence.status -eq "publicly_accepted") {
    Write-Host "publicly_accepted"
} else {
    Write-Host "REPLACE_WITH status not yet publicly_accepted"
}
