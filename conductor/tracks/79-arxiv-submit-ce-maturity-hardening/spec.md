# Track 79: arXiv submit-ce Maturity Hardening

## Goal

Raise the current arXiv `submit-ce` adapter from fixture-backed local support to
upstream-ready maturity, stability, and testing evidence.

## User Outcome

The current-platform adapter has stable fixture coverage, API/schema drift
detection, security boundaries, and maintainer-facing evidence before upstream
submission is attempted.

## Scope

- `submit-ce` source-bundle and metadata screening fixtures.
- API/schema compatibility checks against the current repository.
- Stability budget for fixture drift, optional live/local development smoke, and
  no-writeback guarantees.
- Maintainer-facing issue/PR evidence packet.

## Out Of Scope

- Submitting papers to arXiv.
- Mutating arXiv systems.
- Upstream issue/PR creation before Track 81 approval.

## Data Contracts

The adapter consumes synthetic or exported submission metadata and emits
`sourceright.journal_screening.v1`. It must not mutate canonical CSL or arXiv
state.

## Claim Boundary

This track may claim `submit-ce` upstream-readiness only after hardening gates
pass. It must not claim upstream acceptance.

## Evidence Level Target

Hardened local package and submission-ready evidence.

## Parallelization Plan

Fixture expansion, schema drift detection, and maintainer-facing docs can run in
parallel after Track 78 freezes requirements.

## Maturity, Stability, And Testing

Maturity requires compatibility matrix coverage, deterministic fixture tests,
negative/error fixtures, drift detection, optional local `submit-ce` smoke, and
security review. Stability requires no platform writeback and no dependency on
live credentials in default CI.
