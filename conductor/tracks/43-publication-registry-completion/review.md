# Track 43 — Publication Registry Completion — Review

**Status:** completed
**Date:** 2026-05-13
**Reviewer:** codex

## Track Objective

Turn publication registry status into a repeatable, documented process covering all registry surfaces and package-manager decisions.

## Test Matrix Verification

| # | Test | Result | Evidence |
|---|------|--------|----------|
| 1 | Accepted registry — URL, version, date, install metadata recorded | ✅ Pass | docs/src/release-status.md — Accepted Registries table. 4 registries documented with all columns. |
| 2 | Prepared registry — metadata exists but docs say not accepted yet | ✅ Pass | docs/src/release-status.md — Prepared Registries table. GHCR, Glama, Smithery, MCP client configs, GitHub Copilot prep, Zotero, and OJS/PKP are documented as prepared rather than accepted. |
| 3 | Deferred registry — blocking requirement and revisit trigger documented | ✅ Pass | docs/src/release-status.md — Deferred Registries table. Package-manager and host-package surfaces record missing artifacts, blocking requirements, and revisit triggers. |
| 4 | GHCR evidence — captured or labelled as MCP Registry indirection | ✅ Pass | GHCR listed as "prepared" with note: "Documented as MCP Registry indirection — the OCI image is published but the GHCR package listing is indirectly evidenced." |
| 5 | Package-manager decision — each has yes/no/defer | ✅ Pass | Package-Manager Feasibility Decisions table. Homebrew (Defer), Scoop (Defer), Chocolatey (No), winget (Defer), npm (Defer), PyPI (Defer). All with rationale. |
| 6 | Review loop — $conductor-review | ✅ Pass | This review.md file created as the track review artifact. |

## Files Modified

| File | Change |
|------|--------|
| `docs/src/release-status.md` | Expanded from simple table to comprehensive Registry Completion Table with 4 status categories, 6 package-manager decisions, and evidence summary. |
| `docs/src/publishing.md` | Added cross-reference to release-status.md for the full registry table. |
| `docs-site/src/content/docs/release-status.md` | Mirror updated to match docs/src version. |
| `docs-site/src/content/docs/guides/publishing.md` | Mirror updated to cross-reference release-status. |
| `conductor/tracks/43-publication-registry-completion/metadata.json` | Status changed from "planned" to "completed". |
| `conductor/evidence-ledger.json` | Track 43 evidence_level upgraded from "contracted" to "fixture-backed" with expanded allowed_claims and blockers. |

## Files Unchanged

- `.github/` — not touched per task constraints
- `src/` — not touched per task constraints
- `plugins/` — not touched per task constraints
- `conductor/requirements.md` — not touched per task constraints

## Evidence Summary

The registry completion table now reconciles registry, package-manager, host
packaging, citation-manager, and journal-plugin surfaces:

- **4 accepted** (GitHub Release, crates.io, docs.rs, Official MCP Registry) — each with version, URL, date, and install metadata.
- **Prepared** (GHCR MCP image, Glama, Smithery, Claude Desktop client config, Codex MCP client config, generic MCP client config, GitHub Copilot coding-agent prep, Zotero, OJS/PKP) — repository metadata, package/config source, and acceptance blockers are recorded without claiming public listing acceptance.
- **Deferred** (Homebrew, Scoop, winget, npm launcher, PyPI launcher, VS Code Marketplace/Open VSX, Microsoft AppSource/Word add-in, LibreOffice Extensions) — each records the missing artifact, blocking requirement, and revisit trigger.
- **1 n/a** (Chocolatey) — rationale documented.

Package-manager feasibility decisions cover 6 managers with clear defer/no rationale.

All docs-site mirrors are in parity with docs/src archival source.

This review was reconciled after later marketplace and host-packaging tracks
expanded prepared surfaces beyond the original GHCR/Glama-only scope.
