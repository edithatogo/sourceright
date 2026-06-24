param(
    [string]$ResourceGroup = "rg-kairos-batch-canary-20260520",
    [string]$Location = "australiaeast",
    [string]$OutputDir = "",
    [string]$ContainerName = ""
)

$ErrorActionPreference = "Stop"

$repoRoot = Resolve-Path (Join-Path $PSScriptRoot "..\..")
if ([string]::IsNullOrWhiteSpace($OutputDir)) {
    $OutputDir = Join-Path $repoRoot "conductor\tracks\75-journal-platform-publication-hardening"
}
New-Item -ItemType Directory -Force -Path $OutputDir | Out-Null

if ([string]::IsNullOrWhiteSpace($ContainerName)) {
    $ContainerName = "sr-ojs-smoke-" + (Get-Date -Format "yyyyMMddHHmmss")
}

# Linux runner equivalent to `colab run scripts/colab/ojs-docker-smoke.py` when Colab CLI
# cannot run on native Windows (fcntl). Compact inline runner avoids Windows cmd length limits.
$smokeScript = @'
set -eu
export RUNNER=azure-aci-linux
apk add --no-cache git python3 curl tar gzip >/dev/null
git clone --depth 1 --branch main https://github.com/edithatogo/sourceright.git /tmp/sr
mkdir -p /tmp/sr/bin /tmp/sr/packages
curl -fsSL -o /tmp/sr/bin/sourceright https://github.com/edithatogo/sourceright/releases/download/v0.1.20/sourceright-linux-x86_64
chmod +x /tmp/sr/bin/sourceright
SR=/tmp/sr/bin/sourceright
tar -czf /tmp/sr/packages/sourceright-ojs-generic-plugin-0.1.0.tar.gz -C /tmp/sr/plugins/ojs sourceright
python3 <<'PY'
import json, subprocess, pathlib
r = pathlib.Path("/tmp/sr")
f = json.loads((r / "fixtures/journal/ojs-submission.json").read_text())
w = r / "screen"
w.mkdir()
subprocess.check_call(["/tmp/sr/bin/sourceright", "init"], cwd=w)
s = f["submission"]
ws = w / ".sourceright"
(ws / "references.csl.json").write_text(json.dumps(f["csl_references"]))
sc = dict(f["verification_sidecar"])
sc["schema_version"] = "sourceright.verification.v1"
sc.pop("schema", None)
(ws / "references.verification.json").write_text(json.dumps(sc))
p = subprocess.run(
    ["/tmp/sr/bin/sourceright", "journal-screen", "--platform", s["platform"],
     "--submission-id", s["submission_id"], "--manuscript", s["manuscript_label"], ".sourceright"],
    cwd=w, capture_output=True, text=True,
)
print(p.stdout)
print("exit", p.returncode)
assert p.returncode == 0 and "sourceright.journal_screening.v1" in p.stdout
PY
echo ojs_colab_smoke_done
'@

$smokeB64 = [Convert]::ToBase64String([Text.Encoding]::UTF8.GetBytes($smokeScript))
$commandLine = "sh -c `"echo $smokeB64 | base64 -d | sh`""

Write-Host "Creating ACI OJS smoke job $ContainerName in $ResourceGroup ($Location)..."

$azArgs = @(
    "container", "create",
    "--resource-group", $ResourceGroup,
    "--name", $ContainerName,
    "--image", "python:3.12-alpine",
    "--os-type", "Linux",
    "--location", $Location,
    "--restart-policy", "Never",
    "--cpu", "1",
    "--memory", "1.5",
    "--command-line", $commandLine,
    "--only-show-errors",
    "--output", "none"
)
& az @azArgs
if ($LASTEXITCODE -ne 0) {
    throw "az container create failed with exit code $LASTEXITCODE"
}

$state = "Unknown"
$deadline = (Get-Date).AddMinutes(10)
while ((Get-Date) -lt $deadline) {
    $state = az container show `
        --resource-group $ResourceGroup `
        --name $ContainerName `
        --query "instanceView.state" `
        --output tsv
    if ($state -in @("Succeeded", "Failed", "Terminated")) {
        break
    }
    Start-Sleep -Seconds 10
}

$logs = (az container logs `
    --resource-group $ResourceGroup `
    --name $ContainerName 2>&1 | Out-String).Trim()

$stamp = Get-Date -Format "yyyy-MM-dd"
$jsonPath = Join-Path $OutputDir "ojs-colab-smoke-$stamp.json"
$mdPath = Join-Path $OutputDir "ojs-colab-smoke-$stamp.md"

if ($logs -match '\{[\s\S]*"schema":\s*"sourceright.ojs_colab_smoke.v1"') {
    $jsonBody = $Matches[0]
    Set-Content -LiteralPath $jsonPath -Value $jsonBody -Encoding utf8
}

@"
# OJS/PKP Colab-equivalent smoke (Azure ACI)

Date: $stamp  
Runner: Azure Container Instances (`python:3.12-alpine`) — Linux substitute for Colab CLI on Windows  
Resource group: $ResourceGroup  
Container: $ContainerName  
Final state: $state

## Log

``````text
$logs
``````

## Notes

- Native Windows cannot run `google-colab-cli` (`fcntl`); this job runs the same
  `scripts/colab/ojs-docker-smoke.py` entrypoint on Linux.
- Full disposable OJS (`pkp/containers` + browser install) still requires a Docker host.
"@ | Set-Content -LiteralPath $mdPath -Encoding utf8

Write-Host "Wrote $mdPath"
if (Test-Path $jsonPath) { Write-Host "Wrote $jsonPath" }
Write-Host "Container state: $state"

az container delete `
    --resource-group $ResourceGroup `
    --name $ContainerName `
    --yes `
    --output none | Out-Null

@{
    markdown_path = $mdPath
    json_path = $(if (Test-Path $jsonPath) { $jsonPath } else { $null })
    container_state = $state
    log = $logs
} | ConvertTo-Json -Depth 3
