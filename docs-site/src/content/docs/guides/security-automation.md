---
title: Security Automation
description: Quiet dependency, supply-chain, and Copilot cloud-agent automation for Sourceright.
---

Sourceright uses quiet-by-default automation for dependency and supply-chain
maintenance. The goal is fast remediation with reviewable pull requests, not a
steady stream of account notifications.

## Active Automation

| Surface | Status | Contract |
| --- | --- | --- |
| Renovate | Active | Groups routine minor, patch, pin, and digest updates into a monthly PR that is eligible for PR automerge after required checks pass. Major updates stay manual. |
| Dependabot alerts | Active in GitHub | Alerts identify vulnerable dependencies. Dependency updates are handled by Renovate to avoid duplicate bot PRs. |
| Dependency review | Active on pull requests | Blocks or reports risky dependency changes before merge. |
| CodeQL | Active | Uploads SARIF for Rust security analysis. |
| OpenSSF Scorecard | Active | Reports supply-chain posture through SARIF without changing code. |
| Copilot cloud agent | Prepared | Repository instructions, setup steps, and a security-remediation issue template are present. |

## Copilot Cloud-Agent Path

The repo does not run Copilot from the CLI. GitHub Copilot coding agent is a
GitHub-side feature that works from issues and pull requests when enabled for
the repository or organization.

Repo-local preparation lives in:

- `.github/copilot-instructions.md`
- `.github/workflows/copilot-setup-steps.yml`
- `.github/ISSUE_TEMPLATE/copilot_security_remediation.yml`

To delegate a security task, create a focused "Copilot security remediation"
issue and assign it to `copilot-swe-agent[bot]` if that assignee is available
in GitHub. The setup workflow gives the agent Rust, Node, cargo cache, locked
dependency fetches, and docs-site dependency installation.

Copilot-generated dependency or security PRs must still pass normal branch
protection and CI. The repo does not auto-create scheduled Copilot tasks or
notification-heavy reminder workflows.

## Notification Posture

Repository files cannot change personal GitHub email preferences. This repo
therefore avoids adding scheduled issue creation, broad bot assignment, or
default watcher notifications. Renovate uses grouped PRs, limited concurrency,
and no automerge assignees.

## GitHub App And Plugin Checks

Local `gh extension list` returned no installed GitHub CLI extensions in the
current environment. The GitHub REST endpoints for listing user or repository
app installations were not available to the current token, so installed
Marketplace apps must be verified in GitHub repository settings if that
inventory is needed.
