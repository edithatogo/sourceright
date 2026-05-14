# Public API Provider Adapters Test Matrix

| Scenario | Acceptance |
| --- | --- |
| Per-provider fixtures | Each public provider has success, no-match, ambiguous, malformed, rate-limit, and transient-failure fixtures. |
| Sidecar-only evidence | No adapter writes to `references.csl.json`. |
| Confidence semantics | Confidence and conflicts are deterministic for each provider. |
| Opt-in live smoke | Live calls skip cleanly unless explicitly enabled. |
| Review loop | `$conductor-review` runs and local fixes are applied before promotion. |
