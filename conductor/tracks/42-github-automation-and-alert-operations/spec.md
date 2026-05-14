# GitHub Automation And Alert Operations Spec

## Goal

Verify and harden GitHub-side automation without creating noisy notifications
or bypassing review gates.

## Scope

- Confirm Copilot coding agent availability and document the exact issue
  assignment path.
- Verify installed GitHub Apps or Marketplace integrations from GitHub settings
  when API scopes allow it.
- Re-check Dependabot and code-scanning alert state after dependency fixes.
- Keep Renovate as the dependency-update PR engine unless a better low-noise
  policy is explicitly chosen.
- Keep Copilot work issue-driven and review-gated.

## Out Of Scope

- Changing personal email notification preferences.
- Enabling organization-level Copilot or Marketplace apps without explicit
  account-side action.
- Auto-merging major upgrades or security fixes that fail required checks.

## Parallelization

- Subagent A: GitHub settings and installed-app inventory.
- Subagent B: alert-state inventory and stale alert diagnosis.
- Subagent C: workflow/Renovate/Copilot setup review.

Subagents must not edit the same `.github/` workflow files in parallel.
