# Track 89 Plan

## Phase 1: Workflow harness

- [x] Pin the remaining floating GitHub Action references.
- [x] Add a repository-wide workflow harness for immutable action pins,
  explicit permissions, and non-persisting checkout credentials.
- [x] Run the harness in required quality CI and protect it with a Rust policy test.
- [x] Add bounded job timeouts and concurrency contracts where missing.
  All 18 workflows now declare top-level concurrency and all 31 jobs declare
  validated 1-360 minute timeouts; `scripts/check-workflow-harness.ps1` fails
  closed if either contract regresses.

## Phase 2: Security and supply chain

- [x] Complete the repository security scan and record validated findings.
  `cargo-audit`, `cargo-deny`, `zizmor`, and `actionlint` passed on
  2026-07-12; no actionable findings were emitted.
- [x] Reconcile cargo-audit, cargo-deny, dependency review, CodeQL, Scorecard,
  secret boundaries, SBOMs, provenance, and dependency-update automation.
  Workflow controls, Dependabot, attestations, and Cargo metadata SBOMs are
  wired; local cargo-audit/cargo-deny/zizmor execution is now verified.
- [x] Add deterministic policy tests for every adopted control.

## Phase 3: Quality and verification depth

- [ ] Separate fast PR sensors from scheduled fuzz, mutation, stress, and
  benchmark lanes with explicit budgets and failure artifacts.
- [ ] Raise coverage and regression contracts only when the baseline proves
  the threshold is stable.
- [ ] Verify docs, schemas, examples, CLI, MCP, and packaging end to end.

## Phase 4: Release CI/CD

- [x] Prove dry-run and tagged-release parity.
  `scripts/check-release-parity.ps1` and `tests/provenance_policy.rs` enforce
  shared build, package, dependency, publish-dry-run, attestation, and SBOM
  controls.
- [ ] Enforce artifact immutability, checksums, SBOMs, attestations,
  provenance, environment gates, and rollback evidence.
- [ ] Keep publication claims tied to live acceptance evidence.

## Phase 5: MCP registry scorecards

- [x] Inventory every current Sourceright MCP registry listing and its scoring criteria.
- [ ] Produce dated, reproducible scorecards for Official MCP Registry,
  Smithery, Glama, and any additional live directory.
- [ ] Remediate coding, metadata, install, tool-discovery, security, and
  documentation gaps until each attainable score is 100/100.
- [ ] Record external blockers without claiming unverified scores.

## Phase 6: Review and closeout

- [x] Run full Rust, docs, schema, package, benchmark, and release validation.
  Rust, schema, workflow, release, interoperability, and package security
  gates passed on 2026-07-12.
- [ ] Run `$conductor-review` and address all actionable findings.
- [ ] Synchronize GitHub issues and Project status with repository evidence.

## Bounded parser campaign increment, 2026-11-20

- [x] Configure the existing two parsers for 30 seconds on relevant PRs and 300 seconds on scheduled/manual campaigns, retaining source/compiler logs and minimized failures for 14 days.
- [x] Pin the fuzz-only nightly and cargo-fuzz; cap target inputs at 64 KiB and libFuzzer RSS at 1 GiB with a five-second per-input timeout.
- [x] Validate workflow syntax/expressions with actionlint 1.7.9, changed-target rustfmt and `git diff --check`.
- [ ] Qualify native campaign execution and default sanitizers on the candidate head.

This increments Phase 3 without closing unrelated mutation, stress, benchmark, publication or provider validation obligations. Reproduce retained inputs with the same pinned nightly and `cargo +nightly-2026-11-19 fuzz run TARGET --codegen-units 16 fuzz/artifacts/TARGET/INPUT`. The existing default Rust toolchain is unchanged.

Local native builds were attempted on base
`bd6bc0bf2d1c5066fc093f7c5cd6ad1a2918fd82` with nightly-2026-11-19 and
cargo-fuzz 0.13.2, one Cargo build job and 1,000-input smoke limits. The initial
single-codegen-unit build was killed while compiling Sourceright. A second
build with 16 units was killed while compiling serde-saphyr. Both exited before
campaign execution; neither is passing native evidence or a parser finding.
The second attempt selected local `ASAN_OPTIONS=detect_leaks=0` because this
workspace cannot run LeakSanitizer under ptrace, but compilation still failed.
Hosted CI retains the default sanitizer configuration. Its actual execution
is required before this candidate can be admitted.
