---
title: Security and quality gates
description: The repository's bounded security, quality, and publication evidence contract.
---

Sourceright separates local code quality, repository security, and external
publication evidence. A green local check does not prove that a registry or
GitHub setting is healthy.

## Fast local gates

```powershell
pwsh -NoProfile -ExecutionPolicy Bypass -File scripts/run-local-rust-gates.ps1
pwsh -NoProfile -ExecutionPolicy Bypass -File scripts/check-workflow-harness.ps1
pwsh -NoProfile -ExecutionPolicy Bypass -File scripts/check-release-parity.ps1
```

On Linux or macOS, use `./scripts/run-local-rust-gates.sh`. The PowerShell
runner selects the Windows GNU toolchain only when GCC is available; both
runners use a temporary target directory.

The workflow harness checks full-SHA action pinning, least-privilege
permissions, non-persisting checkout credentials, bounded job timeouts, and
concurrency declarations. Release parity keeps dry-run and publication paths
aligned.

## Report safety boundary

`references.csl.json` is canonical. Provider evidence belongs in
`references.verification.json`, and derived review work belongs in
`review-queue.jsonl`. `report` supports CSL-only directories with a degraded
coverage diagnostic and gives an actionable path when an existing sidecar is
malformed.

## External evidence

Registry manifests are not proof of accepted listings, deployments, or callable
tools. Use the dated evidence in the Conductor submission packet for current
external state and residual blockers.
