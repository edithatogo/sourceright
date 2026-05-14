# Contract Evidence And Overclaim Gates Spec

## Goal

Create a repo-local mechanism that prevents Sourceright from claiming a feature,
registry, benchmark, platform integration, or security posture is complete
unless the required evidence exists.

## Scope

- Treat `conductor/requirements.md` and `conductor/design.md` as the planning
  authority.
- Keep `docs/src/feature-contract-matrix.md` and `docs/src/design.md` as public
  mirrors.
- Add tests that scan README, docs, release notes, and track status language for
  forbidden or unsupported claims.
- Add evidence-level language: contracted, scaffolded, fixture-backed,
  opt-in-live proven, publicly accepted.
- Require each track to record evidence before status promotion.

## Forbidden Claims Until Evidence Changes

- Production-ready institutional platform.
- Examiner-grade final verifier.
- SOTA benchmarked performance.
- AI detector.
- Legal filing compliance system.
- Live provider verified when only fixture-backed tests ran.
- Registry accepted when only metadata is prepared.

## Parallelization

- Subagent A: docs and README wording.
- Subagent B: Conductor track status and metadata.
- Subagent C: tests and policy gates.
- Subagent D: release/publication evidence wording.
