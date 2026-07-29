# arXiv Upstream Submission Packet

## Requirements Evidence

- arXiv/submit-ce
- arXiv/arxiv-submission-core

## Local Gates

- `scripts/verify-submission-readiness.ps1`
- `scripts/verify-arxiv-package.ps1`
- LaTeX compilation of `arxiv/sourceright.tex` in an environment with an
  available TeX runtime.

## Draft

The repository now contains an improved technical-preview manuscript bundle
under `arxiv/`. It is a local submission candidate, not an arXiv submission.
The package contains only source, bibliography, and submission instructions;
the build directory and repository working files are excluded.

## Approval Gate

Explicit approval is required before any external submission is created.

## Blockers

None.

## Acceptance boundary

The upstream issues are submitted, but no arXiv maintainer acceptance is
claimed. Acceptance remains a separate external state.

## Local limitations

- No local TeX runtime is installed on the current Windows workstation, so a
  PDF compilation has not been verified here.
- Author identity, category selection, endorsement, license selection, and
  final source-package review still require the human submitter.

## Current manuscript package

- `arxiv/sourceright.tex` — technical-preview manuscript.
- `arxiv/references.bib` — GROBID, arXiv guidance, and repository references.
- `arxiv/00README` — arXiv-oriented source-package note.
- `arxiv/README.md` — local build and claim-boundary instructions.

The manuscript explicitly states that Sourceright is GROBID-inspired, not a
GROBID fork or compatibility claim; that it is a technical preview; and that
it does not submit to or mutate arXiv state.
Track 78 requirements matrix complete at the contracted evidence level; see
`conductor/tracks/78-arxiv-upstream-requirements-recon/requirements-matrix.md`.
No upstream write or acceptance claim is made.
Track 80 legacy submission-core evidence is recorded in
`conductor/tracks/80-arxiv-submission-core-maturity-hardening/evidence-packet.md`,
`security-boundaries.md`, and `migration-mapping-check-2026-06-09.md`. The
legacy surface has no active elevation path; migrate to submit-ce via Track 79.
Track 81 readiness review complete at the submitted evidence level; upstream acceptance remains unverified.
