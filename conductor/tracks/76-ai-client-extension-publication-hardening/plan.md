# Plan

## Phases

### 1. [ ] Discover

- **Claude Desktop**: Audit existing `docs/src/mcp.md` and `examples/mcp-clients/claude-desktop.json` for currency. Confirm the no-Claude-plugin position is stated. Check `host-manifest.json` for Claude status. Check `docs/src/release-status.md` Claude row.
- **Codex**: Audit `examples/mcp-clients/codex-config.toml`, `codex-mcp.json`, and `docs/src/mcp.md` Codex section. Confirm the no-Codex-plugin position is stated.
- **GitHub Copilot**: Review `.github/copilot-instructions.md`, `.github/workflows/copilot-setup-steps.yml`, `examples/mcp-clients/github-copilot-coding-agent.md` and `vscode-mcp.json`. Confirm no Copilot extension package exists.
- **Gemini CLI**: Research Gemini CLI MCP client configuration support via official documentation. Determine if Gemini CLI supports local MCP stdio clients, REST-based plugins, or has an extension directory.
- **Qwen CLI**: Research Qwen CLI MCP client configuration support via official documentation. Determine if Qwen CLI supports local MCP stdio clients or uses a separate plugin/extension system.
- **Evidence ledger**: Check `conductor/evidence-ledger.json` (or Track 69 `marketplace-evidence.md`) for existing AI client surface entries.
- **Submission packet**: Check `conductor/submission-packets/` for any existing AI client extension packet.
- **Docs-site**: Verify whether `docs-site/src/content/docs/mcp.md` exists.

### 2. [ ] Lock spec

- Document package decision format per client:
  - Claude Desktop: "client-configured — no Claude marketplace package"
  - Codex: "client-configured — no Codex marketplace package"
  - GitHub Copilot: "prepared coding-agent prep — no Copilot marketplace extension"
  - Gemini CLI: "TBD — research result" (to be determined by Subagent D)
  - Qwen CLI: "TBD — research result" (to be determined by Subagent E)
- Document transcript smoke gate per client: `sourceright mcp` initialize, tools/list, resources/list, prompts/list via the host-specific config.
- Record claim boundary: "client-configured" not "marketplace-published".
- Lock data contracts, parallelization plan, and evidence targets in `spec.md`.
- Assign subagents A–F to their parallel tasks.


### 3. [ ] Implement

- **Subagent A — Claude Desktop decision and transcript smoke**:
  - Revalidate the no-Claude-plugin position. Record finding in `package-decisions-2026-07-01.md`.
  - Run transcript smoke against `examples/mcp-clients/claude-desktop.json` config:
    ```text
    sourceright mcp  # start server
    # Send initialize, tools/list, resources/list, prompts/list via JSON-RPC
    ```
  - Record transcript in `transcript-smoke-2026-07-01.md`.

- **Subagent B — Codex decision and transcript smoke**:
  - Revalidate the no-Codex-plugin position. Record finding in `package-decisions-2026-07-01.md`.
  - Run transcript smoke against `examples/mcp-clients/codex-config.toml` config.
  - Record transcript in `transcript-smoke-2026-07-01.md`.

- **Subagent C — GitHub Copilot boundary revalidation**:
  - Review `.github/copilot-instructions.md`, setup workflow, security-remediation template, and `examples/mcp-clients/github-copilot-coding-agent.md`.
  - Confirm no Copilot extension package exists in the repository.
  - Record boundary finding in `package-decisions-2026-07-01.md`.
  - Optionally run transcript smoke if Copilot agent-mode MCP is confirmed functional.

- **Subagent D — Gemini CLI research**:
  - Research Gemini CLI at official docs (e.g., cloud.google.com/gemini-cli).
  - Determine MCP client support: local stdio, REST plugins, or none.
  - If supported, create `examples/mcp-clients/gemini-cli-mcp.json` with config example and run transcript smoke.
  - If not supported, record no-package decision with blocker rationale in `package-decisions-2026-07-01.md`.

