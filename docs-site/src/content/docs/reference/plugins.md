---
title: Plugins
description: Runtime plugin loading and registry policy.
---

Plugins are discovered from manifests and validated before use.

- Registry entries must be trusted and provenance-aware.
- Capability discovery should remain explicit and auditable.
- Runtime loading must never bypass the repository's dry-run and review rules.
- Use the registry status matrix in [Plugin registry](../plugin-registry) when
  turning implementation statuses into public-facing readiness language.
