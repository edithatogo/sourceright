# Security and Quality Gates

Sourceright treats security, quality, and publication as separate evidence
surfaces. A green local check does not prove that a GitHub setting, registry
listing, or external deployment is healthy.

## Fast local gates

Run these from the repository root:

```powershell
pwsh -NoProfile -ExecutionPolicy Bypass -File scripts/run-local-rust-gates.ps1
pwsh -NoProfile -ExecutionPolicy Bypass -File scripts/check-workflow-harness.ps1
pwsh -NoProfile -ExecutionPolicy Bypass -File scripts/check-release-parity.ps1
```

The Rust runners use a dedicated temporary target directory. The POSIX runner
uses the host stable toolchain; the PowerShell runner selects the Windows GNU
toolchain only when GCC is available. The workflow harness checks full-SHA action pinning, least-privilege
permissions, non-persisting checkout credentials, bounded job timeouts, and
concurrency declarations. Release parity checks keep dry-run and publication
paths aligned, including provenance attestation and dependency policy checks.

## Deep and scheduled checks

The GitHub workflows provide separate lanes for dependency policy, CodeQL and
Scorecard, coverage, mutation, fuzz/robustness, documentation, release dry
runs, and optional interoperability fixtures. Optional third-party runners are
bounded and must not weaken the mandatory Rust gates.

## Report safety boundary

`references.csl.json` remains the canonical bibliographic source. Provider
evidence belongs in `references.verification.json`; derived review work belongs
in `review-queue.jsonl`. `sourceright report` supports CSL-only directories by
returning a degraded-coverage diagnostic. If a sidecar exists but is malformed,
the error identifies the sidecar path and tells the operator to repair or move
it aside before rerunning the report.

## Publication evidence

Release, crate, MCP registry, Glama, Smithery, and documentation claims must be
supported by dated workflow or external listing evidence. In particular, a
repository manifest is not evidence that a registry has accepted, deployed,
or exposed callable tools. See `conductor/submission-packets/live-evidence.json`
and the relevant Conductor track for the current boundary and residual action.