- **Subagent E — Qwen CLI research**:
  - Research Qwen CLI at official docs (e.g., github.com/QwenLM or help.aliyun.com).
  - Determine MCP client support: local stdio, plugin system, or none.
  - If supported, create `examples/mcp-clients/qwen-cli-mcp.json` with config example and run transcript smoke.
  - If not supported, record no-package decision with blocker rationale in `package-decisions-2026-07-01.md`.


- **Subagent F — Documentation and evidence consolidation**:
  - Update `docs/src/mcp.md`:
    - Add "Gemini CLI" section with config example or no-support notice.
    - Add "Qwen CLI" section with config example or no-support notice.
    - Harden existing Claude/Codex/Copilot sections with package decision references.
    - Update Client Packaging Status table with Gemini and Qwen rows.
  - Create/update `docs-site/src/content/docs/mcp.md` mirroring the updated `docs/src/mcp.md`.
  - Create `conductor/tracks/76-ai-client-extension-publication-hardening/package-decisions-2026-07-01.md`.
  - Create `conductor/tracks/76-ai-client-extension-publication-hardening/transcript-smoke-2026-07-01.md`.
  - Scaffold `conductor/submission-packets/ai-client-extensions.md`:
    - Per-surface rows: Claude, Codex, Copilot, Gemini CLI, Qwen CLI.
    - Columns: surface name, package decision, config file, transcript smoke ref, claim boundary, gate status.
  - Update evidence ledger with per-surface entries.

### 4. [ ] Run checks

- **Transcript smoke** (fixture-backed):
  ```bash
  # Start MCP server
  sourceright mcp &
  # Send smoke requests from examples/mcp-clients/smoke-requests.jsonl
  ```
  Verify each JSON-RPC response includes expected `result` fields.
- **Config file validation**: Each config file in `examples/mcp-clients/` is valid JSON/TOML and references `sourceright mcp` as the stdio command.
- **Docs check**: `docs/src/mcp.md` and `docs-site` mirror contain sections for all five clients with package decision references.
- **Package decisions file check**: `package-decisions-2026-07-01.md` contains rows for all five clients.
- **Transcript smoke file check**: `transcript-smoke-2026-07-01.md` contains transcripts for clients with documented config paths.
- **Submission packet check**: `conductor/submission-packets/ai-client-extensions.md` exists with per-surface rows.
- **No marketplace-overclaim grep**: `grep -ri "accepted\|listed\|published\|approved" docs/src/mcp.md docs-site/src/content/docs/mcp.md | grep -v "not-accepted\|not-applicable\|deferred\|prepared\|client-configured"` should return no false-positive claims.
- **Standard checks**:
  ```bash
  cargo fmt --check
  cargo clippy --all-targets -- -D warnings
  cargo test
  ```

### 5. [ ] conductor-review

- Run `$conductor-review` gate:
  - Verify all `owned_paths` are present and consistent.
  - Verify package decisions are documented for all five surfaces.
  - Verify transcript smoke exists for each client with a documented config path.
  - Verify `docs/src/mcp.md` and `docs-site` mirror are updated.
  - Verify `conductor/submission-packets/ai-client-extensions.md` exists with per-surface rows.
  - Verify claim boundary is enforced in all documentation (no "marketplace-published" claims without listing evidence).
  - Record review finding in `conductor/tracks/76-ai-client-extension-publication-hardening/review.md`.

### 6. [ ] Apply fixes

- Apply any findings from `$conductor-review`:
  - Missing transcript smoke — run smoke and record transcript.
  - Missing config example — create config file.
  - Outdated docs — update `docs/src/mcp.md` and docs-site mirror.
  - Claim boundary violations — reword any "accepted" or "listed" language to "client-configured" or "prepared."
  - Missing submission packet — scaffold the packet.
- Re-run affected checks after fixes.

### 7. [ ] Progress — Advance to listing evidence only after review passes

- After `$conductor-review` passes and all fixes are applied:
  - Update `conductor/evidence-ledger.json` with AI client extension hardening evidence entries.
  - Update `docs/src/release-status.md` and docs-site mirror to reflect per-client package decision status.
  - Mark AI client extension surfaces in the marketplace evidence table as `package_hardened`.
  - Record track completion metadata.
  - Only promote any client from `prepared` to `accepted` if a public listing URL, version, date, and install metadata are verified in the evidence ledger.
