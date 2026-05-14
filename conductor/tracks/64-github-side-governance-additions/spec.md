# GitHub-Side Governance Additions Spec

## Goal

Ensure recommended GitHub-side governance settings are either configured or
explicitly documented as account/settings tasks.

## Required Checks

- Branch protection requires `CI`, `Security`, `Pages`, and relevant release
  checks before protected merges.
- Copilot coding agent is enabled if available and uses issue assignment.
- Renovate app is installed/enabled and remains low-noise.
- CodeQL/code scanning and Dependabot alerts are enabled.
- Coverage reporting app/service decision is made.
- Release environments protect crates.io and registry tokens.
- CODEOWNERS remains active.
- Labels/milestones support security, registry, plugin, provider, and external
  proof work.

Repo files may document or prepare these settings, but must not claim GitHub
state is enabled unless observed.
