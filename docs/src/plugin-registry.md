# Plugin Registry

The registry in `plugins/registry.toml` is now a runtime discovery surface.
Sourceright validates the listed manifests at load time and exposes the
validated catalog through `sourceright plugins` and the MCP discovery surface.

Registry entries point to manifests under `plugins/manifests/`. Each manifest
declares a plugin category, capability summary, authentication expectations,
cache and licence policy, and the Sourceright contracts it reads or writes.

## Categories

The current registry covers the planned plugin families:

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
