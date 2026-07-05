# Track 82 — Test Matrix

| Scenario | Acceptance | Evidence | Default-CI or opt-in-live |
|---|---|---|---|
| Submission inventory JSON schema validation | `submission-inventory.json` validates against the schema (all required fields present, types correct, no unknown fields) | `scripts/check-submission-readiness.ps1` output includes `"schema_valid": true`; `tests/submission_health_policy.rs` deserializes without error | default-CI |
| Readiness check script runs and exits 0 | `scripts/check-submission-readiness.ps1` exits with code 0 and outputs valid JSON with `"pass": true` | Script execution output captured in CI logs; `$LASTEXITCODE -eq 0` | default-CI |
| Health score computed | Health score is a float between 0.0 and 10.0, derived from surface health_contributions | `submission-inventory.json` `health_score` field; script output `health_score` field | default-CI |
| All 10 surfaces tracked | `submission-inventory.json` `surfaces` array contains exactly 10 entries (tracks 73–81, plus track 82) | Count assertion in `tests/submission_health_policy.rs`; script validation `surface_count == 10` | default-CI |
| Health >= 9.5 gate | When `SOURCERIGHT_CLAIM_GATE=1`, health score must be >= 9.5 or the test fails | `tests/submission_health_policy.rs` assertion with env-var-gated check | default-CI |
| Inventory updated when new tracks added | Adding a new track requires updating the surface count and entries; the inventory schema supports extensibility | Schema allows `surfaces` array to grow; test asserts count matches known tracks | default-CI |
| All surfaces have non-zero gates | Each surface entry has `total_gates > 0` | `tests/submission_health_policy.rs` iterates all surfaces and asserts `total_gates > 0` | default-CI |
| Track IDs correspond to known tracks | All `track_id` values in inventory match actual conductor track directories | `tests/submission_health_policy.rs` verifies each `track_id` exists in `conductor/tracks/` | default-CI |
| Evidence level minimum enforced | All surfaces have `evidence_level` set to at least `"contracted"` in the inventory | `scripts/check-submission-readiness.ps1` validates evidence_level values against the known levels list | default-CI |
| Health score formula reproducible | Manually computing health score from individual surface health_contributions matches the `health_score` field | Formula: `health_score = (sum(health_contribution) / surface_count) * 10.0`; verified by policy test | default-CI |
| Claim boundary documented | All track evidence docs include disclaimer: health-monitored, not fully-automated | Review of evidence docs in this track | default-CI |
