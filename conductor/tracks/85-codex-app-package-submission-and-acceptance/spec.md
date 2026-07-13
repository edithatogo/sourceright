# Track 85: Codex App Package Submission and Acceptance

## Goal

Explore Codex app MCP/plugin distribution and complete any supported package submission.

## User Outcome

Operators can install Sourceright through the host's supported package or listing
path with feature-complete behavior, install smoke, and public acceptance evidence.

## Scope

- Feature-complete host package or marketplace listing assets for `codex-app`.
- Official requirements reconnaissance and submission mechanism documentation.
- Approval-gated external submission and `live-evidence.json` promotion.

## Out Of Scope

- Reimplementing Sourceright verification in host-native languages.
- Claiming acceptance before listing URL, API evidence, or maintainer receipt exists.

## Data Contracts

Host packages call `sourceright` CLI/MCP with stable JSON outputs. Write paths
remain preview-first with audit evidence.

## Submission Target

Codex MCP server registration or app plugin path per OpenAI Codex docs.

## Required Artifact

Codex MCP config package or documented registry artifact with install smoke.

## Claim Boundary

No `codex-app` acceptance claim until submission evidence is recorded
with URL, date, version, and install metadata.

## Evidence Level Target

Feature-complete local package → submission_ready → submitted → publicly_accepted.

## Parallelization Plan

Requirements recon, package scaffold, and install smoke can run in parallel after
shared MCP contracts are frozen.

## Maturity, Stability, And Testing

Maturity requires requirements search, package validation, install smoke,
submission drafts, approval gate, and live evidence verifier pass.
