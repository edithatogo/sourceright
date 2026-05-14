# Public Release And Registry Submission Spec

## Goal

Complete the live publication path from a reviewed tag to public artifacts in
the accepted core release channels, and record the prepared/deferred boundaries
for every other registry or index in the repo's declared distribution surface.

## Scope

- Execute the release workflows against real tags after validation passes.
- Publish the Rust crate only after `cargo publish --dry-run --locked` succeeds.
- Publish the MCP distribution metadata and image only after the image and
  manifest checks are green.
- Record checksums, release notes, and submission evidence as part of the
  release record.
- Keep manual approval gates where a registry or release channel requires them.
- Classify registries as accepted, prepared, deferred, or not applicable without
  treating prepared metadata as public acceptance.

## Outputs

- Release runbook for the full publication sequence.
- Registry submission evidence and artifact references.
- Public release checklist tied to the existing workflows.
- Accepted/prepared/deferred registry status in the release-status evidence.

## Boundaries

This track must not bypass manual approval or clean-tree checks. It may improve
automation around release steps, but it must not auto-publish from an
unreviewed commit.
