# Release Runbook

Sourceright release work is gated by a clean tree, locked builds, dependency
policy checks, and explicit publication approval.

## Sequence

1. Confirm the tag or release candidate has already passed CI.
2. Run `cargo package --locked`.
3. Run `cargo publish --dry-run --locked` with `CARGO_REGISTRY_TOKEN` set only
   in the publish environment.
4. Validate the GitHub Release artifacts and checksums.
5. Validate the MCP image, `server.json`, and MCP registry metadata.
6. Publish the crate only after the dry run succeeds.
7. Publish the GitHub Release only after the artifacts are staged.
8. Submit the MCP metadata only after the image and labels match the manifest.

On `v*.*.*` tags, the crate publish workflow now runs automatically and the
MCP registry workflow follows the release workflow completion; the manual
dispatch path remains available as an override.

## Evidence

- Release notes generated from the tag.
- Checksum files for the platform binaries.
- Artifact attestations for the binaries and crate package where supported.
- Dry-run logs for crate publication.
- Registry submission evidence for MCP distribution.
- `release-status.md` from each tag-triggered publish workflow.

## Notes

This runbook does not replace the release workflows. It documents the manual
sequence those workflows already enforce.
