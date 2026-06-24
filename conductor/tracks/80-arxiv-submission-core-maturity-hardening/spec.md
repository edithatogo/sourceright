# Track 80: arXiv submission-core Maturity Hardening

## Goal

Raise the legacy arXiv `arxiv-submission-core` adapter to migration-safe,
upstream-ready maturity, stability, and testing evidence.

## User Outcome

The legacy-platform adapter has domain/event compatibility proof, backward
compatibility fixtures, and a clearly scoped upstream submission packet.

## Scope

- Legacy event/domain fixtures for submission state and metadata signals.
- Migration-safe output mapping into `sourceright.journal_screening.v1`.
- Backward-compatible behavior for old submission-core shapes.
- Maintainer-facing evidence for issue/PR submission.

## Out Of Scope

- Changing legacy arXiv state.
- Depending on live credentials in default CI.
- Upstream issue/PR creation before Track 81 approval.

## Data Contracts

The adapter maps legacy submission-core evidence into screening output without
changing platform data, canonical CSL, or provider evidence.

## Claim Boundary

This track may claim legacy upstream-readiness only after hardening gates pass.
It must not claim upstream acceptance.

## Evidence Level Target

Hardened local package and submission-ready evidence.

## Parallelization Plan

Legacy fixtures, migration mapping, and maintainer-facing docs can run in
parallel after Track 78 freezes requirements.

## Maturity, Stability, And Testing

Maturity requires domain/event compatibility, migration-safe mapping, regression
fixtures, negative tests, and explicit fallback behavior for unknown legacy
events.
