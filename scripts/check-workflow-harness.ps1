[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"
$failures = [System.Collections.Generic.List[string]]::new()
$workflows = Get-ChildItem -LiteralPath ".github/workflows" -File |
    Where-Object { $_.Extension -in ".yml", ".yaml" }

foreach ($workflow in $workflows) {
    $lines = Get-Content -LiteralPath $workflow.FullName
    $text = $lines -join "`n"

    if ($text -notmatch "(?m)^permissions:\s*$") {
        $failures.Add("$($workflow.Name): missing top-level permissions declaration")
    }
    if ($text -notmatch "(?m)^concurrency:\s*$") {
        $failures.Add("$($workflow.Name): missing top-level concurrency declaration")
    }

    $inJobs = $false
    $jobName = $null
    $jobHasTimeout = $false
    for ($index = 0; $index -lt $lines.Count; $index++) {
        $line = $lines[$index]
        if ($line -eq "jobs:") {
            $inJobs = $true
            continue
        }
        if ($inJobs -and $line -match "^  ([A-Za-z0-9_-]+):$") {
            if ($jobName -and -not $jobHasTimeout) {
                $failures.Add("$($workflow.Name): job '$jobName' is missing timeout-minutes")
            }
            $jobName = $Matches[1]
            $jobHasTimeout = $false
            continue
        }
        if ($inJobs -and $jobName -and $line -match "^    timeout-minutes:\s*(\d+)\s*$") {
            $minutes = [int]$Matches[1]
            if ($minutes -lt 1 -or $minutes -gt 360) {
                $failures.Add("$($workflow.Name): job '$jobName' timeout-minutes must be between 1 and 360")
            }
            $jobHasTimeout = $true
        }
    }
    if ($jobName -and -not $jobHasTimeout) {
        $failures.Add("$($workflow.Name): job '$jobName' is missing timeout-minutes")
    }

    for ($index = 0; $index -lt $lines.Count; $index++) {
        $line = $lines[$index]
        if ($line -match "uses:\s+([^\s#]+)@([^\s#]+)") {
            $reference = $Matches[2]
            if ($reference -notmatch "^[0-9a-f]{40}$") {
                $failures.Add("$($workflow.Name):$($index + 1): action is not pinned to a full commit SHA")
            }
        }

        if ($line -match "uses:\s+actions/checkout@") {
            $windowEnd = [Math]::Min($index + 6, $lines.Count - 1)
            $window = $lines[$index..$windowEnd] -join "`n"
            if ($window -notmatch "persist-credentials:\s*false") {
                $failures.Add("$($workflow.Name):$($index + 1): checkout must disable persisted credentials")
            }
        }
    }
}

if ($failures.Count -gt 0) {
    $failures | ForEach-Object { Write-Host $_ -ForegroundColor Red }
    exit 1
}

Write-Host "Workflow harness passed for $($workflows.Count) workflows."
exit 0
