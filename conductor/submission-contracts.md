# Submission Contracts

This file is the contract layer between prepared local artifacts and external
submissions. A surface may move to mature submission only when the owning track
records the host requirements, package contract, validation evidence, submission
artifact, external issue or pull request URL, registry/listing URL where
applicable, version or artifact id, verification date, and install metadata.

The machine-readable inventory is `conductor/submission-requirements.json`.
It is checked by `scripts/verify-submission-readiness.ps1` and the submission
readiness workflow. Local maintainer-facing submission drafts live under
`conductor/submission-packets/`.

Prepared metadata, local configuration, draft packages, and account-side drafts
are not accepted submissions. Private submissions also do not count as public
acceptance until a public URL or maintainer-verifiable receipt is recorded.

## Evidence Gates

| Gate | Required evidence |
| --- | --- |
| Requirements searched | Official documentation, repository contribution rules, registry requirements, and issue or PR templates are recorded with retrieval date. |
| Contracted | Host-specific package shape, permission model, dry-run/write semantics, compatibility matrix, and claim boundary are documented. |
| Hardened local package | Build, lint, schema validation, install/uninstall or client smoke, supply-chain metadata, and default-CI policy tests pass. |
| Submission-ready | Release artifact or patch branch exists, issue/PR body is drafted, maintainer-facing risk notes are complete, and no blocking contract gaps remain. |
| Submitted | External issue, pull request, marketplace listing, or registry submission URL is recorded with date, submitter, artifact id, and expected maintainer review path. |
| Publicly accepted | Public listing, merged PR, accepted registry entry, or maintainer acceptance URL is recorded with version/date/install metadata. |

## Cross-Surface Rules

- All write-capable integrations remain dry-run first and require explicit
  apply plus audit logs.
- Provider evidence stays in `references.verification.json`; external host
  integrations must not silently overwrite `references.csl.json`.
- Legal citation flows remain separate from academic CSL.
- Host package code must call the Rust CLI, stdio MCP server, or stable JSON
  contracts instead of reimplementing verification logic.
- Submission claims must stay below the evidence level in
  `conductor/evidence-ledger.json`.
- External submissions require an explicit human approval gate before creating
  issues, pull requests, marketplace listings, or registry entries.

## Surface Contracts

| Surface | Mature artifact | Submission target | Owning track | Minimum maturity contract |
| --- | --- | --- | --- | --- |
| Official MCP Registry | `server.json`, release OCI image, transcript proof | Official MCP Registry | 56, 72, 73 | Keep current accepted binding refreshed per release; record versioned registry evidence before claiming accepted. |
| Smithery | `.mcpb` bundle or supported Smithery package metadata | Smithery registry | 57, 72, 73 | Build from release binary, validate manifest, run local install smoke, then submit and record listing evidence. |
| Glama | Valid `glama.json`, repository metadata, MCP launch docs | Glama directory | 27, 72, 73 | Verify directory requirements, submit listing or metadata refresh, and record accepted URL/API evidence. |
| Zotero | CLI/Web API adapter package or explicit `.xpi` decision | GitHub Release, Zotero forum/listing path, or Zotero plugin path if chosen | 58, 72, 74 | Disposable-library smoke, preview/apply/audit proof, install notes, and accepted listing evidence before plugin claims. |
| EndNote | ENW/RIS handoff package and reference-checking guide | GitHub Release/docs distribution; EndNote plugin path only if a real plugin exists | 59, 72, 74 | Reparse checks, no live library mutation, clear export/import limits, and no EndNote-plugin claim without installable plugin evidence. |
| OJS/PKP | Generic plugin package | PKP Plugin Gallery or GitHub Release distribution | 60, 72, 75 | OJS compatibility matrix, settings/permissions docs, package build, fixture and opt-in test-instance smoke, and Gallery submission evidence. |
| arXiv submit-ce | Upstream-compatible module or documented integration patch | `arXiv/submit-ce` issue/PR | 71, 78, 79, 81 | Official repo requirements, API/schema compatibility, stable test fixtures, no writeback, security review, and maintainer-ready PR/issue. |
| arXiv submission-core | Legacy-compatible module or documented integration patch | `arXiv/arxiv-submission-core` issue/PR | 71, 78, 80, 81 | Domain/event contract review, backward-compatible fixtures, migration-safe behavior, no live credential dependency, and maintainer-ready PR/issue. |
| Claude Cowork / Claude Desktop | Host-specific package only if host supports one; otherwise MCP config | Claude/Cowork package path or documented no-package decision | 65, 72, 76, 84 | Transcript smoke, install proof, MCPB/connector submission, and accepted listing evidence. |
| Codex app | Codex-compatible plugin/package only if supported; otherwise MCP config | Codex app/plugin registry or documented no-package decision | 65, 72, 76, 85 | Target build requirements, MCP config smoke, package submission, and accepted listing evidence. |
| GitHub Copilot | Copilot extension/package or coding-agent setup evidence | GitHub Marketplace/Copilot extension path if available | 64, 65, 72, 76, 86 | Entitlement/settings proof, feature-complete extension or MCP path, and accepted listing evidence. |
| Gemini CLI extensions | Gemini CLI extension package | Gemini CLI extension repository/registry path | 72, 76, 87 | Official extension schema, install smoke, npm/OCX publish, and accepted listing evidence. |
| Qwen CLI extensions | Qwen CLI extension package | Qwen CLI extension repository/registry path | 72, 76, 88 | Official extension schema, install smoke, npm publish, and accepted listing evidence. |
| OpenCode | MCP config, npm plugin, or OCX bundle | OpenCode plugin registry / npm | 72, 76, 89 | MCP launch smoke, plugin load smoke, npm/OCX publish, and distribution URL evidence. |
| Cline MCP Marketplace | MCP server listing assets | `cline/mcp-marketplace` issue | 72, 76, 90 | Logo, llms-install.md, Cline install smoke, marketplace issue, and accepted listing evidence. |
| VS Code / Open VSX | VSIX extension | VS Code Marketplace and Open VSX | 66, 72, 77, 83 | Feature-complete VSIX, install smoke, Marketplace/Open VSX publish, and accepted listing evidence. |

## arXiv Granularity Standard

arXiv work is split more finely than other hosts because upstream module
submission touches external submission platforms rather than only a local
plugin package.

1. Requirements reconnaissance must search official docs, repo contribution
   files, open issues/PRs, API/schema files, license, and local development
   commands for `arXiv/submit-ce` and `arXiv/arxiv-submission-core`.
2. Current `submit-ce` hardening must prove parser stability, source-bundle
   fixture coverage, API drift detection, and no arXiv-side writeback.
3. Legacy `arxiv-submission-core` hardening must prove event/domain-model
   compatibility, migration-safe output, and backward-compatible fixture
   behavior.
4. Upstream submission may start only after both platform hardening tracks pass
   their maturity, stability, and testing gates.
5. Upstream issues and PRs must include the local evidence matrix, security
   boundary, fixture summary, maintenance burden, rollback path, and explicit
   statement that Sourceright does not submit papers or mutate arXiv state.
