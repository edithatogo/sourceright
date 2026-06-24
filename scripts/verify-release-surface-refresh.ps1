param(
    [string]$ReleaseTag = "v0.1.20"
)

$ErrorActionPreference = 'Stop'

function Add-Missing {
    param(
        [System.Collections.Generic.List[string]]$Missing,
        [string]$Message
    )
    $Missing.Add($Message)
}

function Assert-Markers {
    param(
        [string]$Path,
        [string[]]$Markers,
        [System.Collections.Generic.List[string]]$Missing
    )

    if (-not (Test-Path -LiteralPath $Path)) {
        Add-Missing $Missing "missing file: $Path"
        return
    }

    $content = Get-Content -Raw -LiteralPath $Path
    foreach ($marker in $Markers) {
        if (-not $content.Contains($marker)) {
            Add-Missing $Missing "missing marker in ${Path}: $marker"
        }
    }
}

function Split-MarkdownRow {
    param([string]$Line)

    return $Line.Trim().Trim('|').Split('|') | ForEach-Object { $_.Trim() }
}

function Test-SeparatorRow {
    param([string[]]$Cells)

    return ($Cells | Where-Object { $_ -notmatch '^\s*:?-{3,}:?\s*$' }).Count -eq 0
}

function Get-SectionTable {
    param(
        [string]$Path,
        [string]$Heading
    )

    $lines = Get-Content -LiteralPath $Path
    $inSection = $false
    $rows = @()

    foreach ($line in $lines) {
        if ($line.Trim() -eq $Heading) {
            $inSection = $true
            continue
        }

        if (-not $inSection) {
            continue
        }

        if ($line -match '^#{1,6}\s+') {
            break
        }

        if ($line.Trim().StartsWith('|')) {
            $rows += $line
        } elseif ($rows.Count -gt 0 -and $line.Trim() -eq '') {
            break
        }
    }

    if ($rows.Count -lt 2) {
        throw "No markdown table found after heading '$Heading' in $Path"
    }

    $headers = @(Split-MarkdownRow $rows[0])
    $items = @()
    foreach ($row in $rows[1..($rows.Count - 1)]) {
        $cells = @(Split-MarkdownRow $row)
        if (Test-SeparatorRow $cells) {
            continue
        }

        $item = [ordered]@{}
        for ($i = 0; $i -lt $headers.Count; $i++) {
            $item[$headers[$i]] = if ($i -lt $cells.Count) { $cells[$i] } else { '' }
        }
        $items += [pscustomobject]$item
    }

    return $items
}

function Get-ReleaseRows {
    param([string]$Path)

    $rows = @()
    foreach ($heading in @('### Accepted Registries', '### Prepared Registries', '### Deferred Registries', '### Not Applicable')) {
        $rows += Get-SectionTable -Path $Path -Heading $heading
    }
    return $rows
}

function Normalize-Link {
    param([string]$Text)

    return ($Text -replace '\]\(release-surface-refresh\.md\)', '](release-surface-refresh)' `
                  -replace '\]\(guides/release-surface-refresh\)', '](release-surface-refresh)' `
                  -replace '\s+', ' ').Trim()
}

function Assert-ReleaseStatusParity {
    param(
        [string]$SourcePath,
        [string]$SitePath,
        [System.Collections.Generic.List[string]]$Missing
    )

    $sourceRows = Get-ReleaseRows $SourcePath
    $siteRows = Get-ReleaseRows $SitePath

    $sourceByRegistry = @{}
    foreach ($row in $sourceRows) {
        $sourceByRegistry[$row.Registry] = $row
    }

    $siteByRegistry = @{}
    foreach ($row in $siteRows) {
        $siteByRegistry[$row.Registry] = $row
    }

    foreach ($registry in $sourceByRegistry.Keys) {
        if (-not $siteByRegistry.ContainsKey($registry)) {
            Add-Missing $Missing "docs-site release status missing registry: $registry"
            continue
        }

        $sourceRow = $sourceByRegistry[$registry]
        $siteRow = $siteByRegistry[$registry]
        foreach ($property in $sourceRow.PSObject.Properties.Name) {
            if (-not ($siteRow.PSObject.Properties.Name -contains $property)) {
                Add-Missing $Missing "docs-site release status missing column '$property' for $registry"
                continue
            }
            $sourceValue = Normalize-Link ([string]$sourceRow.$property)
            $siteValue = Normalize-Link ([string]$siteRow.$property)
            if ($sourceValue -ne $siteValue) {
                Add-Missing $Missing "release status drift for ${registry}/${property}: source='$sourceValue' docs-site='$siteValue'"
            }
        }
    }

    foreach ($registry in $siteByRegistry.Keys) {
        if (-not $sourceByRegistry.ContainsKey($registry)) {
            Add-Missing $Missing "source release status missing registry present in docs-site: $registry"
        }
    }

    foreach ($row in $sourceRows | Where-Object { $_.Status -eq 'accepted' }) {
        foreach ($property in @('Version', 'URL / Evidence', 'Date', 'Install Metadata')) {
            $value = [string]$row.$property
            if ([string]::IsNullOrWhiteSpace($value) -or $value -eq '—') {
                Add-Missing $Missing "accepted registry '$($row.Registry)' missing accepted evidence column: $property"
            }
        }
    }
}

