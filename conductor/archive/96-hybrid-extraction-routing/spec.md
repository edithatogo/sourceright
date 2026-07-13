# Hybrid Extraction Routing and Production Hardening Spec

## Goal

Provide explicit, traceable backend selection and safe production policy
contracts without hidden fallback or incomparable confidence scores.

## User outcome

Operators can select a backend or an auditable `auto` policy, see attempts and
fallback reasons, enforce input/resource/privacy limits, and reproduce a route
decision from a stable configuration and cache key.

## Scope

- Manual and auto routing policies for GROBID/native/NER/experimental backends.
- Calibrated confidence records that refuse cross-task comparison.
- Route traces, explicit abstention, resource limits, and redacted diagnostics.
- Deterministic cache keys over document/backend/model/config/options.
- Fixture-backed policy tests; backend execution remains owned by its adapter.

## Out of scope

- Silent cloud fallback, automatic writeback, or universal optimality claims.
- Actual learned-model calibration, load testing, or deployment rollback drills.
- Merging conflicting fields without provenance.

## Data contracts

Every route has policy, attempts, reasons, accepted backend or abstention, and
resource decisions. Manual mode never silently falls back. Auto mode compares
only matching task/calibration IDs. Cache keys include document hash and all
backend/model/config/options fingerprints. Logs redact common secret fields.

## Claim boundary

This slice proves routing and observability policy contracts, not production
throughput, model quality, or universal backend selection.

## Evidence level target

Deterministic fixture-backed route traces, policy failures, cache-key replay,
confidence-comparison guards, and redaction checks.

## Parallelization plan

- Lane A: route schema and deterministic policy.
- Lane B: cache/resilience/resource controls.
- Lane C: security/privacy/redaction.
- Lane D: live load/fault/rollback evidence after backend adapters mature.
