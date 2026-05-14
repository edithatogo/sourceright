# Plugin and Provider Roadmap — Status Dashboard

> Generated from `plugins/registry.toml` and `plugins/manifests/*.toml`
> Schema: `sourceright.plugin-registry.v1` | Runtime loading: `true`
> Last updated: 2026-05-14

## Status Taxonomy

| Status | Color | Meaning | Count |
|--------|-------|---------|-------|
| `core_normalizer` | 🟢 | Implemented as Rust built-in normalizer | 2 |
| `core_exporter` | 🟢 | Implemented as Rust built-in exporter | 1 |
| `fixture_tested` | 🟡 | Fixture evidence exists; partial implementation | 4 |
| `planned_public_api` | 🔵 | Metadata/manifest exists; free/public API | 5 |
| `planned_byo_key` | 🔵 | Metadata/manifest exists; requires user API key | 3 |
| `planned_adapter` | 🔵 | Metadata/manifest exists; adapter pattern | 3 |
| `planned` | 🔵 | Metadata/manifest exists; no special category | 2 |

**Total: 20 plugins** — 🟢 3 completed · 🟡 4 in_progress · 🔵 13 planned · ⏸️ 0 deferred

> **Note:** Manifest-level statuses are source of truth (4 plugins show `fixture_tested` in manifest vs registry's `planned`/`planned_adapter`).

## Full Plugin Dashboard

| # | ID | Category | Status | Owner Track | Fixtures | Docs Reference |
|---|---|---|---|---|---|---|
| 1 | `provider.crossref` | Provider | 🟢 `core_normalizer` | [Track 48 — Public API Provider Adapters](../48-public-api-provider-adapters/) | ✅ Example files | [`providers.md`](../../docs/src/providers.md) |
| 2 | `provider.dimensions` | Provider | 🔵 `planned_byo_key` | [Track 49 — Licensed BYO-Key Provider Adapters](../49-licensed-byo-key-provider-adapters/) | ❌ None | [`providers.md`](../../docs/src/providers.md) |
| 3 | `provider.scopus` | Provider | 🔵 `planned_byo_key` | [Track 49 — Licensed BYO-Key Provider Adapters](../49-licensed-byo-key-provider-adapters/) | ❌ None | [`providers.md`](../../docs/src/providers.md) |
| 4 | `provider.web-of-science` | Provider | 🔵 `planned_byo_key` | [Track 49 — Licensed BYO-Key Provider Adapters](../49-licensed-byo-key-provider-adapters/) | ❌ None | [`providers.md`](../../docs/src/providers.md) |
| 5 | `provider.unpaywall` | Provider | 🔵 `planned_public_api` | [Track 48 — Public API Provider Adapters](../48-public-api-provider-adapters/) | ✅ Example | [`providers.md`](../../docs/src/providers.md) |
| 6 | `provider.opencitations` | Provider | 🔵 `planned_public_api` | [Track 48 — Public API Provider Adapters](../48-public-api-provider-adapters/) | ✅ Example | [`providers.md`](../../docs/src/providers.md) |
| 7 | `provider.arxiv` | Provider | 🔵 `planned_public_api` | [Track 48 — Public API Provider Adapters](../48-public-api-provider-adapters/) | ✅ Example | [`providers.md`](../../docs/src/providers.md) |
| 8 | `provider.europepmc` | Provider | 🔵 `planned_public_api` | [Track 48 — Public API Provider Adapters](../48-public-api-provider-adapters/) | ✅ Example | [`providers.md`](../../docs/src/providers.md) |
| 9 | `provider.repository-records` | Provider | 🔵 `planned_public_api` | [Track 50 — Repository Record Provider Adapters](../50-repository-record-provider-adapters/) | ✅ Example | [`providers.md`](../../docs/src/providers.md) |
| 10 | `citation-manager.zotero` | Adapter | 🔵 `planned_adapter` | [Track 58 — Mature Zotero Plugin](../58-mature-zotero-plugin/) | ✅ 3 fixtures | [`citation-manager-integrations.md`](../../docs/src/citation-manager-integrations.md) |
| 11 | `citation-manager.endnote` | Adapter | 🔵 `planned_adapter` | [Track 59 — Other Citation Manager Integrations](../59-other-citation-manager-integrations/) | ❌ None | [`citation-manager-integrations.md`](../../docs/src/citation-manager-integrations.md) |
| 12 | `journal.ojs` | Adapter | 🔵 `planned_adapter` | [Track 60 — Mature OJS Plugin](../60-mature-ojs-plugin/) | ✅ OJS fixture | [`journal-integrations.md`](../../docs/src/journal-integrations.md) |
| 13 | `repository.pubmed` | Repository | 🟢 `core_normalizer` | [Track 48 — Public API Provider Adapters](../48-public-api-provider-adapters/) | ✅ Example | [`providers.md`](../../docs/src/providers.md) |
| 14 | `legal.courtlistener` | Legal | 🔵 `planned_public_api` | [Track 53 — CourtListener Legal Provider](../53-courtlistener-legal-provider/) | ✅ Example | [`legal-roadmap.md`](../../docs/src/legal-roadmap.md) |
| 15 | `matcher.local-bibliographic` | Matcher | 🟡 `fixture_tested` | [Track 52 — Non-Provider Pipeline Plugins](../52-non-provider-pipeline-plugins/) | ✅ Present | [`plugin-registry.md`](../../docs/src/plugin-registry.md) |
| 16 | `recency.retractions` | Recency | 🟡 `fixture_tested` | [Track 23 — Provider-Backed Recency Evidence](../23-provider-backed-recency-evidence/) | ✅ Present | [`style-and-recency.md`](../../docs/src/style-and-recency.md) |
| 17 | `relevance.claim-source` | Relevance | 🟡 `fixture_tested` | [Track 14 — Claim Source Provenance](../14-claim-source-provenance/) | ✅ Present | [`claim-provenance-roadmap.md`](../../docs/src/claim-provenance-roadmap.md) |
| 18 | `extraction.docx-pdf` | Extraction | 🟡 `fixture_tested` | [Track 36 — Document Extraction Hardening](../36-document-extraction-hardening/) | ✅ Present | [`workflow.md`](../../docs/src/workflow.md) |
| 19 | `export.citation-files` | Export | 🟢 `core_exporter` | [Track 11 — Export Suite](../11-export-suite/) | ✅ Export fixtures | [`exports.md`](../../docs/src/exports.md) |
| 20 | `demo.github-pages` | Demo | 🔵 `planned` | [Track 54 — Demo Public Surface Proof](../54-demo-public-surface-proof/) | ✅ Demo sample | [`plugin-registry.md`](../../docs/src/plugin-registry.md) |

## Coverage Summary

| Metric | Value |
|--------|-------|
| Total plugins | 20 |
| 🟢 Completed (core_normalizer + core_exporter) | 3 |
| 🟡 In progress (fixture_tested) | 4 |
| 🔵 Planned (all `planned_*` statuses) | 13 |
| ⏸️ Deferred | 0 |
| With manifest | 20 (100%) |
| With registry entry | 20 (100%) |
| With docs page | 19 (95%) |
| With fixture evidence | 8 (40%) |
| With fixture-backed automated CI test | 0 (0%) |
| With default-CI should-pass | 9 |
| With CI skip (BYO key / adapter) | 7 |
| Overclaim risk (planned != fixture_tested_or_implemented) | 13 |

## Owner Track Mapping

| Plugin | Owner Track |
|--------|------------|
| `provider.crossref` | Track 48 — Public API Provider Adapters |
| `provider.dimensions` | Track 49 — Licensed BYO-Key Provider Adapters |
| `provider.scopus` | Track 49 — Licensed BYO-Key Provider Adapters |
| `provider.web-of-science` | Track 49 — Licensed BYO-Key Provider Adapters |
| `provider.unpaywall` | Track 48 — Public API Provider Adapters |
| `provider.opencitations` | Track 48 — Public API Provider Adapters |
| `provider.arxiv` | Track 48 — Public API Provider Adapters |
| `provider.europepmc` | Track 48 — Public API Provider Adapters |
| `provider.repository-records` | Track 50 — Repository Record Provider Adapters |
| `citation-manager.zotero` | Track 58 — Mature Zotero Plugin |
| `citation-manager.endnote` | Track 59 — Other Citation Manager Integrations |
| `journal.ojs` | Track 60 — Mature OJS Plugin |
| `repository.pubmed` | Track 48 — Public API Provider Adapters |
| `legal.courtlistener` | Track 53 — CourtListener Legal Provider |
| `matcher.local-bibliographic` | Track 52 — Non-Provider Pipeline Plugins |
| `recency.retractions` | Track 23 — Provider-Backed Recency Evidence |
| `relevance.claim-source` | Track 14 — Claim Source Provenance |
| `extraction.docx-pdf` | Track 36 — Document Extraction Hardening |
| `export.citation-files` | Track 11 — Export Suite |
| `demo.github-pages` | Track 54 — Demo Public Surface Proof |

## Key Status Indicators

- 🟢 **Completed** — Core Rust implementation exists (`core_normalizer` / `core_exporter`)
- 🟡 **In Progress** — Fixture evidence exists; partial implementation (`fixture_tested`)
- 🔵 **Planned** — Metadata/manifest exists; implementation pending (`planned_*`)
- ⏸️ **Deferred** — Blocked by documented requirement; revisit trigger recorded

## Manifest vs Registry Status Discrepancies

The following plugins have differing status values between `plugins/registry.toml` and their manifest:

| Plugin | Registry Status | Manifest Status | Impact |
|--------|----------------|-----------------|--------|
| `matcher.local-bibliographic` | `planned` | `fixture_tested` | Manifest is more current (fixture evidence exists) |
| `recency.retractions` | `planned` | `fixture_tested` | Manifest is more current (fixture evidence exists) |
| `relevance.claim-source` | `planned` | `fixture_tested` | Manifest is more current (fixture evidence exists) |
| `extraction.docx-pdf` | `planned_adapter` | `fixture_tested` | Manifest is more current (fixture evidence exists) |

> **Recommendation:** Update `plugins/registry.toml` status for these 4 plugins to match their manifest values.

