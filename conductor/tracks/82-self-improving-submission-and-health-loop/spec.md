# Track 82: Self-Improving Submission and Health Loop

## Goal

Make submission hardening self-improving by turning host requirements, blockers,
and repo-health controls into machine-readable inventory and CI checks.

## User Outcome

The repository can tell maintainers which submission surfaces are blocked, which
requirements still need search evidence, and whether repo health remains at
least the 9.5 target before external submission claims are made.

## Scope

- Machine-readable submission requirements inventory.
- Maintainer-facing local submission packet drafts.
- Submission-readiness verifier script.
- CI workflow for submission-contract changes.
- Policy tests that require the inventory and workflow to stay in sync.
- Guidance that agents, skills, or workflows are added only when they have a
  stable package or host workflow contract.

## Out Of Scope

- Creating external submissions.
- Calculating a subjective health score from live external services.
- Adding host-specific agents before package paths are proven.

## Data Contracts

`conductor/submission-requirements.json` is the machine-readable source of truth
for Track 72-81 submission readiness. Markdown contracts remain the human-facing
explanation. `conductor/submission-packets/manifest.json` is the
machine-readable packet index that binds packet paths, surfaces, blockers, local
validation, and approval gates.

## Claim Boundary

This track may claim readiness automation and a repo-health target. It must not
claim that every surface is mature, submitted, or accepted.

## Evidence Level Target

Fixture-backed local verification.

## Parallelization Plan

Inventory expansion, verifier hardening, and workflow wiring can run in
parallel after the JSON field contract is stable. Host-specific remediation
continues in Tracks 73-81.

## Maturity, Stability, And Testing

Maturity requires deterministic default-CI checks, non-zero verifier failures,
docs parity, and policy tests that fail when a required submission surface has no
inventory row.
