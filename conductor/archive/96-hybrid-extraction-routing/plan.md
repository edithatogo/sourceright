# Hybrid Extraction Routing and Production Hardening Plan

1. **[x] Discover.** Inspect current GROBID safety config, neutral backends,
   entity/reference boundaries, and historical routing design.
2. **[x] Lock documents.** Define route modes, trace fields, confidence
   comparability, resource limits, cache inputs, and redaction rules.
3. **[x] Implement.** Add manual/auto routing policy, traces, cache keys,
   resource guards, and redaction helpers with fixtures.
4. **[x] Validate.** Run formatting, fixture, serialization, policy, and
   workflow checks; attempt Rust tests and record toolchain limits.
5. **[x] Review locally.** Check hidden fallback, score mixing, secret leakage,
   deterministic cache behavior, and resource safety.
6. **[x] Apply fixes.** Resolve local findings before closeout.
7. **[x] Closeout.** Record live load, rollback, and backend-quality deferrals;
   leave archive cleanup to review.

## Review fixes

- [x] Prevent empty secret values from causing a redaction loop.
- [x] Add an incomparable-calibration regression test.
- [x] Include route mode, backend availability, policy, and config inputs in
  cache keys; enforce experimental policy in manual mode.

## Completion signal

Manual and auto route decisions produce deterministic traces, fail safely under
resource/privacy policy, refuse incomparable confidence, and expose reproducible
cache keys without changing backend semantics.

## Validation note

`cargo test` was attempted with the locked dependency graph but remains blocked
by the workstation linker: host build scripts resolve Git's Unix `link.exe`
instead of a usable MSVC linker. Formatting, diff, fixture, and workflow checks
are the available local evidence.
