# Test Matrix

## Server card generation parity

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| Checked-in `server-card.json` matches generated output from `mcp::server_card()` | Policy test `smithery_distribution_policy.rs` passes: no diff between checked-in card and CLI-generated card | `tests/smithery_distribution_policy.rs` (`#[test] fn checked_in_server_card_matches_generated`) | default-CI |
| `sourceright mcp server-card --json` exits 0 and outputs valid SEP-1649 JSON | CLI tool returns valid JSON with `serverInfo`, `tools`, `resources`, `prompts`, `authentication` | GitHub Actions workflow or local test run | default-CI |
| Dual-location card files are identical | `diff .well-known/mcp/server-card.json docs-site/public/.well-known/mcp/server-card.json` exits 0 | Policy test or CI step | default-CI |

## Directory listing probes (opt-in live)

### Smithery

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| Smithery listing probe | `https://smithery.ai/servers/edithatogo/sourceright` returns HTTP 200 with valid server page | `browser-listing-verification-*.md` row with probe URL and 200 status | opt-in-live |
| Smithery release scan pass | Smithery release scan returns green (not 422 or 405) | `smithery-server-card-*.md` scan result table row with green status | opt-in-live |
| Smithery search result | `https://smithery.ai/search?q=sourceright` returns Sourceright in first page results | `browser-listing-verification-*.md` search result row | opt-in-live |
| Smithery install metadata | Smithery listing page shows install command and version 0.1.20 | Screenshot or HTML scrape recorded in evidence doc | opt-in-live |

### Glama

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| Glama listing probe | `https://glama.ai/mcp/servers/edithatogo/sourceright` or API equivalent returns HTTP 200 | `browser-listing-verification-*.md` row with probe URL and 200 status | opt-in-live |
| Glama search result | `https://glama.ai/mcp/servers?q=sourceright` or `?q=edithatogo` returns listing match | `browser-listing-verification-*.md` search result row | opt-in-live |
| Glama install metadata | Glama listing shows server name `sourceright` and valid metadata from `glama.json` | Screenshot or HTML scrape recorded in evidence doc | opt-in-live |

## GitHub Pages well-known path

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| `/.well-known/mcp/server-card.json` on Pages host | `https://edithatogo.github.io/sourceright/.well-known/mcp/server-card.json` returns HTTP 200 with JSON body | `smithery-server-card-*.md` or new evidence doc with curl/probe result | opt-in-live |
| `/.well-known/` path does not return Starlight 404 | HTTP status is 200, not 404 or 405 | Probe result recorded | opt-in-live |

## Live evidence recording

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| Smithery accepted listing recorded | `conductor/submission-packets/live-evidence.json` contains entry for Smithery with `status: "accepted"`, URL, version | `live-evidence.json` readback | opt-in-live |
| Glama accepted listing recorded | `conductor/submission-packets/live-evidence.json` contains entry for Glama with `status: "accepted"`, URL, version | `live-evidence.json` readback | opt-in-live |
| Release-status docs updated | `docs/src/release-status.md` and docs-site mirror reflect accepted Smithery and Glama (not just prepared) | Docs build and review | opt-in-live |
