# Track 72: Submission Requirements Contracts

## Goal

Create the shared requirements contract that every external registry,
marketplace, plugin host, and upstream repository submission must satisfy before
Sourceright claims a submitted or accepted state.

## User Outcome

A maintainer can see which requirements were searched, which package contract
applies, which evidence is still missing, and which external issue, pull
request, registry submission, or marketplace listing is allowed next.

## Scope

- Official MCP Registry refresh, Smithery, Glama, Zotero, EndNote, OJS/PKP,
  arXiv, Claude Cowork, Codex app, GitHub Copilot, Gemini CLI extensions, Qwen
  CLI extensions, VS Code Marketplace, and Open VSX.
- Evidence gates from requirements searched through publicly accepted.
- Approval boundary for all external submissions.

## Out Of Scope

- Creating external issues, pull requests, listings, or registry submissions.
- Building every host package.
- Promoting prepared or deferred surfaces to accepted.

## Data Contracts

The source contract is `conductor/submission-contracts.md`; public mirrors live
in `docs/src/submission-contracts.md` and the docs-site mirror.

## Claim Boundary

This track may claim the submission requirements model exists. It must not claim
that any new external submission has been made or accepted.

## Evidence Level Target

Contracted.

## Parallelization Plan

Only this track edits the shared submission contract. Downstream tracks may work
in parallel after this track freezes host names, evidence gates, and claim
boundaries.

## Maturity, Stability, And Testing

The contract is mature only when default policy tests prove all requested
surfaces and gates are represented in Conductor and docs.
