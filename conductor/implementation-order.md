# Implementation Order

Use this order to keep the roadmap moving without parallel drift.

## Foundation First

1. Track 47: contract evidence and overclaim gates.
2. Track 63: plugin packaging and supply-chain maturity.
3. Track 64: GitHub-side governance additions.
4. Track 42: GitHub automation and alert operations.
5. Track 55: benchmark robustness contract.

## Core Hardening

These can run in parallel after the foundation gates are stable:

- Track 36: document extraction hardening.
- Track 37: live core provider verification.
- Track 38: citation matching disambiguation.
- Track 39: URL/archive integrity.
- Track 40: low-noise writeback suggestions.
- Track 62: expanded normaliser/provider catalogue.

## Integration Lanes

These can run in parallel when their owned paths do not overlap:

- Track 48: public API provider adapters.
- Track 49: licensed BYO-key provider adapters.
- Track 50: repository record provider adapters.
- Track 58: mature Zotero plugin.
- Track 59: other citation-manager integrations.
- Track 60: mature OJS plugin.
- Track 61: Streamlit app publication and hardening.
- Track 65: AI client MCP packaging.
- Track 66: VS Code extension packaging.
- Track 67: Microsoft Word add-in packaging.
- Track 68: LibreOffice extension packaging.
- Track 83: Janeway platform reconnaissance.
- Track 84: Janeway plugin package hardening.
- Track 85: Janeway install and smoke evidence.

## Platform Expansion

- Track 86: proprietary journal-platform matrix.
- Track 87: self-improving platform registry.

## Publication And Proof

- Track 45 proves external behavior.
- Track 43 records registry status.
- Track 56 binds MCP release artifacts.
- Track 57 closes Smithery.
- Track 54 proves demos.
- Track 69 records marketplace submission evidence for host packages.
- Track 70 refreshes release-surface evidence before publication wording changes.
- Track 81 records arXiv upstream submission and acceptance evidence after
  requirements, maturity, stability, testing, and approval gates pass.
- Track 82 validates the submission inventory and repo-health target before
  submission-related wording changes.

## Host Plugin Submission and Acceptance

Tracks 83–90 convert hardened local MCP/config artifacts into feature-complete
host packages and approval-gated public submissions. Run after Track 82.

1. Track 83: VS Code and Open VSX submission and acceptance.
2. Track 84: Claude Desktop package submission and acceptance.
3. Track 85: Codex app package submission and acceptance.
4. Track 86: GitHub Copilot extension submission and acceptance.
5. Track 87: Gemini CLI extension submission and acceptance.
6. Track 88: Qwen CLI extension submission and acceptance.
7. Track 89: OpenCode plugin submission and acceptance.
8. Track 90: Cline MCP Marketplace submission and acceptance.

Tracks 87–89 can run in parallel when extension scaffolds do not overlap.
Track 83 should land before Track 86 if a shared VS Code/Copilot MCP surface is reused.

## Parallelization Rule

Subagents may work in parallel only when their owned paths do not overlap.
Shared contracts, schemas, and public docs are edited serially by the lead.

## Submission Hardening

- Track 72: submission requirements contracts and readiness checks.
- Track 77: editor and registry publication hardening.
- Track 78: arXiv upstream requirements reconnaissance.
- Track 79: arXiv submit-ce maturity hardening.
- Track 80: arXiv submission-core maturity hardening.
- Track 81: arXiv upstream submission and acceptance.
