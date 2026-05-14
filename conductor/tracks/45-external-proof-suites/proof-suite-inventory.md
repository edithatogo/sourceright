# External Proof Suites — Inventory

## Environment Variable Contract

All opt-in live smoke tests MUST respect the following contract:

| Variable | Value | Effect |
|----------|-------|--------|
| `SOURCERIGHT_SKIP_LIVE` | `1`, `true`, `TRUE` | Skip all live-service tests; emit diagnostic skip message |
| `SOURCERIGHT_DEMO_BROWSER_SMOKE` | any non-empty | Enable Playwright browser smoke for static demo |
| `SOURCERIGHT_DEMO_SERVER_SMOKE` | any non-empty | Enable Streamlit server smoke |
| `SOURCERIGHT_PROVIDER_SMOKE` | any non-empty | Enable live provider smoke tests |
| `SOURCERIGHT_MCP_TRANSCRIPT` | any non-empty | Enable MCP transcript smoke test (requires real MCP client) |
| `CI` | `1`, `true`, `TRUE` | Auto-detected — skip all live smokes in CI; required commands must exist or tests panic |
| `GITHUB_ACTIONS` | `1`, `true`, `TRUE` | Same as CI detection |

In CI, all live smokes MUST be skipped. Tests that require a command MUST panic if the command is missing. Outside CI, missing commands produce a diagnostic skip message instead of a failure.

## Proof Families

### 1. Installed CLI Smoke

**Status:** PARTIAL
**Test file:** `tests/cli_end_to_end.rs` (exists), `tests/demo_policy.rs` (partial)
**Commands required:** `init`, `validate-csl`, `report`, `export`, `bench`, `citation-sync`, `mcp status`
**Currently tested:** Binary invocation, some subcommand coverage
**Fixture path:** `github_pages_demo/sample/`, `streamlit_app/sample_workspace/`
**Opt-in live:** N/A
**Default CI:** SHOULD pass
**Overclaim wording:** Must NOT claim all 7 commands proven until each has a dedicated fixture-backed assertion.

### 2. MCP Stdio Transcript Smoke

**Status:** MISSING
**Test file:** None
**Required:** Real client-style transcript proving tool, resource, and prompt discovery via stdio
**Fixture path:** `tests/fixtures/mcp-transcript/` (not yet created)
**Opt-in live:** Via `SOURCERIGHT_MCP_TRANSCRIPT`
**Default CI:** Must skip
**Overclaim wording:** Must NOT claim MCP transcript smoke passing without fixture or transcript evidence.

### 3. OJS Proof

**Status:** MISSING
**Test file:** None
**Required:** Fixture-backed adapter contract producing editor and author screening outputs
**Fixture path:** `tests/fixtures/ojs/` (not yet created)
**Opt-in live:** Optional disposable OJS/test instance smoke
**Default CI:** Must skip
**Overclaim wording:** Must NOT claim OJS integration without fixture-backed adapter evidence.

### 4. Citation-Manager Proof (Zotero/EndNote)

**Status:** MISSING
**Test file:** None
**Required:** Preview/apply/audit semantics fixture-backed for Zotero; EndNote export handoff proof
**Fixture path:** `tests/fixtures/citation-manager/` (not yet created)
**Opt-in live:** Optional disposable-library smoke (Zotero API)
**Default CI:** Must skip
**Overclaim wording:** Must NOT claim Zotero or EndNote integration without fixture-backed adapter evidence.

### 5. Live Provider Proof

**Status:** MISSING
**Test file:** None
**Required:** Opt-in provider smoke with timeout, retry, min-interval, and cache controls
**Env gate:** `SOURCERIGHT_PROVIDER_SMOKE`
**Default CI:** Must skip
**Overclaim wording:** Must NOT claim live provider verification without opt-in smoke evidence.

### 6. Registry Proof

**Status:** MISSING
**Test file:** None
**Required:** Public listing/install checks for accepted registries (GitHub Release, crates.io, MCP Registry)
**Default CI:** Should pass (listing checks are read-only)
**Overclaim wording:** Must NOT claim registry acceptance beyond evidence documented in release-status.md.

## Overclaim Guard

All documentation MUST use one of these status qualifiers for proof families:

| Status | Wording | When Used |
|--------|---------|-----------|
| ✅ Proven | "Supported — fixture-backed tests pass in default CI" | All required commands tested, fixtures exist, CI passes |
| 🔶 Partial | "Partially supported — some commands tested" | At least one command proven but not all required |
| ❌ Missing | "Not yet supported — tracked in track 45" | No fixture-backed evidence exists |
| 🔄 Opt-in | "Available with opt-in environment variables" | Live smoke exists but gated |
| ⏸️ Deferred | "Deferred — see track 45 for revisit trigger" | Intentionally not implemented |

The default claim for all 6 proof families is **MISSING** until proven otherwise.
