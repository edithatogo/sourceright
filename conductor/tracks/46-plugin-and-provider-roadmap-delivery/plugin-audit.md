# Plugin and Provider Roadmap — Audit

## Status Taxonomy

| Status | Meaning |
|--------|---------|
| `core_normalizer` | Implemented in the Rust core as a built-in normalizer |
| `core_exporter` | Implemented in the Rust core as a built-in exporter |
| `fixture_tested` | Fixture evidence exists; partial implementation or adapter/export proof is present |
| `planned_public_api` | Metadata/manifest exists; implementation deferred; free/public API |
| `planned_byo_key` | Metadata/manifest exists; implementation deferred; requires user API key |
| `planned_adapter` | Metadata/manifest exists; implementation deferred; adapter pattern |
| `planned` | Metadata/manifest exists; implementation deferred; no special category |
| `deferred` | Blocked by documented requirement; revisit trigger recorded |

## Full Plugin Inventory

Source: `plugins/registry.toml` (schema: `sourceright.plugin-registry.v1`)
Runtime loading: `true`

| # | ID | Manifest | Status | Family | Fixtures | Docs Page | Default CI | Opt-in Live |
|---|-----|----------|--------|--------|----------|-----------|------------|-------------|
| 1 | `provider.crossref` | `plugins/manifests/provider.crossref.toml` | `core_normalizer` | Public API | Rust unit fixtures | providers.md | Pass | N/A |
| 2 | `provider.dimensions` | `plugins/manifests/provider.dimensions.toml` | `planned_byo_key` | Licensed BYO | Not found | providers.md | Skip | Requires key |
| 3 | `provider.scopus` | `plugins/manifests/provider.scopus.toml` | `planned_byo_key` | Licensed BYO | Not found | providers.md | Skip | Requires key |
| 4 | `provider.web-of-science` | `plugins/manifests/provider.web-of-science.toml` | `planned_byo_key` | Licensed BYO | Not found | providers.md | Skip | Requires key |
| 5 | `provider.unpaywall` | `plugins/manifests/provider.unpaywall.toml` | `planned_public_api` | Public API | Example fixture | providers.md | Should pass | N/A |
| 6 | `provider.opencitations` | `plugins/manifests/provider.opencitations.toml` | `planned_public_api` | Public API | Example fixture | providers.md | Should pass | N/A |
| 7 | `provider.arxiv` | `plugins/manifests/provider.arxiv.toml` | `planned_public_api` | Public API | Example fixture | providers.md | Should pass | N/A |
| 8 | `provider.europepmc` | `plugins/manifests/provider.europepmc.toml` | `planned_public_api` | Public API | Example fixture | providers.md | Should pass | N/A |
| 9 | `provider.repository-records` | `plugins/manifests/provider.repository-records.toml` | `planned_public_api` | Public API | Repository fixtures | providers.md | Should pass | N/A |
| 10 | `citation-manager.zotero` | `plugins/manifests/citation-manager.zotero.toml` | `fixture_tested` | Adapter | Zotero fixtures | citation-manager-integrations.md | Skip | Requires Zotero |
| 11 | `citation-manager.endnote` | `plugins/manifests/citation-manager.endnote.toml` | `fixture_tested` | Adapter | ENW/RIS fixtures | citation-manager-integrations.md | Skip | File handoff |
| 12 | `journal.ojs` | `plugins/manifests/journal.ojs.toml` | `planned_adapter` | Adapter | OJS fixture | journal-integrations.md | Skip | Requires OJS |
| 13 | `repository.pubmed` | `plugins/manifests/repository.pubmed.toml` | `core_normalizer` | Public API | Rust unit fixtures | providers.md | Pass | N/A |
| 14 | `legal.courtlistener` | `plugins/manifests/legal.courtlistener.toml` | `planned_public_api` | Public API | CourtListener fixtures | legal-roadmap.md | Should pass | Optional key |
| 15 | `matcher.local-bibliographic` | `plugins/manifests/matcher.local-bibliographic.toml` | `fixture_tested` | Local logic | Fixture evidence | providers.md | Should pass | N/A |
| 16 | `recency.retractions` | `plugins/manifests/recency.retractions.toml` | `fixture_tested` | Local logic | Fixture evidence | style-and-recency.md | Should pass | N/A |
| 17 | `relevance.claim-source` | `plugins/manifests/relevance.claim-source.toml` | `fixture_tested` | Local logic | Fixture evidence | claim-provenance-roadmap.md | Should pass | N/A |
| 18 | `extraction.docx-pdf` | `plugins/manifests/extraction.docx-pdf.toml` | `fixture_tested` | Adapter | Fixture evidence | docs/src/workflow.md | Skip | Requires external tool |
| 19 | `export.citation-files` | `plugins/manifests/export.citation-files.toml` | `core_exporter` | Exporter | Export fixtures | exports.md | Pass | N/A |
| 20 | `demo.github-pages` | `plugins/manifests/demo.github-pages.toml` | `planned` | Demo | See github_pages_demo/ | N/A | N/A (demo) | N/A |

## Coverage Summary

| Category | Total | `core_normalizer` | `core_exporter` | `fixture_tested` | `planned_public_api` | `planned_byo_key` | `planned_adapter` | `planned` |
|----------|-------|------------------:|----------------:|-----------------:|---------------------:|------------------:|------------------:|----------:|
| All plugins | 20 | 2 | 1 | 6 | 6 | 3 | 1 | 1 |
| With fixture evidence or examples | 17 | 2 | 1 | 6 | 5 | 0 | 1 | 1 |
| With docs page | 19 | 2 | 1 | 6 | 6 | 3 | 1 | 0 |
| With manifest | 20 | 2 | 1 | 6 | 6 | 3 | 1 | 1 |
| With registry/manifest validation tests | 20 | 2 | 1 | 6 | 6 | 3 | 1 | 1 |
| Overclaim risk | 11 | 0 | 0 | 0 | 6 | 3 | 1 | 1 |

### Key Observations

1. **Every plugin has a manifest** — the `plugins/manifests/` directory has 20 TOML files, one per plugin.
2. **Three plugins have core implementation** — `provider.crossref` and `repository.pubmed` are `core_normalizer`; `export.citation-files` is `core_exporter`.
3. **Six plugins are fixture-tested** — Zotero, EndNote, local matcher, recency, relevance, and extraction are promoted to `fixture_tested`; several planned provider and journal surfaces also have example fixtures without being claimed as implemented.
4. **Docs exist for most** — 19/20 have a docs page. `demo.github-pages` is the exception.
5. **Overclaim risk for `planned_public_api`** — 6 plugins are listed as `planned_public_api` but have no fixture evidence. Docs must clearly say "planned" not "supported."
6. **BYO-key plugins correctly skip CI** — 3 plugins are `planned_byo_key` and should never run in default CI.
