# Downstream Requirements Handoff

Track 72 freezes host names, evidence gates, and claim boundaries. Downstream
tracks may implement packages and submission bodies only after the requirements
sources below are recorded. This handoff maps each downstream track to its
official requirements evidence and owning submission packet.

| Downstream track | Surfaces | Requirements evidence | Submission packet | Package evidence (if any) |
| --- | --- | --- | --- | --- |
| 73 MCP directory submission hardening | Official MCP Registry, Smithery, Glama | `conductor/submission-requirements.json` surfaces `official-mcp-registry`, `smithery`, `glama` | `conductor/submission-packets/mcp-directories.md` | `conductor/tracks/73-mcp-directory-submission-hardening/smithery-mcpb-build-2026-06-09.md` |
| 74 Citation-manager publication hardening | Zotero, EndNote | `zotero`, `endnote` inventory rows | `conductor/submission-packets/citation-managers.md` | `conductor/tracks/74-citation-manager-publication-hardening/zotero-adapter-hardening-2026-06-09.md`, `endnote-reparse-verification-2026-06-09.md` |
| 75 Journal-platform publication hardening | OJS/PKP | `ojs-pkp` inventory row | `conductor/submission-packets/journal-platforms.md` | `conductor/tracks/75-journal-platform-publication-hardening/ojs-fixture-smoke-2026-06-09.md`, `ojs-compatibility-matrix.md` |
| 76 AI client extension publication hardening | Claude, Codex, Copilot, Gemini CLI, Qwen CLI | `claude-cowork`, `codex-app`, `github-copilot`, `gemini-cli-extensions`, `qwen-cli-extensions` | `conductor/submission-packets/ai-client-extensions.md` | `conductor/tracks/76-ai-client-extension-publication-hardening/mcp-client-smoke-2026-06-09.md`, `package-decisions-2026-05-18.md` |
| 77 VS Code and Open VSX publication hardening | VS Code Marketplace, Open VSX | `vscode-open-vsx` inventory row | `conductor/submission-packets/vscode-open-vsx.md` | `conductor/tracks/77-vscode-open-vsx-publication-hardening/vsix-smoke-2026-06-09.md`, `marketplace-metadata-draft.md`, `submission-drafts.md` |

## arXiv Granular Tracks (78–81)

arXiv upstream work is not a host package lane. Requirements reconnaissance and
maturity hardening run on separate tracks after Track 72 freezes the contract.

| Downstream track | Surfaces | Requirements evidence | Submission packet | Package evidence (if any) |
| --- | --- | --- | --- | --- |
| 78 arXiv upstream requirements recon | `arXiv/submit-ce`, `arXiv/arxiv-submission-core` | `arxiv-submit-ce`, `arxiv-submission-core` inventory rows | `conductor/submission-packets/arxiv-upstream.md` | `conductor/tracks/78-arxiv-upstream-requirements-recon/requirements-matrix.md` |
| 79 arXiv submit-ce maturity hardening | `arXiv/submit-ce` | Track 78 matrix plus Track 79 spec | `arxiv-upstream.md` | `conductor/tracks/79-arxiv-submit-ce-maturity-hardening/schema-drift-check-2026-06-09.md`, `security-boundaries.md`, `evidence-packet.md` |
| 80 arXiv submission-core maturity hardening | `arXiv/arxiv-submission-core` | Track 78 matrix plus Track 80 spec | `arxiv-upstream.md` | `conductor/tracks/80-arxiv-submission-core-maturity-hardening/migration-mapping-check-2026-06-09.md`, `security-boundaries.md`, `evidence-packet.md` |
| 81 arXiv upstream submission and acceptance | Both arXiv repos | Tracks 78–80 gates | `arxiv-upstream.md` | `conductor/tracks/81-arxiv-upstream-submission-and-acceptance/readiness-review-2026-06-09.md`, `submission-drafts.md`, `approval-gates.md` |

## Retrieval Standard

Every `requirements_sources` row in `conductor/submission-requirements.json`
must:

- cite an official host document, registry page, or upstream repository path
- record `status: searched` and `retrieved_at`
- stay aligned with the matching submission packet table

Downstream tracks must not promote a surface beyond the lowest unmet gate in the
inventory.