function Assert-TrackEvidenceParity {
    param(
        [string]$ReleaseStatusPath,
        [string]$EvidencePath,
        [System.Collections.Generic.List[string]]$Missing
    )

    $releaseRows = Get-ReleaseRows $ReleaseStatusPath
    $evidenceRows = Get-SectionTable -Path $EvidencePath -Heading '## Current Evidence'

    $releaseStatus = @{}
    foreach ($row in $releaseRows) {
        $releaseStatus[$row.Registry] = $row.Status
    }

    $mapping = [ordered]@{
        'GitHub Release' = @('GitHub Release')
        'crates.io' = @('crates.io')
        'docs.rs' = @('docs.rs')
        'Official MCP Registry' = @('Official MCP Registry')
        'GHCR MCP image' = @('GHCR MCP image')
        'Glama' = @('Glama')
        'Smithery' = @('Smithery')
        'Claude Desktop' = @('Claude Desktop client config')
        'Codex' = @('Codex MCP client config')
        'Generic MCP clients' = @('Generic MCP client config')
        'GitHub Copilot' = @('GitHub Copilot coding-agent prep')
        'Zotero' = @('Zotero')
        'OJS/PKP' = @('OJS/PKP')
        'VS Code Marketplace / Open VSX' = @('VS Code Marketplace / Open VSX')
        'Microsoft AppSource / Word add-in' = @('Microsoft AppSource / Word add-in')
        'LibreOffice Extensions' = @('LibreOffice Extensions')
        'Homebrew, Scoop, winget, npm, PyPI' = @('Homebrew', 'Scoop', 'winget', 'npm launcher', 'PyPI launcher')
        'Chocolatey' = @('Chocolatey')
    }

    foreach ($evidenceRow in $evidenceRows) {
        $hostName = $evidenceRow.'Host or directory'
        if (-not $mapping.Contains($hostName)) {
            Add-Missing $Missing "release-surface verifier has no mapping for evidence row: $hostName"
            continue
        }

        foreach ($registry in $mapping[$hostName]) {
            if (-not $releaseStatus.ContainsKey($registry)) {
                Add-Missing $Missing "release status missing mapped evidence registry: $registry"
                continue
            }
            if ($releaseStatus[$registry] -ne $evidenceRow.State) {
                Add-Missing $Missing "release status mismatch for ${registry}: release='$($releaseStatus[$registry])' evidence='$($evidenceRow.State)'"
            }
        }
    }

    $mappedRegistries = New-Object System.Collections.Generic.HashSet[string]
    foreach ($registries in $mapping.Values) {
        foreach ($registry in $registries) {
            [void]$mappedRegistries.Add($registry)
        }
    }

    foreach ($releaseRow in $releaseRows) {
        if (-not $mappedRegistries.Contains($releaseRow.Registry)) {
            Add-Missing $Missing "release status row has no mapped evidence source: $($releaseRow.Registry)"
        }
    }
}

$missing = New-Object System.Collections.Generic.List[string]

$requiredRows = @(
    '| GitHub Release | accepted |',
    '| crates.io | accepted |',
    '| docs.rs | accepted |',
    '| Official MCP Registry | accepted |',
    '| GHCR MCP image | prepared |',
    '| Glama | prepared |',
    '| Smithery | submitted |',
    '| Claude Desktop client config | prepared |',
    '| Codex MCP client config | prepared |',
    '| Generic MCP client config | prepared |',
    '| GitHub Copilot coding-agent prep | prepared |',
    '| Zotero | prepared |',
    '| OJS/PKP | prepared |',
    '| VS Code Marketplace / Open VSX | prepared |',
    '| Homebrew | deferred |',
    '| Scoop | deferred |',
    '| winget | deferred |',
    '| npm launcher | deferred |',
    '| PyPI launcher | deferred |',
    '| Microsoft AppSource / Word add-in | deferred |',
    '| LibreOffice Extensions | deferred |',
    '| Chocolatey | n/a |'
)

Assert-Markers -Path 'conductor/tracks/69-marketplace-submission-evidence/marketplace-evidence.md' -Missing $missing -Markers @(
    'Prepared metadata is not public marketplace acceptance',
    '| GHCR MCP image | prepared |',
    '| Claude Desktop | prepared |',
    '| Codex | prepared |',
    '| GitHub Copilot | prepared |',
    '| Generic MCP clients | prepared |',
    '| Homebrew, Scoop, winget, npm, PyPI | deferred |',
    '| Chocolatey | n/a |'
)

Assert-Markers -Path 'docs/src/release-status.md' -Missing $missing -Markers (@(
    "latest verified public release surface is ``$ReleaseTag``",
    '[Release Surface Refresh](release-surface-refresh.md)'
) + $requiredRows)

Assert-Markers -Path 'docs-site/src/content/docs/release-status.md' -Missing $missing -Markers (@(
    '[Release Surface Refresh](guides/release-surface-refresh)'
) + $requiredRows)

Assert-Markers -Path 'docs/src/release-surface-refresh.md' -Missing $missing -Markers @(
    'Prepared metadata can move to `accepted` only when the public listing is visible and installable',
    'Local config examples, package templates, source skeletons, or registry-ready metadata are not enough',
    'scripts/verify-release-surface-refresh.ps1',
    'technical preview',
    'pilot-ready',
    'fixture-backed regression benchmark',
    'deterministic benchmark scaffold'
)

Assert-Markers -Path 'docs-site/src/content/docs/guides/release-surface-refresh.md' -Missing $missing -Markers @(
    'title: Release surface refresh',
    'Prepared metadata can move to `accepted` only when the public listing is visible and installable',
    'scripts/verify-release-surface-refresh.ps1'
)

Assert-ReleaseStatusParity -SourcePath 'docs/src/release-status.md' -SitePath 'docs-site/src/content/docs/release-status.md' -Missing $missing
Assert-TrackEvidenceParity -ReleaseStatusPath 'docs/src/release-status.md' -EvidencePath 'conductor/tracks/69-marketplace-submission-evidence/marketplace-evidence.md' -Missing $missing

if ($missing.Count -gt 0) {
    $missing | ForEach-Object { Write-Error $_ }
    exit 1
}

Write-Host 'Release surface refresh evidence checks passed.'
