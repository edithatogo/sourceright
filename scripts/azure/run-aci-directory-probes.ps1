param(
    [string]$ResourceGroup = "rg-kairos-batch-canary-20260520",
    [string]$Location = "australiaeast",
    [string]$OutputDir = "",
    [string]$ContainerName = ""
)

$ErrorActionPreference = "Stop"

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")
if ([string]::IsNullOrWhiteSpace($OutputDir)) {
    $OutputDir = Join-Path $repoRoot "conductor\tracks\73-mcp-directory-submission-hardening"
}
New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null

if ([string]::IsNullOrWhiteSpace($ContainerName)) {
    $ContainerName = "sr-dir-probe-" + (Get-Date -Format "yyyyMMddHHmmss")
}

# Single-line shell avoids cmd.exe multiline/brace parsing when az forwards args on Windows.
$probeOneLiner = @(
    "echo directory_probe_run"
    "echo runner:azure-aci"
    "curl -sS -L --max-time 30 -w 'smithery_listing %{http_code}\n' https://smithery.ai/servers/edithatogo/sourceright >/tmp/body"
    "curl -sS -L --max-time 30 -w 'smithery_search %{http_code}\n' 'https://smithery.ai/search?q=sourceright' >/tmp/body"
    "curl -sS -L --max-time 30 -w 'glama_listing %{http_code}\n' https://glama.ai/mcp/servers/edithatogo/sourceright >/tmp/body"
    "curl -sS -L --max-time 30 -w 'glama_api %{http_code}\n' https://glama.ai/api/mcp/v1/servers/edithatogo/sourceright >/tmp/body"
    "curl -sS -L --max-time 30 -w 'glama_search %{http_code}\n' 'https://glama.ai/mcp/servers?q=sourceright' >/tmp/body"
    "echo done"
) -join "; "

$probeScript = ($probeOneLiner -split '; ' ) -join "`n"
$probeB64 = [Convert]::ToBase64String([Text.Encoding]::UTF8.GetBytes($probeScript))
$commandLine = "sh -c `"echo $probeB64 | base64 -d | sh`""

Write-Host "Creating ACI job $ContainerName in $ResourceGroup ($Location)..."

$azArgs = @(
    "container", "create",
    "--resource-group", $ResourceGroup,
    "--name", $ContainerName,
    "--image", "curlimages/curl:8.12.1",
    "--os-type", "Linux",
    "--location", $Location,
    "--restart-policy", "Never",
    "--cpu", "1",
    "--memory", "1",
    "--command-line", $commandLine,
    "--only-show-errors",
    "--output", "none"
)
& az @azArgs
if ($LASTEXITCODE -ne 0) {
    throw "az container create failed with exit code $LASTEXITCODE"
}

$state = "Unknown"
$deadline = (Get-Date).AddMinutes(6)
while ((Get-Date) -lt $deadline) {
    $state = az container show `
        --resource-group $ResourceGroup `
        --name $ContainerName `
        --query "instanceView.state" `
        --output tsv
    if ($state -in @("Succeeded", "Failed", "Terminated")) {
        break
    }
    Start-Sleep -Seconds 8
}

$logs = (az container logs `
    --resource-group $ResourceGroup `
    --name $ContainerName 2>&1 | Out-String).Trim()

$stamp = Get-Date -Format "yyyy-MM-dd"
$outPath = Join-Path $OutputDir "azure-directory-probes-$stamp.md"

@"
# Azure ACI Directory Probes

Date: $stamp  
Runner: Azure Container Instances (`curlimages/curl:8.12.1`)  
Resource group: $ResourceGroup  
Container: $ContainerName  
Final state: $state

## Log

``````text
$logs
``````

## Notes

- Glama **Add Server** still requires a signed-in browser session; this job only
  re-probes public listing/API URLs from Linux egress.
- Smithery install smoke and Glama submission are not automated here.
"@ | Set-Content -LiteralPath $outPath -Encoding utf8

Write-Host "Wrote $outPath"
Write-Host "Container state: $state"

az container delete `
    --resource-group $ResourceGroup `
    --name $ContainerName `
    --yes `
    --output none | Out-Null

@{
    output_path = $outPath
    container_state = $state
    log = $logs
} | ConvertTo-Json -Depth 3
