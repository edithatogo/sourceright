# StandardFlow integration handoff

## Status

- Contract status: source implemented.
- Evidence level: source verified after the local validator passes.
- Activation status: prepared, not enabled as a runtime dependency.

## Scope

Expose bounded citation-verification evidence to StandardFlow without changing Sourceright's canonical CSL or verification-sidecar authority. The exact proposed consumer revision is `820898e7ae21784145f98e04d8fc482367a6f015`.

## Remaining gates

- [ ] StandardFlow pull request 59 is merged or the consumer pin is deliberately rebased.
- [ ] Consumer-driven fixture validation runs in both repositories.
- [ ] Rust adapter and feature flag are implemented without a canonical-write path.
- [ ] Downstream canary, degraded mode and rollback are exercised.
- [ ] Rights, privacy and hostile-document review passes.
- [ ] Any public interoperability claim receives an explicit evidence receipt.

This handoff does not alter the status of existing completed Sourceright tracks.
