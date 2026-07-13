---
title: Release runbook
description: Public release, registry submission, and evidence sequence.
---

Sourceright release work is gated by a clean tree, locked builds, dependency
policy checks, and explicit publication approval.

## Sequence

1. Confirm the tag or release candidate has already passed CI.
2. Run `cargo package --locked`.
3. Run `cargo publish --dry-run --locked` with `CARGO_REGISTRY_TOKEN` set only in the publish environment.
4. Run `scripts/verify-release-surface-refresh.ps1` and confirm accepted, prepared, deferred, and not-applicable release surfaces still match the evidence table.
5. Validate the GitHub Release artifacts and checksums.
6. Validate the MCP image, `server.json`, and MCP registry metadata.
7. Publish the crate only after the dry run succeeds.
8. Publish the GitHub Release only after the artifacts are staged.
9. Submit the MCP metadata only after the image and labels match the manifest.

On `v*.*.*` tags, the crate publish workflow now runs automatically and the MCP
registry workflow follows the release workflow completion; the manual dispatch
path remains available as an override.

## Evidence

- Release notes generated from the tag.
- Checksum files for the platform binaries.
- Artifact attestations for the binaries and crate package where supported.
- Dry-run logs for crate publication.
- Release-surface refresh output from `scripts/verify-release-surface-refresh.ps1`.
- Registry submission evidence for MCP distribution.
- `release-status.md` from each tag-triggered publish workflow.

## Notes

This runbook does not replace the release workflows. It documents the manual
sequence those workflows already enforce.
