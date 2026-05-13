---
title: Providers
description: Provider normalization, evidence, and confidence policy.
---

Providers are evidence sources, not truth assertions.

- Normalize each provider into a comparable candidate shape.
- Preserve conflicts and confidence metadata in the sidecar.
- Do not let provider values silently overwrite canonical CSL.
- Use provider diagnostics to distinguish no-match, ambiguous, malformed, and
  outage-style fixture outcomes.
- Use the registry status matrix in [Plugin registry](../plugin-registry)
  when describing readiness. In practice, `core_normalizer` and
  `core_exporter` are the only statuses that should be described as technical
  preview; the `planned_*` and `planned` statuses remain roadmap-only.
