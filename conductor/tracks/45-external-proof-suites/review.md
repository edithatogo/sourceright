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

## Proof Documentation (ALL SIX FAMILIES NOW DOCUMENTED)

Six proof documents now exist in the track directory:

| Document | Purpose | Proof Family |
|----------|---------|-------------|
| `cli-smoke-proof.md` | Documents expected output and exit codes for all 7 required CLI commands. Includes example transcripts, JSON output samples, and a runnable shell script. | Installed CLI Smoke |
| `mcp-transcript-proof.md` | Documents MCP stdio endpoint discovery surfaces: tools, resources, prompts. Provides full JSON response examples and transcript template. | MCP Stdio Transcript Smoke |
| `ojs-proof.md` | Documents OJS journal screening pipeline using synthetic submission fixture. Covers fixture validation, workspace init, pipeline execution, and MCP surface verification. | OJS Proof |
| `citation-manager-proof.md` | Documents citation-sync preview/apply using Zotero fixtures. Covers exact match, title update, apply success, audit log, and MCP surface verification. | Citation-Manager Proof |
| `provider-proof.md` | Documents provider adapter discovery, manifest validation, fixture smoke tests, and CLI surface verification. | Live Provider Proof |
| `registry-proof.md` | Documents registry binding verification: server.json, glama.json, Dockerfile labels, release/publish workflows, and CI tests. | Registry Proof |

All six documents cross-reference existing CI tests where applicable.

## Gaps by Proof Family

| Proof Family | Evidence | Gap |
|-------------|----------|-----|
| **Installed CLI smoke** | `cli_end_to_end.rs` exists. `cli-smoke-proof.md` documents all 7 commands. | **Gap closed** — all 7 commands documented with expected output and exit codes. |
| **MCP stdio transcript smoke** | `mcp-transcript-proof.md` documents discovery surfaces. `mcp_distribution_checks.rs` validates server.json. | **Gap partially closed** — discovery surfaces are documented and CI-tested. Server startup remains opt-in only. |
| **OJS proof** | `ojs-proof.md` documents OJS screening pipeline with synthetic fixture. | **Gap partially closed** — pipeline is documented but not automated in CI. |
| **Citation-manager proof** | `citation-manager-proof.md` documents three Zotero fixture scenarios. | **Gap partially closed** — documented but not automated in CI. |
| **Live provider proof** | `provider-proof.md` documents registry discovery, manifest validation, fixture smoke. | **Gap partially closed** — static fixture validation works; live provider API calls remain planned. |
| **Registry proof** | `registry-proof.md` documents server.json, glama.json, Dockerfile, workflows. | **Gap closed** — all registry bindings are CI-validated via `mcp_distribution_checks.rs`. |


## Completion Signal Assessment

The spec requires 6 proof families. Current status:

| Family | Status | Evidence |
|--------|--------|----------|
| Installed CLI Smoke | ✅ Proven (documented + CI) | `cli-smoke-proof.md`, `tests/cli_end_to_end.rs` |
| MCP Stdio Transcript Smoke | 🔶 Partial (discovery ✅, server startup 🔄) | `mcp-transcript-proof.md`, `tests/cli_end_to_end.rs` |
| OJS Proof | 🔶 Partial (documented, not CI-automated) | `ojs-proof.md`, `fixtures/journal/ojs-submission.json` |
| Citation-Manager Proof | 🔶 Partial (documented, not CI-automated) | `citation-manager-proof.md`, `fixtures/providers/zotero/` |
| Live Provider Proof | 🔶 Partial (static discovery ✅, live API 🔄) | `provider-proof.md`, `plugins/registry.toml` |
| Registry Proof | ✅ Proven (CI-validated) | `registry-proof.md`, `mcp_distribution_checks.rs` |

## ⛔ Track Completion Assessment (2026-06-24)

**Verdict: Do NOT promote to completed.**

The track requires 6 proof families with documented evidence matching the
test-matrix acceptance criteria. While all 6 proof documents exist and 5 of 6
acceptance criteria are met, one critical gap remains:

| Proof Family | Acceptance | Verdict |
|-------------|-----------|---------|
| Installed CLI | Binary install/run smoke produces expected JSON or help output. | ✅ Met (`cli-smoke-proof.md`) |
| MCP stdio | Transcript fixture proves initialize/list/read paths. | ✅ Met (`mcp-transcript-proof.md`) |
| OJS | Fixture adapter produces editor and author screening outputs; live smoke is opt-in. | ✅ Met (`ojs-proof.md`) |
| Zotero/EndNote | Preview/apply/audit semantics are fixture-backed; live library smoke is opt-in. | ✅ Met (`citation-manager-proof.md`) |
| **Live providers** | **Provider smoke respects timeout, retry, min-interval, and cache controls.** | ❌ **Not met** — `provider-proof.md` covers static discovery, manifest validation, and fixture smoke, but does **not** document timeout, retry, min-interval, or cache-control behavior for provider smoke. |
| Registries | Accepted listings have install/listing checks; prepared surfaces do not overclaim. | ✅ Met (`registry-proof.md`) |

**Required action to close:** Update `provider-proof.md` to document how provider
smoke verifies timeout, retry, min-interval, and cache-control semantics — or
update the test-matrix acceptance if these controls are deferred to a downstream
implementation track.

**Readiness:** Five of six proof families are documented and acceptance-matched.
One gap remains. Next step: add timeout/retry/min-interval/cache-control
documentation to the Live Provider proof, then reassess for completion.
