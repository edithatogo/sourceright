---
title: Release runbook
description: Public release, registry submission, and evidence sequence.
---

1. Confirm the tag has passed CI.
2. Package and dry-run the crate publish.
3. Stage GitHub Release artifacts and checksums.
4. Validate MCP distribution metadata and image labels.
5. Submit to the registries that are part of the declared distribution surface.
