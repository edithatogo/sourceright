# Plugin and Provider Roadmap — Audit

## Status Taxonomy

| Status | Meaning |
|--------|---------|
| `core_normalizer` | Implemented in the Rust core as a built-in normalizer |
| `core_exporter` | Implemented in the Rust core as a built-in exporter |
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
| 1 | `provider.crossref` | `plugins/manifests/provider.crossref.toml` | `core_normalizer` | Public API | Not found | providers.md | Pass | N/A |
| 2 | `provider.dimensions` | `plugins/manifests/provider.dimensions.toml` | `planned_byo_key` | Licensed BYO | Not found | providers.md | Skip | Requires key |
| 3 | `provider.scopus` | `plugins/manifests/provider.scopus.toml` | `planned_byo_key` | Licensed BYO | Not found | providers.md | Skip | Requires key |
| 4 | `provider.web-of-science` | `plugins/manifests/provider.web-of-science.toml` | `planned_byo_key` | Licensed BYO | Not found | providers.md | Skip | Requires key |
| 5 | `provider.unpaywall` | `plugins/manifests/provider.unpaywall.toml` | `planned_public_api` | Public API | Not found | providers.md | Should pass | N/A |
| 6 | `provider.opencitations` | `plugins/manifests/provider.opencitations.toml` | `planned_public_api` | Public API | Not found | providers.md | Should pass | N/A |
| 7 | `provider.arxiv` | `plugins/manifests/provider.arxiv.toml` | `planned_public_api` | Public API | Not found | providers.md | Should pass | N/A |
| 8 | `provider.europepmc` | `plugins/manifests/provider.europepmc.toml` | `planned_public_api` | Public API | Not found | providers.md | Should pass | N/A |
| 9 | `provider.repository-records` | `plugins/manifests/provider.repository-records.toml` | `planned_public_api` | Public API | Not found | providers.md | Should pass | N/A |
| 10 | `citation-manager.zotero` | `plugins/manifests/citation-manager.zotero.toml` | `planned_adapter` | Adapter | Not found | citation-manager-integrations.md | Skip | Requires Zotero |
| 11 | `citation-manager.endnote` | `plugins/manifests/citation-manager.endnote.toml` | `planned_adapter` | Adapter | Not found | citation-manager-integrations.md | Skip | Requires EndNote |
| 12 | `journal.ojs` | `plugins/manifests/journal.ojs.toml` | `planned_adapter` | Adapter | Not found | journal-integrations.md | Skip | Requires OJS |
| 13 | `repository.pubmed` | `plugins/manifests/repository.pubmed.toml` | `core_normalizer` | Public API | Not found | providers.md | Pass | N/A |
| 14 | `legal.courtlistener` | `plugins/manifests/legal.courtlistener.toml` | `planned_public_api` | Public API | Not found | legal-roadmap.md | Should pass | N/A |
| 15 | `matcher.local-bibliographic` | `plugins/manifests/matcher.local-bibliographic.toml` | `planned` | Local logic | Not found | providers.md | Should pass | N/A |
| 16 | `recency.retractions` | `plugins/manifests/recency.retractions.toml` | `planned` | Local logic | Not found | style-and-recency.md | Should pass | N/A |
| 17 | `relevance.claim-source` | `plugins/manifests/relevance.claim-source.toml` | `planned` | Local logic | Not found | claim-provenance-roadmap.md | Should pass | N/A |
| 18 | `extraction.docx-pdf` | `plugins/manifests/extraction.docx-pdf.toml` | `planned_adapter` | Adapter | Not found | docs/src/ (none specific) | Skip | Requires external tool |
| 19 | `export.citation-files` | `plugins/manifests/export.citation-files.toml` | `core_exporter` | Exporter | Not found | exports.md | Pass | N/A |
| 20 | `demo.github-pages` | `plugins/manifests/demo.github-pages.toml` | `planned` | Demo | See github_pages_demo/ | N/A | N/A (demo) | N/A |

## Coverage Summary

| Category | Total | `core_normalizer` | `core_exporter` | `planned_public_api` | `planned_byo_key` | `planned_adapter` | `planned` |
|----------|-------|------------------:|----------------:|---------------------:|------------------:|------------------:|----------:|
| All plugins | 20 | 2 | 1 | 6 | 3 | 4 | 4 |
| With fixture evidence | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| With docs page | 19 | 2 | 1 | 6 | 3 | 4 | 3 |
| With manifest | 20 | 2 | 1 | 6 | 3 | 4 | 4 |
| With default-CI test | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Overclaim risk | 14 | 0 | 0 | 6 | 3 | 4 | 1 |

### Key Observations

1. **Every plugin has a manifest** — the `plugins/manifests/` directory has 20 TOML files, one per plugin.
2. **Only 2 plugins have core implementation** — `provider.crossref` and `repository.pubmed` (both `core_normalizer`). `export.citation-files` is `core_exporter`.
3. **No plugins have fixture-backed tests** — none of the `tests/*.rs` files reference plugin IDs directly or test plugin behavior against fixtures.
4. **Docs exist for most** — 19/20 have a docs page. `demo.github-pages` is the exception.
5. **Overclaim risk for `planned_public_api`** — 6 plugins are listed as `planned_public_api` but have no fixture evidence. Docs must clearly say "planned" not "supported."
6. **BYO-key plugins correctly skip CI** — 3 plugins are `planned_byo_key` and should never run in default CI.
