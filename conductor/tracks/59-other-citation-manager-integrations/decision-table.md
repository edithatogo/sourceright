# Track 59 — Other Citation Manager Integrations: Decision Table

## Purpose

Document support status, approach, and rationale for citation managers beyond Zotero.
The spec requires: each manager gets either an adapter plan, export/import proof, or explicit deferral.
No manager is called "supported" without fixture-backed evidence.

## Decision Table

| Manager | Adapter Family | API Available | Interchange Formats | Recommended Approach | Manifest Exists? | Fixtures? | Decision | Rationale |
|---------|---------------|--------------|--------------------|---------------------|-----------------|-----------|----------|-----------|
| **Zotero** | `api_and_file` | Zotero Web API v3 (public, byo-key) | RIS, BibLaTeX, CSL-JSON | **Sync adapter** (Web API) with dry-run-first contract | ✅ `citation-manager.zotero.toml` | ❌ | **Implementing** (Track 51, 58) | Primary target; preview/apply/audit engine exists, pending fixtures |
| **EndNote** | `file` | No public live-sync API (local file only) | ENW, RIS, XML | **File handoff** via ENW/RIS export | ✅ `citation-manager.endnote.toml` | ❌ | **Implementing** (Track 51) | ENW/RIS generators exist in `export.rs`; pending fixture-backed reparse tests |
| **Mendeley** | `api_and_file` | Mendeley API (public, OAuth 2.0, byo-token) | RIS, BibLaTeX | **Deferred** — API exists but priority lower than Zotero | ❌ | ❌ | **Deferred** | Mendeley profile YAML exists at `examples/citation-manager-profiles/mendeley.yaml` but no manifest, no engine code, no fixtures. Requires OAuth 2.0 token management and API adapter. |
| **Paperpile** | `file` | Paperpile CSV/BibTeX export, no public API | BibTeX, CSV | **Deferred** — file-based only; API unclear | ❌ | ❌ | **Deferred** | No profile YAML exists. Paperpile is browser-based with limited export surface. Recommend RIS/BibTeX file handoff if demand emerges. |
| **JabRef** | `file` | BibTeX/BibLaTeX file-based, no sync API | BibLaTeX, RIS | **Deferred** — BibLaTeX export already exists | ❌ | ❌ | **Deferred** | Profile YAML exists at `examples/citation-manager-profiles/jabref.yaml`. The existing BibLaTeX export in `export.rs` covers JabRef's primary format. No additional adapter needed. |
| **RefWorks** | `file` | RefWorks legacy export (RIS), new RefWorks has API | RIS | **Deferred** — legacy RIS handoff sufficient | ❌ | ❌ | **Deferred** | No profile YAML exists. RIS export in `export.rs` covers legacy RefWorks. New RefWorks API requires institutional credentials. |
| **Citavi** | `file` | Citavi import/export (RIS, Citavi XML), no public API | RIS, Citavi XML | **Deferred** — RIS handoff sufficient | ❌ | ❌ | **Deferred** | No profile YAML exists. RIS export covers the primary interchange format. Citavi XML export would require separate generator. |
| **Papers/ReadCube** | `file` | ReadCube API exists but limited | RIS, BibLaTeX | **Deferred** | ❌ | ❌ | **Deferred** | Profile YAML exists at `examples/citation-manager-profiles/papers-readcube.yaml`. RIS/BibLaTeX export covers handoff. |

## Extended Managers (Systematic Review Tools)

These are listed in `docs/src/citation-manager-integrations.md` and have profile YAMLs:

| Manager | Profile YAML | Adapter Family | Approach |
|---------|-------------|---------------|----------|
| **RevMan** (Cochrane) | `examples/citation-manager-profiles/revman.yaml` | `file` | RIS file handoff — `export_ris()` covers this |
| **Rayyan** | `examples/citation-manager-profiles/rayyan.yaml` | `file` | RIS file handoff — `export_ris()` covers this |
| **Covidence** | `examples/citation-manager-profiles/covidence.yaml` | `file` | RIS file handoff — `export_ris()` covers this |

All three are **deferred** with RIS file handoff as the supported interchange. No adapter manifests, no engine code, no fixtures.

## Implementation Priority

| Priority | Manager | Reason |
|----------|---------|-------|
| P0 | Zotero | Primary integration target; engine exists, needs fixtures |
| P1 | EndNote | ENW/RIS export exists; needs reparse fixtures |
| P2 | Mendeley | Public API + existing profile; OAuth 2.0 complexity |
| P3 | JabRef | BibLaTeX export exists; minimal additional work |
| P4 | Paperpile, RefWorks, Citavi, Papers/ReadCube | File handoff only; low priority |
| P5 | RevMan, Rayyan, Covidence | Systematic review tools; RIS handoff sufficient |

## Evidence Summary

| Evidence Type | Zotero | EndNote | Mendeley | Others |
|--------------|--------|---------|----------|--------|
| Plugin manifest | ✅ | ✅ | ❌ | ❌ |
| Registry entry | ✅ | ✅ | ❌ | ❌ |
| Profile YAML | ✅ | ✅ | ✅ | ✅ (JabRef, Papers) |
| Rust engine | ✅ `citation_sync.rs` | ✅ `export.rs` (ENW/RIS) | ❌ | ❌ |
| Unit tests | ✅ 8 tests | ✅ 3 tests | ❌ | ❌ |
| Schema tests | ✅ 3 tests | ❌ | ❌ | ❌ |
| JSON Schema | ✅ 2 schemas | ✅ 1 schema | ❌ | ❌ |
| Fixtures | ❌ | ❌ | ❌ | ❌ |

## Claim Boundary Enforcement

Per spec: "No manager is called supported without fixture-backed import/export or live smoke evidence."

- **Zotero** and **EndNote** are the only managers with any code-based evidence. Both lack fixture-backed tests.
- All other managers are **deferred** — no code, no tests, no fixtures exist.
- Documentation must clearly state that only Zotero and EndNote have partial implementation, and all others are planned/deferred with RIS/BibLaTeX file handoff as the expected integration path.
- The phrase "supported" must not appear for any manager without fixture-backed evidence.
