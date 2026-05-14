# Track Template

Copy this structure for new Conductor tracks.

## `metadata.json`

Required fields:

- `id`
- `status`
- `priority`
- `owners`
- `dependencies`
- `owned_paths`

## `spec.md`

Required sections:

- Goal
- User outcome
- Scope
- Out of scope
- Data contracts
- Claim boundary
- Evidence level target
- Parallelization plan

## `plan.md`

Required phases:

1. Discover with subagents where useful.
2. Lock spec, plan, and test matrix.
3. Implement the smallest owned-path slice.
4. Run targeted checks.
5. Run `$conductor-review`.
6. Apply local review fixes automatically.
7. Progress only after findings are fixed or deferred.

## `test-matrix.md`

Required columns:

- Scenario
- Acceptance
- Evidence
- Default-CI or opt-in-live
