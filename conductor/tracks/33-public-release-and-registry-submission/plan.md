# Public Release And Registry Submission Plan

1. [x] Validate the release gate inputs against the current workflows and
   metadata.
2. [x] Run the crate, GitHub Release, GHCR-backed MCP image, and official MCP
   registry publication sequence on a tag that has already passed CI.
3. [x] Record checksums, artifacts, registry responses, and manual approval
   boundaries needed for public submission.
4. [x] Fold the observed steps back into the repo docs and release checklist.

## Completion Signal

The core live release path has documented public evidence for `v0.1.20`, and
remaining registry surfaces are explicitly classified as prepared or deferred
with no hidden manual side channel.
