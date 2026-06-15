# Plan

## Phases

### 1. [x] Discover
- Investigated Smithery release scan failure (422) caused by missing `/.well-known/mcp/server-card.json` on the publish URL origin.
- Identified Smithery's dual path: URL publishing (requires reachable well-known card) and MCPB bundle publishing (returns 400 `No values to set` on Win32).
- Confirmed Glama requires sign-in for Add Server flow; no public listing exists.
- Verified GitHub Pages blocks dot-prefixed paths (`/.well-known/...`, `/.nojekyll`) returning Starlight 404.
- Evidence: `smithery-server-card-2026-06-10.md`, `browser-listing-verification-2026-06-10.md`.

### 2. [x] Lock spec
- SEP-1649 server card schema derived from live `tools_list` (14), `resources_list` (8), `prompts_list` (5).
- Card includes `serverInfo` (name + version), `authentication` (required: false), and full arrays for tools, resources, prompts.
- Spec locked in `spec.md` with data contracts, claim boundary, and parallelization plan.

### 3. [x] Implement
- `mcp::server_card()` in `src/mcp.rs` — derives SEP-1649 card from live MCP surface.
- `sourceright mcp server-card [--json]` — CLI emission for regeneration.
- `scripts/generate-mcp-server-card.ps1` — writes dual-location card:
  - `.well-known/mcp/server-card.json`
  - `docs-site/public/.well-known/mcp/server-card.json`
- `tests/smithery_distribution_policy.rs` — policy test enforcing checked-in card parity with generated output.
- `docs/src/publishing.md` — documents URL-publish server-card contract.
- Repo-root card committed: `.well-known/mcp/server-card.json` (verified 2026-06-10).

### 4. [x] Run checks
- Local docs build copies card to `docs-site/dist/.well-known/mcp/server-card.json`.
- Astro prerender route builds on CI: `/.well-known/mcp/server-card.json (+5ms)` in Pages workflow `27230037375`.
- Policy test passes: checked-in card matches generated output.
- CLI tool tested: `cargo run -- sourceright mcp server-card --json` produces valid SEP-1649 JSON.

### 5. [ ] Browser listing verification for Smithery and Glama
- **Smithery**: Re-probe `https://smithery.ai/servers/edithatogo/sourceright` once server-card is reachable on Pages or alternative URL. Attempt republish with `smithery mcp publish https://edithatogo.github.io/sourceright/`. Record scan result (green/422/405).
- **Glama**: Sign in to Glama, use Add Server flow with `https://github.com/edithatogo/sourceright`. Record listing URL or blocking error.
- Document results in updated `browser-listing-verification-*.md`.

### 6. [ ] Resolve GitHub Pages dot-path 404 blocker
- Investigate GitHub Pages `/.well-known/` 404: possible causes are Starlight custom 404 handler, Jekyll `.nojekyll` bypass, or Pages static serving restriction.
- Options to resolve:
  a. Add `/.nojekyll` file to docs-site dist root (already present?).
  b. Configure Astro/Starlight to serve `/.well-known/mcp/server-card.json` via a prerendered route.
  c. Use an alternative publish URL that does not block dot paths (e.g., custom domain, raw GitHub, or GitHub release artifact).
- Confirm live URL returns 200 with `serverInfo.version` 0.1.20.

### 7. [x] conductor-review
- Server card generation, CLI tool, script, and policy test reviewed.
- Evidence docs (`smithery-server-card-*.md`, `browser-listing-verification-*.md`) recorded and checked against claim boundary.
- `$conductor-review` applied before any surface promotion to `submitted`.

### 8. [ ] Apply fixes
- Apply GitHub Pages dot-path workaround.
- Republish Smithery against resolved URL.
- Complete Glama Add Server flow.
- Write accepted listing URLs and scan status to `conductor/submission-packets/live-evidence.json`.

### 9. [ ] Progress — Record accepted listing URLs in live-evidence.json
- Only promote Smithery and Glama from `prepared` to `accepted` after:
  - Public listing URL returns 200.
  - Release scan returns green (Smithery) or equivalent acceptance (Glama).
  - `live-evidence.json` entry recorded with `{surface, url, status: "accepted", version, date}`.
- Update `docs/src/release-status.md` and `docs-site` mirror to reflect accepted status.
