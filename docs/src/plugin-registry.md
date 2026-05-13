# Plugin Registry

The registry in `plugins/registry.toml` is now a runtime discovery surface.
Sourceright validates the listed manifests at load time and exposes the
validated catalog through `sourceright plugins` and the MCP discovery surface.

Registry entries point to manifests under `plugins/manifests/`. Each manifest
declares a plugin category, capability summary, authentication expectations,
cache and licence policy, and the Sourceright contracts it reads or writes.

## Status Matrix

Registry `status` values are implementation labels, not release promises.
Use the matrix below when translating them into market-readiness wording:

| Registry status | Market-readiness label | Meaning |
| --- | --- | --- |
| `core_normalizer` | Technical preview | Implemented core behavior with fixture-backed and trust-gated use. |
| `core_exporter` | Technical preview | Implemented exporter behavior with the same controlled-use limits. |
| `planned_public_api` | Roadmap, not preview | Public API target is described, but implementation is still pending. |
| `planned_byo_key` | Roadmap, not preview | BYO-key or licensed-data target is described, but implementation is still pending. |
| `planned_adapter` | Roadmap, not preview | Adapter target is described, but implementation is still pending. |
| `planned` | Roadmap, not preview | Concept is catalogued, but no implementation-ready surface exists yet. |

Technical preview in Sourceright means the contract, fixtures, and validation
path are in place, but runtime execution still follows explicit trust,
dry-run, and provenance limits.

## Categories

The current registry covers the current plugin families:

- `provider`: scholarly and legal metadata providers.
- `citation-manager`: file or API sync targets such as Zotero and EndNote.
- `journal`: editorial workflow integrations such as OJS.
- `repository`: repository and preprint archive integrations.
- `legal`: legal citation lookup and enrichment providers.
- `matcher`: local matching/ranking adapters.
- `recency`: checks for retractions, corrections, and stale guidelines.
- `relevance`: source relevance and claim/source review adapters.
- `extraction`: document text and reference extraction adapters.
- `export`: downstream reference-file and sync exporters.
- `demo`: local or hosted demonstrators.

Paid or closed providers, including Scopus, Web of Science, and Dimensions, are
marked as BYO-key or licensed-data plugins. Their manifests describe expected
contracts without requiring credentials or live network access in normal tests.

## Runtime Status

Runtime plugin discovery is implemented. Execution remains gated by explicit
trust policy and future capability enablement. Before Sourceright executes
plugins at runtime, it still needs:

- manifest validation wired into CI;
- signed or pinned plugin provenance;
- clear sandbox, network, and cache policies;
- deterministic dry-run modes;
- audit logs for every provider or exporter action;
- stable read-only MCP contracts for plugin discovery and reports.
