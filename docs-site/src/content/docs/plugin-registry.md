---
title: Plugin registry
description: Plugin registry layout and discovery policy.
---

The plugin registry declares the installable capabilities Sourceright can load.

## Status Matrix

Registry `status` values are implementation labels, not release promises.
Use the matrix below when translating them into market-readiness wording:

| Registry status | Market-readiness label | Meaning |
| --- | --- | --- |
| `core_normalizer` | Technical preview | Implemented core behavior with fixture-backed and trust-gated use. |
| `core_exporter` | Technical preview | Implemented exporter behavior with the same controlled-use limits. |
| `fixture_tested` | Technical preview | Fixture-backed adapter or workflow evidence exists, but host/live execution remains gated. |
| `planned_public_api` | Roadmap, not preview | Public API target is described, but implementation is still pending. |
| `planned_byo_key` | Roadmap, not preview | BYO-key or licensed-data target is described, but implementation is still pending. |
| `planned_adapter` | Roadmap, not preview | Adapter target is described, but implementation is still pending. |
| `planned` | Roadmap, not preview | Concept is catalogued, but no implementation-ready surface exists yet. |

Technical preview in Sourceright means the contract, fixtures, and validation
path are in place, but runtime execution still follows explicit trust,
dry-run, and provenance limits.

- Registry entries are validated before use.
- Capabilities remain explicit and provenance-aware.

## Packaging Policy

Plugins should not be split into git submodules by default. Keep plugin
manifests and immature adapters in this repository while the plugin API is
changing. Split a plugin into a separate repository or package only when it has
an independent release lifecycle, separate maintainers, host-specific packaging
requirements, or a stable compatibility contract. Track 63 owns the supply-chain
criteria for that decision.
