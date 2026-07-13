# Track 89: Harness Engineering, Security, Quality, and CI/CD Hardening

## Intent

Turn Sourceright's already strong automation into an explicit harness: every
important engineering claim must have a deterministic sensor, bounded execution,
retained failure evidence, a remediation route, and a policy test preventing
silent regression.

## Requirements

- All third-party GitHub Actions use immutable full commit SHAs.
- Workflow permissions are explicit and least privilege; checkout credentials
  are never persisted without a documented exception.
- Fast pull-request checks and slower scheduled verification are separate,
  bounded, and concurrency-aware.
- Dependency, advisory, code-scanning, secret, fuzz, mutation, coverage,
  benchmark, docs, and release controls fail closed where evidence permits.
- Releases produce checksums, SBOMs, provenance, attestations, and rollback
  evidence without broadening product claims.
- MCP registry scores are recorded from live registry evidence. A 100/100 claim
  is allowed only where the registry itself or a reproducible local scorecard
  proves it.

## Boundaries

- Security automation must not weaken canonical CSL/verification-sidecar
  separation or enable MCP writes by default.
- External publication, marketplace submission, secrets, and repository
  settings remain explicit human-approval gates.
- "Bleeding edge" means current, authoritative, pinned, reproducible controls;
  it does not mean floating dependencies or unbounded experimental automation.

## GitHub ledger

- [#25 Workflow harness invariants](https://github.com/edithatogo/sourceright/issues/25)
- [#26 Dependency and supply-chain security](https://github.com/edithatogo/sourceright/issues/26)
- [#27 Test, coverage, fuzz, mutation, and benchmark loops](https://github.com/edithatogo/sourceright/issues/27)
- [#28 Release CI/CD, provenance, and rollback](https://github.com/edithatogo/sourceright/issues/28)
- [#29 MCP registry 100/100 scorecards](https://github.com/edithatogo/sourceright/issues/29)
- [Sourceright Conductor Roadmap Project](https://github.com/users/edithatogo/projects/17)
