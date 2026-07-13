# arXiv submission package update

Date: 2026-07-12

## Change

Added a conservative technical-preview manuscript package at `arxiv/`:

- `sourceright.tex`
- `references.bib`
- `00README`
- `README.md`

The manuscript describes the canonical CSL/verification-sidecar boundary,
read-only workflow, fixture-backed evidence levels, optional GROBID adapter,
and open journal/preprint integration roadmap. It avoids claims of citation
truth, benchmark superiority, production readiness, arXiv acceptance, or
platform support.

## arXiv alignment

The official arXiv TeX guidance requires a tidy source package, warns against
extraneous files, requires the bibliography inputs needed by the selected
processor, and removes hidden files at announcement. The package therefore
contains no generated PDF, build output, repository archive, credentials, or
hidden working directory. See:

https://info.arxiv.org/help/submit_tex.html

## Validation

- `scripts/verify-arxiv-package.ps1` passed.
- `cargo fmt --all --check` passed.
- `git diff --check` passed.
- PDF compilation is deferred because no Python or TeX runtime is available on
  the current workstation.

## Status boundary

This is a local submission candidate. It has not been uploaded to arXiv and
has no arXiv identifier or acceptance evidence. Human review remains required
for authorship, category, license, endorsement, and final PDF/source review.
