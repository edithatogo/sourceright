# Test Matrix

## Claude Desktop — default CI

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| Package decision documented | `package-decisions-2026-07-01.md` records Claude as "client-configured — no Claude marketplace package" | File exists with Claude row | default-CI |
| Config example valid | `claude-desktop.json` valid JSON referencing `sourceright mcp` | JSON validation pass | default-CI |
| Transcript smoke passes | `transcript-smoke-2026-07-01.md` contains Claude section with `initialize`, `tools/list`, `resources/list`, `prompts/list` transcript | Transcript file contains Claude section | default-CI |
| Dry-run write safety | Transcript shows `workspace.init` without `apply` returns `apply_requested: false` | Transcript contains dry-run call | default-CI |
| No marketplace overclaim | Uses "client-configured" — never "Claude-approved" or "marketplace-published" | grep returns no false positives | default-CI |

## Codex — default CI

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| Package decision documented | `package-decisions-2026-07-01.md` records Codex as "client-configured — no Codex marketplace package" | File exists with Codex row | default-CI |
| Config examples valid | `codex-config.toml` and `codex-mcp.json` valid, reference `sourceright mcp` | TOML/JSON validation pass | default-CI |
| Transcript smoke passes | `transcript-smoke-2026-07-01.md` contains Codex section with transcript | Transcript file contains Codex section | default-CI |
| No marketplace overclaim | Uses "client-configured" — never "Codex-listed" or "Codex marketplace package" | grep returns no false positives | default-CI |

## GitHub Copilot — default CI

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| Package decision documented | `package-decisions-2026-07-01.md` records Copilot as "prepared coding-agent prep — no Copilot marketplace extension" | File exists with Copilot row | default-CI |
| Coding-agent prep files present | `.github/copilot-instructions.md`, setup workflow, security template exist | File existence check | default-CI |
| VS Code MCP config valid | `vscode-mcp.json` valid JSON referencing `sourceright mcp` | JSON validation pass | default-CI |
| Copilot boundary documented | `github-copilot-coding-agent.md` states prep not extension | File contains boundary statement | default-CI |
| No Copilot extension package | No `plugins/copilot/` or `copilot-extension.*` in repo | Repo scan confirms none | default-CI |
| No marketplace overclaim | Uses "coding-agent prep" — never "Copilot extension" | grep returns no false positives | default-CI |

## Gemini CLI — opt-in live

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| MCP support researched | decision file documents finding (supported / not / deferred) with source URL | File exists with Gemini row | opt-in-live |
| Config example (if supported) | `gemini-cli-mcp.json` exists with valid JSON if stdio supported | File existence check | opt-in-live |
| Transcript (if supported) | transcript contains Gemini CLI section if config exists | Transcript file section present | opt-in-live |
| No-package decision (if unsupported) | decision file records blocker rationale and revisit trigger | File row explains blocker | opt-in-live |
| Documentation updated | `docs/src/mcp.md` and docs-site contain Gemini section | File review confirms section | opt-in-live |

## Qwen CLI — opt-in live

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| MCP support researched | decision file documents finding (supported / not / deferred) with source URL | File exists with Qwen row | opt-in-live |
| Config example (if supported) | `qwen-cli-mcp.json` exists with valid JSON if stdio supported | File existence check | opt-in-live |
| Transcript (if supported) | transcript contains Qwen CLI section if config exists | Transcript file section present | opt-in-live |
| No-package decision (if unsupported) | decision file records blocker rationale and revisit trigger | File row explains blocker | opt-in-live |
| Documentation updated | `docs/src/mcp.md` and docs-site contain Qwen section | File review confirms section | opt-in-live |

## Consolidated evidence — default CI

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| All five decisions recorded | decision file has rows for Claude, Codex, Copilot, Gemini CLI, Qwen CLI | File row count >= 5 | default-CI |
| Transcript file has Claude + Codex minimum | transcript file has sections for supported clients | Section count >= 2 | default-CI |
| `docs/src/mcp.md` updated | Sections for all five clients with package decisions and claim boundaries | File review | default-CI |
| `docs-site/src/content/docs/mcp.md` exists | Starlight MCP page exists (mirror/summary) | File existence check | default-CI |
| Submission packet exists | `conductor/submission-packets/ai-client-extensions.md` with per-surface rows | File existence; row count >= 5 | default-CI |
| No marketplace overclaims | Docs use "client-configured"/"prepared" — never "approved"/"listed"/"published" without evidence | grep returns no false positives | default-CI |
| Host manifest updated | `host-manifest.json` includes Gemini CLI and Qwen CLI rows | File review confirms | default-CI |
