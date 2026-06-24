param(
    [string]$OutputDir = "",
    [string]$Python = ""
)

$ErrorActionPreference = "Stop"

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")
$probePy = Join-Path $repoRoot "scripts\colab\directory-probes.py"
if (-not (Test-Path -LiteralPath $probePy)) {
    throw "Missing probe script: $probePy"
}

if ([string]::IsNullOrWhiteSpace($OutputDir)) {
    $OutputDir = Join-Path $repoRoot "conductor\tracks\73-mcp-directory-submission-hardening"
}
New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null

if ([string]::IsNullOrWhiteSpace($Python)) {
    $Python = "C:\Users\60217257\AppData\Local\miniconda3\python.exe"
    if (-not (Test-Path -LiteralPath $Python)) {
        $Python = "python"
    }
}

$account = az account show --query "{name:name, id:id}" --output json 2>$null
if ($LASTEXITCODE -ne 0) {
    Write-Warning "Azure CLI is not logged in; probes still run locally."
    $account = $null
}

$stamp = Get-Date -Format "yyyy-MM-dd"
$jsonPath = Join-Path $OutputDir "directory-probes-$stamp.json"
$mdPath = Join-Path $OutputDir "azure-directory-probes-$stamp.md"

& $Python $probePy | Set-Content -LiteralPath $jsonPath -Encoding utf8
$probeJson = Get-Content -Raw -LiteralPath $jsonPath | ConvertFrom-Json

$lines = New-Object System.Collections.Generic.List[string]
$lines.Add("# Directory listing probes ($stamp)")
$lines.Add("")
$lines.Add("Runner: local Python (`scripts/colab/directory-probes.py`) on Windows.")
if ($account) {
    $lines.Add("Azure context: $($account.name) ($($account.id))")
} else {
    $lines.Add("Azure context: not verified (run `az login` for subscription metadata).")
}
$lines.Add("")
$lines.Add("| Label | HTTP | URL |")
$lines.Add("| --- | --- | --- |")
foreach ($row in $probeJson.probes) {
    $lines.Add("| $($row.label) | $($row.http_code) | $($row.url) |")
}
$lines.Add("")
$lines.Add("Raw JSON: ``directory-probes-$stamp.json``")
$lines.Add("")
$lines.Add("## Next steps")
$lines.Add("")
$lines.Add("- **Glama** listing/API 404 means Add Server is still required at https://glama.ai/mcp/servers (signed-in browser).")
$lines.Add("- **Colab CLI** (Linux/WSL): ``colab run scripts/colab/directory-probes.py``; see ``scripts/colab/README.md``.")
$lines.Add("- **Azure ACI** from Windows `az` has fragile `--command-line` quoting; prefer this script, Azure Cloud Shell, or WSL.")

$lines -join "`n" | Set-Content -LiteralPath $mdPath -Encoding utf8

Write-Host "Wrote $jsonPath"
Write-Host "Wrote $mdPath"

@{
    json_path = $jsonPath
    markdown_path = $mdPath
    probes = $probeJson.probes
} | ConvertTo-Json -Depth 5
