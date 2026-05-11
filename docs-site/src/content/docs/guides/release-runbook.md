---
title: Release runbook
description: Public release, registry submission, and evidence sequence.
---

1. Confirm the tag has passed CI.
2. Package and dry-run the crate publish.
3. Stage GitHub Release artifacts and checksums.
4. Generate artifact attestations for the crate and binaries where supported.
5. Validate MCP distribution metadata and image labels.
6. Submit to the registries that are part of the declared distribution surface.
7. Capture the `release-status.md` artifact from each tag-triggered workflow.
