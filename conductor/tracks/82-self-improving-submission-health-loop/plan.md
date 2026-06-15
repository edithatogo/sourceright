# Track 82 — Implementation Plan

## Phase 1: Discover [ ]

- [ ] Audit all 10 submission surfaces (tracks 73–81, plus this track's own health loop) and capture per-surface:
  - Current evidence level from `conductor/evidence-ledger.json`.
  - Number of gates (checkboxes) and passed gates from each track's `plan.md`.
  - Blocker summaries from `evidence-ledger.json` entries.
  - Surface category (publication, integration, provider, arxiv).
- [ ] Identify the health score formula: `health_score = (sum of surface health_contributions / surface_count) * 10.0`, where each surface contributes `gates_passed / total_gates` as a float.
- [ ] Identify surface coverage requirement: all 10 surfaces must be present in the inventory with non-zero total gates.
- [ ] Document inventory schema and update cadence in this track's spec.

## Phase 2: Lock spec [ ]

- [ ] Formalise `submission-inventory.json` schema (defined in spec.md Data contracts section).
- [ ] Formalise health score formula and surface coverage rule.
- [ ] Formalise CI gate: `submission_health_policy.rs` asserts health >= 9.5 when `SOURCERIGHT_CLAIM_GATE=1`.
- [ ] Formalise claim boundary: "health-monitored" is not "fully-automated".
- [ ] Draft and lock spec.md, plan.md, and test-matrix.md.

## Phase 3: Implement — submission-inventory.json [ ]

- [ ] Create `conductor/submission-inventory.json` with initial readiness matrix for all 10 surfaces.
- [ ] Populate each surface entry with:
  - Track ID and human-readable name.
  - Category classification.
  - Gates passed / total gates (computed from each track's `plan.md`).
  - Evidence level from `evidence-ledger.json`.
  - Health contribution (gates_passed / total_gates).
  - Blocker summary from `evidence-ledger.json`.
  - Last updated date.
- [ ] Compute initial health score.
- [ ] Validate `submission-inventory.json` against its schema.

## Phase 4: Implement — check-submission-readiness.ps1 [ ]

- [ ] Create `scripts/check-submission-readiness.ps1` that:
  - Reads `conductor/submission-inventory.json`.
  - Validates JSON schema (all required fields present, types correct).
  - Verifies all 10 surfaces are present.
  - Computes health score from the inventory's `health_contribution` values.
  - Checks minimum evidence level per surface (contracted minimum).
  - Exits 0 on pass, 1 on failure, outputs structured JSON result.
- [ ] Run script manually to verify it produces correct output with current inventory.
- [ ] Ensure script is callable from CI workflow (does not require interactive input).

## Phase 5: Implement — submission_health_policy.rs [ ]

- [ ] Create `tests/submission_health_policy.rs` that:
  - Reads `conductor/submission-inventory.json`.
  - Deserializes to a Rust struct matching the schema.
  - Asserts all 10 surfaces are present.
  - Asserts each surface has `total_gates > 0`.
  - Asserts `health_score >= 9.5` when env var `SOURCERIGHT_CLAIM_GATE=1` is set.
  - Asserts `health_score >= 0.0` in default CI (informational).
  - Asserts all surface IDs correspond to known tracks.
- [ ] Run `cargo test --test submission_health_policy` to verify compilation and default (ungated) pass.
- [ ] Run with `SOURCERIGHT_CLAIM_GATE=1` to verify gated behavior (expected to fail until health reaches 9.5).

## Phase 6: Run checks [ ]

- [ ] `cargo fmt --check` on all new Rust test code.
- [ ] `cargo clippy --all-targets -- -D warnings` passes.
- [ ] `cargo test` passes (ungated health policy test is informational).
- [ ] `scripts/check-submission-readiness.ps1` exits 0 with valid output.
- [ ] `cargo test --test submission_health_policy` passes in default mode.
- [ ] All surfaces verified present in inventory.
- [ ] Inventory schema validated by both the PS1 script and Rust test.

## Phase 7: conductor-review [ ]

- [ ] Run `$conductor-review` gate for this track.
- [ ] Record review findings and any required fixes.
- [ ] Address findings from review.

## Phase 8: Apply fixes [ ]

- [ ] Fix any schema, formula, or data issues found during review.
- [ ] Fix any CI integration issues (workflow steps to run the PS1 script and Rust test).
- [ ] Re-run checks after fixes.

## Phase 9: Progress [ ]

- [ ] Health score gated at 9.5 in CI for external claim promotion.
- [ ] Inventory update procedure documented for maintainers.
- [ ] Track marked as complete only when all 10 surfaces have health >= 9.5 and CI gate is active.
- [ ] Update `conductor/evidence-ledger.json` with this track's evidence level.
- [ ] Update `docs/src/release-status.md` to reflect health-monitored capability.
