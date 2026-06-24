# Submission Packets

Submission packets are local, maintainer-facing drafts. They collect searched
requirements, package gates, smoke evidence, and draft external issue/PR/listing
content before anything is submitted externally.

No packet is itself an external submission. External issues, pull requests,
marketplace listings, and registry entries still require explicit approval.

| Packet | Surfaces | Owning tracks |
| --- | --- | --- |
| `mcp-directories.md` | Official MCP Registry, Smithery, Glama | 73 |
| `citation-managers.md` | Zotero, EndNote | 74 |
| `journal-platforms.md` | OJS/PKP plus arXiv boundaries | 75 |
| `arxiv-upstream.md` | `arXiv/submit-ce`, `arXiv/arxiv-submission-core` | 78-81 |
| `ai-client-extensions.md` | Claude Cowork/Desktop, Codex app, Copilot, Gemini CLI, Qwen CLI | 76 |
| `vscode-open-vsx.md` | VS Code Marketplace, Open VSX | 77 |
| `agent-workflow.md` | Self-improving agent/skill/workflow rules | 82 |

Each packet must include:

- requirement sources and retrieval dates;
- package or no-package decision;
- local validation commands;
- blockers;
- approval gate;
- draft external submission body or explicit reason it is not ready.

`manifest.json` is the machine-readable packet index. The submission-readiness
verifier checks that every packet path exists, every claimed submission surface
exists in `conductor/submission-requirements.json`, blocked packets retain
blockers, and external submission remains approval-gated.
