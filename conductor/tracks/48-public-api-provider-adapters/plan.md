# Public API Provider Adapters Plan

1. Inventory manifest fields and current provider abstractions.
2. For each provider, add recorded fixtures for success, no match, ambiguous
   match, malformed response, rate limit, and transient failure.
3. Implement adapter mapping into the common provider-evidence shape.
4. Add opt-in live smoke with timeout, retry, min-interval, and cache controls.
5. Update provider docs and plugin registry status.
6. Run targeted provider tests, then `$conductor-review`.
7. Apply local review fixes automatically and progress provider lanes independently.
