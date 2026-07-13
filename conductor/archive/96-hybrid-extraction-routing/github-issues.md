# GitHub Issue Ledger

## Issue #41

Track 96 hybrid extraction routing and production hardening. This slice adds
explicit manual/auto routing policy, route traces, confidence guards, resource
limits, cache keys, and redaction. Live load, rollback, and backend-quality
evidence remain separately gated.

Review fixes applied: cache keys include route mode, backend availability,
policy, model/config/options, and input-size inputs; manual experimental
selection now respects the opt-in policy.
