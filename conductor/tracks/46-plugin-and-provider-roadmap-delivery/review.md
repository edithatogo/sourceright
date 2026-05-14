# Plugin and Provider Roadmap Delivery — Review

## Current State

**Status:** Planned → In Progress
**Priority:** High
**Dependencies:** 19, 20, 21, 23, 37

## Evidence Found

### Registry

- `plugins/registry.toml` exists with schema version `sourceright.plugin-registry.v1`, runtime loading enabled
- 20 plugins registered with status taxonomy: `core_normalizer` (2), `core_exporter` (1), `planned_public_api` (6), `planned_byo_key` (3), `planned_adapter` (4), `planned` (4)
- 20 manifests in `plugins/manifests/` — one per plugin
- No plugin is currently `deferred` or `implemented` (beyond the two core normalizers)

### Documentation

| Docs page | Path | Covers |
|-----------|------|--------|
| Plugin registry | `docs/src/plugin-registry.md` | Full plugin table with status |
| Providers | `docs/src/providers.md` | Provider descriptions and configuration |
| Citation manager integrations | `docs/src/citation-manager-integrations.md` | Zotero/EndNote adapter docs |
| Journal integrations | `docs/src/journal-integrations.md` | OJS adapter docs |
| Plugin authoring | `docs/src/plugin-authoring.md` | Plugin development guide |
| Exports | `docs/src/exports.md` | Citation file export docs |
| Legal roadmap | `docs/src/legal-roadmap.md` | CourtListener / legal provider docs |
| Style and recency | `docs/src/style-and-recency.md` | Recency/retraction provider docs |
| Claim provenance roadmap | `docs/src/claim-provenance-roadmap.md` | Relevance provider docs |

### Policy Tests

- `tests/examples_policy.rs` — validates example configurations exist
- `tests/property_checks.rs` — general property assertions
- No test specifically validates plugin manifest schema, registry completeness, or status promotion rules

## Gaps

1. **No fixture-backed tests for any plugin** — even `core_normalizer` plugins lack fixture evidence
2. **No test validates registry.toml vs manifests** — no assertion that every manifest referenced in registry.toml exists (or vice versa)
3. **No status promotion rules enforced** — no test prevents a plugin from being marked `implemented` without fixtures and docs
4. **Docs site parity not verified for plugin docs** — `docs_site_parity.rs` covers general docs but no specific check for plugin-registry.md syncing
5. **Overclaim wording check absent** — no test verifies that `planned_*` plugins use "planned" not "supported" language in docs
6. **Default-CI behavior not explicitly documented per plugin** — the audit table shows expected CI behavior but this is not codified

## Completion Signal Assessment

The spec says every plugin must have an "owned implementation path, fixture path, or explicit deferral."

**Assessment:** All 20 plugins have manifests and most have docs pages. Two core normalizers and one exporter have Rust core implementations. However, **no plugin has fixture-backed tests**. The status taxonomy correctly labels planned plugins as planned rather than implemented.

**Readiness:** Foundation is in place (registry, manifests, docs), but substantial implementation work remains for fixture-backed tests and status promotion enforcement.
