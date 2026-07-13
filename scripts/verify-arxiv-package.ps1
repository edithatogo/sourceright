[CmdletBinding()]
param(
    [string]$PackagePath = "arxiv"
)

$ErrorActionPreference = "Stop"
$required = @("sourceright.tex", "references.bib", "00README", "README.md")
foreach ($name in $required) {
    if (-not (Test-Path -LiteralPath (Join-Path $PackagePath $name) -PathType Leaf)) {
        throw "arXiv package is missing $name"
    }
}

$forbidden = @(".env", ".git", "target", "build", "*.aux", "*.log", "*.out", "*.pdf")
foreach ($pattern in $forbidden) {
    $matches = Get-ChildItem -LiteralPath $PackagePath -Force -Recurse -ErrorAction SilentlyContinue |
        Where-Object { $_.Name -like $pattern }
    if ($matches) {
        throw "arXiv package contains forbidden submission material: $pattern"
    }
}

$tex = Get-Content -LiteralPath (Join-Path $PackagePath "sourceright.tex") -Raw
foreach ($needle in @('\bibliography{references}', 'technical preview', 'GROBID-inspired', 'does not submit to arXiv')) {
    if ($tex -notmatch [regex]::Escape($needle)) {
        throw "sourceright.tex is missing required boundary/content marker: $needle"
    }
}

Write-Host "arXiv package structure and claim boundaries passed."
