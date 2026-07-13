# Track 86: GitHub Copilot Extension Submission and Acceptance

## Goal

Complete a Copilot-facing package (extension or MCP distribution) and submit to the supported GitHub path.

## User Outcome

Operators can install Sourceright through the host's supported package or listing
path with feature-complete behavior, install smoke, and public acceptance evidence.

## Scope

- Feature-complete host package or marketplace listing assets for `github-copilot`.
- Official requirements reconnaissance and submission mechanism documentation.
- Approval-gated external submission and `live-evidence.json` promotion.

## Out Of Scope

- Reimplementing Sourceright verification in host-native languages.
- Claiming acceptance before listing URL, API evidence, or maintainer receipt exists.

## Data Contracts

Host packages call `sourceright` CLI/MCP with stable JSON outputs. Write paths
remain preview-first with audit evidence.

## Submission Target

GitHub Marketplace/Copilot extension path or verified MCP config distribution.

## Required Artifact

Copilot extension, MCP config, or coding-agent bundle with entitlement proof.

## Claim Boundary

No `github-copilot` acceptance claim until submission evidence is recorded
with URL, date, version, and install metadata.

## Evidence Level Target

Feature-complete local package → submission_ready → submitted → publicly_accepted.

## Parallelization Plan

Requirements recon, package scaffold, and install smoke can run in parallel after
shared MCP contracts are frozen.

## Maturity, Stability, And Testing

Maturity requires requirements search, package validation, install smoke,
submission drafts, approval gate, and live evidence verifier pass.
