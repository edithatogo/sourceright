# Licensed BYO-Key Provider Adapters Test Matrix

| Scenario | Acceptance |
| --- | --- |
| Missing credentials | Provider skips with clear diagnostics and no network call. |
| Redaction | Secrets never appear in logs, cache keys, reports, or test output. |
| Fixture evidence | Sample payloads map to sidecar evidence deterministically. |
| Default CI | Licensed live calls are never required in default CI. |
| Review loop | `$conductor-review` runs and local fixes are applied. |
