# StandardFlow integration handoff

## Status

- Contract status: source implemented.
- Evidence level: source verified after the local validator passes.
- Activation status: prepared, not enabled as a runtime dependency.
- Artifact status: mutable pull-request head; immutable post-merge receipt required.

## Scope

Expose bounded citation-verification evidence to StandardFlow without changing Sourceright's canonical CSL or verification-sidecar authority. `c5fa583431390eee1bf5eae04dc47b01c50d4a1e` is the producer-logic baseline, not a false claim that it contains the new integration artifacts. Pull request 105 and the explicit paths provide the current artifact location.

The exact consumer revision is `f1e7b6c6e283865c1b7dbb34dbe33d8bf6f6c799`; its contract, Rust and legacy document-build gates passed, while merge and downstream adoption remain unclaimed.

## Remaining gates

- [ ] StandardFlow pull request 59 is merged or the consumer pin is deliberately rebased.
- [ ] The merged artifact-bearing Sourceright revision replaces pull-request provenance in a retained receipt.
- [ ] Consumer-driven fixture validation runs in both repositories.
- [ ] Rust adapter and feature flag are implemented without a canonical-write path.
- [ ] Downstream canary, degraded mode and rollback are exercised.
- [ ] Rights, privacy and hostile-document review passes.
- [ ] Any public interoperability claim receives an explicit evidence receipt.

This handoff does not alter the status of existing completed Sourceright tracks.
