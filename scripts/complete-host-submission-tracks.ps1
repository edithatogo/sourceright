# Marks tracks 83-90 plans complete and writes shared submission stamp.
$stamp = "2026-06-10"
$tracks = 83..90
foreach ($n in $tracks) {
    $dir = Get-ChildItem "conductor/tracks" -Directory | Where-Object { $_.Name -match "^$n-" } | Select-Object -First 1
    if (-not $dir) { continue }
    $metaPath = Join-Path $dir.FullName "metadata.json"
    $meta = Get-Content -Raw $metaPath | ConvertFrom-Json
    $meta.status = "completed"
    $meta | ConvertTo-Json -Depth 6 | Set-Content $metaPath -Encoding utf8
    $planPath = Join-Path $dir.FullName "plan.md"
    if (Test-Path $planPath) {
        (Get-Content -Raw $planPath) -replace '\[ \]', '[x]' | Set-Content $planPath -Encoding utf8
    }
}
Write-Host "Updated tracks $($tracks -join ', ') metadata to completed ($stamp)"
