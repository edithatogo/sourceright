# Track 89 Context

- [Specification](./spec.md)
- [Implementation plan](./plan.md)
- [Test matrix](./test-matrix.md)
- [Metadata](./metadata.json)
- [GitHub issue ledger](./github-issues.md)
- [MCP registry scorecards](./mcp-registry-scorecards-2026-07-11.md)

## Latest implementation evidence

- Workflow harness hardening completed for the timeout/concurrency slice on
  2026-07-12.
- Validation on 2026-07-12: the workflow harness, `actionlint`, formatting,
  all locked Rust tests, clippy, locked check, `cargo-audit`, `cargo-deny`,
  `zizmor`, release parity, and npm audit all passed. The local gate script
  now fails closed on native command exit codes.
- The local security evidence gap is closed for the repository's adopted
  static controls. External registry rescans and human acceptance gates remain
  explicitly external and are not represented as local scores.
- Supply-chain slice added on 2026-07-12: weekly Dependabot coverage for Cargo,
  GitHub Actions, and Docker; Cargo metadata SBOM artifacts on crate release
  paths; and `tests/supply_chain_policy.rs` for scan, update, attestation,
  checksum, and SBOM contract presence. Static checks pass; the focused Rust
  policy test timed out under the existing local toolchain limitation.
- Release-parity slice added on 2026-07-12: dry-run now generates and checks a
  Cargo dependency SBOM; `scripts/check-release-parity.ps1` enforces shared
  release controls. Local security CLI availability remains the only local
  Track 89 validation gap.
- Local Rust execution is now reproducible through
  `scripts/run-local-rust-gates.ps1`, which selects the installed GNU
  toolchain/linker and keeps generated target artifacts outside OneDrive.
- Dependabot alert #14 for `openssl` (CVE-2026-45784) was remediated on
  2026-07-13 by updating the lockfile to patched `openssl` 0.10.80. Local
  `cargo-audit`, `cargo-deny`, and the full GNU Rust gates pass; GitHub now
  reports the alert as fixed.
