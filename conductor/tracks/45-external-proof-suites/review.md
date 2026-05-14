# External Proof Suites — Review

## Current State

**Status:** Planned → In Progress
**Priority:** High
**Dependencies:** 16, 17, 20, 21, 27, 36, 37, 38, 39, 40

## Evidence Found

### Existing Proof-Related Tests

| Test | Path | What it proves |
|------|------|----------------|
| demo_policy | `tests/demo_policy.rs` | Demo samples are schema-versioned, demos use sample data only (no live services), static demo render smoke (Node), streamlit model smoke (Python), opt-in browser/server smoke scripts exist with env-var gating |
| mcp_distribution_checks | `tests/mcp_distribution_checks.rs` | server.json version matches Cargo.toml, OCI distribution target, Dockerfile labels, release workflow labels, glama.json validity, docs/README references distribution artifacts |
| cli_end_to_end | `tests/cli_end_to_end.rs` | CLI smoke tests for binary-invoked commands |

### Demos

| Artifact | Path | Notes |
|----------|------|-------|
| Static demo | `github_pages_demo/` | Contains `sample/` fixtures, `app.js`, `index.html`, `render-smoke.mjs`, `browser-smoke.mjs` |
| Streamlit demo | `streamlit_app/` | Contains `sample_workspace/`, `app.py`, `test_demo_model.py`, `server_smoke.py` |

### Fixtures

- `github_pages_demo/sample/` — `reference-report.json`, `journal-screening.json`
- `streamlit_app/sample_workspace/` — same files, verified identical by `demo_policy.rs`

### Environment Variable Gating (Opt-in Smokes)

- `SOURCERIGHT_DEMO_BROWSER_SMOKE` — gates Playwright browser smoke (static demo)
- `SOURCERIGHT_DEMO_SERVER_SMOKE` — gates Streamlit server smoke

### Evidence Ledger

No `evidence-ledger.json` file exists in the repository. Track 45 does not have a pre-existing ledger entry.

## Gaps by Proof Family

| Proof Family | Evidence | Gap |
|-------------|----------|-----|
| **Installed CLI smoke** | `cli_end_to_end.rs` exists. `demo_policy.rs` tests binary-invoked commands. | No dedicated "installed binary smoke" test for all 7 required commands (`init`, `validate-csl`, `report`, `export`, `bench`, `citation-sync`, `mcp status`) |
| **MCP stdio transcript smoke** | `mcp_distribution_checks.rs` validates server.json metadata and OCI targets. | No transcript fixture proving `initialize`, `list`, `read` paths for tools/resources/prompts |
| **OJS proof** | No dedicated OJS fixture or adapter test | Family is entirely missing |
| **Citation-manager proof** (Zotero/EndNote) | `citation-manager-integrations.md` exists in docs | No fixture-backed preview/apply/audit test. No EndNote export handoff proof |
| **Live provider proof** | Provider manifests exist in `plugins/manifests/` | No opt-in live provider smoke with cache/rate-limit controls |
| **Registry proof** | MCP Registry is "accepted" in release-status.md. crates.io is accepted. | No automated listing/install checks for accepted registries |

## Completion Signal Assessment

The spec requires 6 proof families. Currently only the Installed CLI family (partial) and some demo/registry infrastructure have evidence. OJS, Citation-manager, Live provider, and Registry proof families need dedicated implementation.

**Readiness:** Early stage. Most proof families need test fixtures, opt-in smoke scripts, and documentation alignment.
