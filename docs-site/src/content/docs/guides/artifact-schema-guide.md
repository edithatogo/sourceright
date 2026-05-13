---
title: Artifact and Schema Guide
description: Main Sourceright artifacts and schema boundaries.
---

The key boundary is simple: `references.csl.json` is canonical clean
bibliographic data, while `references.verification.json` stores evidence,
confidence, conflicts, provenance, and review state.

Derived outputs such as review queues, reports, export manifests, policy
reports, and MCP resources should be validated against schemas before being
handed to downstream tools.

Citation-manager sync has two related surfaces: `sourceright.sync_manifest.v1`
for planned sync manifests and `sourceright.citation_sync.v1` for concrete
preview/apply reports with suggestion classes and audit paths.
