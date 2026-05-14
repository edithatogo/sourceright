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

## Proof Documentation (NEW)

Two proof documents have been added to the track directory:

| Document | Purpose | Proof Family |
|----------|---------|-------------|
| `cli-smoke-proof.md` | Documents expected output and exit codes for all 7 required CLI commands. Includes example transcripts, JSON output samples, and a runnable shell script. | Installed CLI Smoke |
| `mcp-transcript-proof.md` | Documents MCP stdio endpoint discovery surfaces: tools, resources, prompts. Provides full JSON response examples and transcript template. | MCP Stdio Transcript Smoke |

Both documents cross-reference the existing `tests/cli_end_to_end.rs` CI coverage
for all proven surfaces.

## Gaps by Proof Family

| Proof Family | Evidence | Gap |
|-------------|----------|-----|
| **Installed CLI smoke** | `cli_end_to_end.rs` exists. `cli-smoke-proof.md` documents all 7 commands. | **Gap closed** — all 7 commands documented with expected output and exit codes. |
| **MCP stdio transcript smoke** | `mcp-transcript-proof.md` documents discovery surfaces. `mcp_distribution_checks.rs` validates server.json. | **Gap partially closed** — discovery surfaces are documented and CI-tested. Server startup remains opt-in only. |
| **OJS proof** | No dedicated OJS fixture or adapter test | Family is entirely missing |
| **Citation-manager proof** (Zotero/EndNote) | Zotero fixtures exist at `fixtures/providers/zotero/`. | Fixture-backed tests exist at fixture path level but no automated integration test exercises them. |
| **Live provider proof** | Provider manifests exist in `plugins/manifests/` | No opt-in live provider smoke with cache/rate-limit controls |
| **Registry proof** | MCP Registry is "accepted" in release-status.md. crates.io is accepted. | No automated listing/install checks for accepted registries |

## Completion Signal Assessment

The spec requires 6 proof families. Current status:

| Family | Status | Evidence |
|--------|--------|----------|
| Installed CLI Smoke | ✅ Proven (documented + CI) | `cli-smoke-proof.md`, `tests/cli_end_to_end.rs` |
| MCP Stdio Transcript Smoke | 🔶 Partial (discovery ✅, server startup 🔄) | `mcp-transcript-proof.md`, `tests/cli_end_to_end.rs` |
| OJS Proof | ❌ Missing | Nothing |
| Citation-Manager Proof | 🔶 Partial (fixtures exist, no automated test) | `fixtures/providers/zotero/` |
| Live Provider Proof | ❌ Missing | Nothing |
| Registry Proof | ❌ Missing | Nothing |

**Readiness:** Early stage. Two of six proof families now have documentation;
four remain entirely missing or require automated integration tests.
