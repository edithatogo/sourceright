# Sourceright arXiv submission bundle

This directory is a technical-preview manuscript package for arXiv. It is
not evidence of arXiv submission or acceptance.

## Build locally

From the repository root:

```powershell
python C:\Users\60217257\.codex\plugins\cache\openai-bundled\latex\0.2.4\scripts\compile_latex.py "$PWD/arxiv/sourceright.tex" --output-directory "$PWD/arxiv/build" --json
```

The arXiv upload should contain only the files needed to compile the paper:
`sourceright.tex`, `references.bib`, and `00README`. Do not upload the build
directory, repository archives, credentials, `.env` files, or unrelated
fixtures.

## Claim boundary

The paper describes Sourceright as a technical preview and fixture-backed
reference-verification workflow. It does not claim final citation truth,
production institutional readiness, arXiv acceptance, or benchmark
superiority.
