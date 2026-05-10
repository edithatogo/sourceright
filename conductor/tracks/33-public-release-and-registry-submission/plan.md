# Public Release And Registry Submission Plan

1. Validate the release gate inputs against the current workflows and metadata.
2. Run the crate, GH Release, GHCR, and MCP registry publication sequence on a
   tag that has already passed CI.
3. Record checksums, artifacts, registry responses, and any manual approvals
   needed for public submission.
4. Fold the observed steps back into the repo docs and release checklist.

## Completion Signal

The live release path can be executed end to end with documented evidence and
no hidden manual side channel.
