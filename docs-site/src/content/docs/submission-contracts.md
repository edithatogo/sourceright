---
title: Submission Contracts
description: Requirements and evidence gates for external registry, marketplace, plugin, and upstream repository submissions.
---

Submission contracts define the difference between prepared local artifacts and
external submission evidence. A mature submission requires searched host
requirements, a documented package contract, validation evidence, an issue or
pull request body where relevant, and the external registry/listing/PR evidence
after submission.

The machine-readable inventory is `conductor/submission-requirements.json`.
It is checked by `scripts/verify-submission-readiness.ps1` and the submission
readiness workflow. Local maintainer-facing submission drafts live under
`conductor/submission-packets/`. The current repo-health target for submission
work is 9.5.

Prepared metadata, local configuration, draft packages, and private account-side
drafts are not public acceptance.

## Required Gates

| Gate | Evidence |
| --- | --- |
| Requirements searched | Official documentation, contribution rules, registry requirements, and issue or PR templates recorded with date. |
| Contracted | Package shape, permissions, dry-run/write semantics, compatibility matrix, and claim boundary. |
| Hardened local package | Build, lint, schema validation, install or client smoke, supply-chain metadata, and policy tests. |
| Submission-ready | Release artifact or patch branch, drafted issue/PR/listing body, and no blocking contract gaps. |
| Submitted | External issue, pull request, marketplace listing, or registry submission URL with date and artifact id. |
| Publicly accepted | Public listing, merged PR, accepted registry entry, or maintainer acceptance URL with version/date/install metadata. |

## Surface Status

The submission contract covers Official MCP Registry, Smithery, Glama, Zotero,
EndNote, OJS/PKP, arXiv `submit-ce`, arXiv `arxiv-submission-core`, Claude
Cowork/Claude Desktop, Codex app, GitHub Copilot, Gemini CLI extensions, Qwen
CLI extensions, and VS Code/Open VSX.

Each host must keep claims below the evidence level in the Conductor evidence
ledger. External submissions require explicit approval before issues, pull
requests, marketplace listings, or registry entries are created.

## arXiv Standard

arXiv work is split into granular tracks: requirements reconnaissance, current
`submit-ce` hardening, legacy `arxiv-submission-core` hardening, and upstream
submission evidence. Upstream issues or PRs are blocked until maturity,
stability, compatibility, security, and fixture-backed testing gates pass.

Sourceright arXiv adapters screen submission metadata and source-bundle signals.
They do not submit papers, mutate arXiv state, write back to arXiv systems, or
silently change canonical CSL.
