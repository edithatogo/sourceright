# Track 82 — Self-improving submission and health loop

## Goal

Submission readiness is always current, gaps are surfaced in CI, and repo health is measurable against a target of at least 9.5 before external claims are promoted.

## User outcome

Maintainers and reviewers can inspect a single machine-readable inventory of all 10 submission surfaces (tracks 73–81, plus this track's own health loop), see per-surface readiness at a glance, and rely on CI to gate the repo-health score below 9.5 from producing external acceptance claims.

## Scope

- Machine-readable `submission-inventory.json` documenting all tracked surfaces, their current readiness gates, evidence level, blocker summary, and health score contribution.
- CI-eligible readiness check script (`scripts/check-submission-readiness.ps1`) that validates the inventory against actual on-disk track files and computes the health score.
- Policy test (`tests/submission_health_policy.rs`) that enforces the health-score threshold and surface-coverage requirements in CI.
- Dashboard surface: the inventory JSON is designed to be consumed by docs-site or any external rendering tool without modification.
- Per-track health aggregation: each surface's readiness is derived from the owning track's `evidence-ledger.json` entry, `plan.md` checkboxes, and `test-matrix.md` coverage.
- Update cadence: inventory is regenerated or manually updated whenever a tracked surface changes readiness state (new blockers resolved, evidence level promoted, or new tracks added).

## Out of scope

- Automatic external submissions to any platform (MCP directories, app stores, arXiv, etc.).
- GitHub API modification (no settings changes, no branch protection writes, no issue/PR creation from this track).
- Self-modifying track state (the inventory is human-maintained or regenerated via script; no conductor track mutates its own metadata).
- Replacing existing submission contracts (track 72 owns the contract layer; this track reads from it but does not override it).
- Real-time monitoring or webhook subscriptions (all checks are CI-triggered or manual).
- Health score below 9.5 does not block development work — it only gates external-facing acceptance claims.

## Data contracts

- **`conductor/submission-inventory.json`** — schema:

```jsonc
{
  "schema": "sourceright.conductor.submission-inventory.v1",
  "generated_at": "<ISO-8601 timestamp>",
  "surfaces": [
    {
      "track_id": "<string>",
      "name": "<human-readable surface name>",
      "category": "<publication|integration|provider|arxiv>",
      "readiness": {
        "gates_passed": <integer>,
        "total_gates": <integer>,
        "evidence_level": "<evidence-ledger level>",
        "health_contribution": <float 0.0–1.0>
      },
      "blockers": ["<string>", ...],
      "last_updated": "<ISO-8601 date>"
    }
  ],
  "health_score": <float 0.0–10.0>,
  "health_target": 9.5,
  "surface_count": <integer>
}
```

- **`scripts/check-submission-readiness.ps1`** — outputs structured JSON with `{ "pass": bool, "health_score": float, "errors": [string], "warnings": [string] }`.
- **`tests/submission_health_policy.rs`** — reads `submission-inventory.json`, validates schema, computes score, and asserts `health_score >= 9.5` when `SOURCERIGHT_CLAIM_GATE=1`.

## Claim boundary

> **"health-monitored" is not "fully-automated"** — the inventory and health score are maintained with human oversight and CI enforcement. No autonomous external submission, self-healing, or self-modifying track state is claimed.

All evidence docs in this track must include the disclaimer: *health-monitored, not fully-automated*.

## Evidence level target

**fixture-backed** — the highest evidence level achievable for a governance/automation loop that does not perform live external submissions. Achieved when:
1. `submission-inventory.json` exists and passes schema validation.
2. `check-submission-readiness.ps1` runs and exits 0 with a valid score.
3. `submission_health_policy.rs` passes with health score ≥ 9.5 (when gated).

## Parallelization plan

- **Inventory creation** and **script implementation** can run in parallel because the JSON schema is defined upfront and both sides can validate against the same contract.
- **Policy test** depends on the inventory and script both existing, so it must come after both are implemented.
- **Health-score CI gating** is the final step and depends on all three artifacts being stable.
