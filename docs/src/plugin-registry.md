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
| `fixture_tested` | Technical preview | Fixture-backed adapter or workflow evidence exists, but host/live execution remains gated. |
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

Full plugin authoring guidance — including packaging policy, evidence-ledger
requirements, provenance, sandbox rules, status taxonomy, and overclaim guards —
is maintained in [Plugin Authoring](plugin-authoring.md).

## Runtime Status

Runtime plugin discovery is implemented. Execution remains gated by explicit
trust policy and future capability enablement. Before Sourceright executes
plugins at runtime, it still needs:

- signed or pinned plugin provenance;
- clear sandbox, network, and cache policies;
- deterministic dry-run modes;
- audit logs for every provider or exporter action;
- stable read-only MCP contracts for plugin discovery and reports.

## Expanded Provider Catalogue

A comprehensive catalogue of all normaliser provider candidates (including
access model, status, owning track, evidence level, fixture coverage, default-CI
behaviour, and overclaim guard) is maintained in [Provider Strategy](providers#expanded-provider-catalogue).
That catalogue covers:

- **Core/public**: Crossref, DataCite, OpenAlex, PubMed/NCBI, ORCID,
  Unpaywall, OpenCitations, arXiv, Europe PMC.
- **Licensed BYO-key**: Dimensions, Scopus, Web of Science.
- **Economics (deferred)**: RePEc, SSRN, NBER, EconLit, IDEAS — with
  documented decision rationale.
- **Grey literature/repositories**: Zenodo, OSF, Figshare, Dataverse,
  institutional repositories (deferred).
- **Search**: Google Scholar — documented as assessment-only / prohibited
  per ADR 0005.
- **Biomedical**: bioRxiv/medRxiv, clinical trial registries (deferred).

Each row in the catalogue includes an overclaim guard that prevents
unsupported market-readiness claims. Providers with `deferred` status
have no manifest, no fixtures, and no CI path.

## Packaging Policy

Plugins should not be split into git submodules by default. Keep plugin
manifests and immature adapters in this repository while the plugin API is
changing. Split a plugin into a separate repository or package only when it has
an independent release lifecycle, separate maintainers, host-specific packaging
requirements, or a stable compatibility contract. Track 63 owns the supply-chain
criteria for that decision.
