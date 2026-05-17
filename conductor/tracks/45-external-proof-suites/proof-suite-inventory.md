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

**Status:** ✅ Proven
**Test file:** `tests/cli_end_to_end.rs` (exists), `tests/demo_policy.rs` (partial)
**Commands required:** `init`, `validate-csl`, `report`, `export`, `bench`, `citation-sync`, `mcp status`
**Currently tested:** Binary invocation and major dispatch paths for required CLI commands
**Fixture path:** `github_pages_demo/sample/`, `streamlit_app/sample_workspace/`
**Opt-in live:** N/A
**Default CI:** Passes through `cli_end_to_end.rs`
**Overclaim wording:** Claim CLI smoke coverage only for documented fixture-backed commands and help/status surfaces.

### 2. MCP Stdio Transcript Smoke

**Status:** ✅ Proven for discovery transcript; live client startup remains opt-in
**Test file:** `tests/cli_end_to_end.rs`
**Required:** Client-style transcript proving tool, resource, and prompt discovery via stdio-compatible CLI JSON surfaces
**Fixture path:** `conductor/tracks/45-external-proof-suites/mcp-transcript-proof.md`
**Opt-in live:** Full named-client startup remains outside default CI
**Default CI:** Passes discovery surface checks
**Overclaim wording:** Must NOT claim named-client compatibility without client-specific transcript evidence.

### 3. OJS Proof

**Status:** ✅ Proven for fixture-backed screening; live OJS remains opt-in/deferred
**Test file:** `tests/cli_end_to_end.rs`
**Required:** Fixture-backed adapter contract producing editor and author screening outputs
**Fixture path:** `fixtures/journal/ojs-submission.json`
**Opt-in live:** Optional disposable OJS/test instance smoke
**Default CI:** Passes `ojs_fixture_screens_to_editor_and_author_outputs_end_to_end`
**Overclaim wording:** Must NOT claim live OJS deployment or PKP Gallery acceptance without external evidence.

### 4. Citation-Manager Proof (Zotero/EndNote)

**Status:** ✅ Proven for Zotero fixture-backed engine; EndNote remains file-export only
**Test file:** `src/citation_sync.rs` unit tests and `tests/cli_end_to_end.rs`
**Required:** Preview/apply/audit semantics fixture-backed for Zotero; EndNote export handoff proof
**Fixture path:** `fixtures/providers/zotero/`
**Opt-in live:** Optional disposable-library smoke (Zotero API)
**Default CI:** Passes fixture-backed citation-sync and CLI surface checks
**Overclaim wording:** Must NOT claim Zotero `.xpi`, Zotero Plugin Gallery acceptance, or live EndNote sync.

### 5. Live Provider Proof

**Status:** ✅ Proven for runtime-control contract; live API calls remain opt-in
**Test file:** `src/live_providers.rs`
**Required:** Opt-in provider smoke with timeout, retry, min-interval, and cache controls
**Env gate:** `SOURCERIGHT_LIVE_PROVIDERS` and `SOURCERIGHT_LIVE_PROVIDER_SMOKE`
**Default CI:** Passes conservative defaults, skipped-smoke report, fixture parsing, and cache-read tests
**Overclaim wording:** Must NOT claim live provider API verification without opt-in smoke evidence.

### 6. Registry Proof

**Status:** ✅ Proven for accepted/prepared/deferred evidence boundaries
**Test file:** `tests/mcp_distribution_checks.rs`, `tests/marketplace_submission_evidence_policy.rs`
**Required:** Public listing/install checks for accepted registries (GitHub Release, crates.io, MCP Registry)
**Default CI:** Passes repo-local evidence checks
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

All six proof families now have repo-local proof. Live service and marketplace
acceptance claims remain opt-in or external-evidence-gated as documented above.
