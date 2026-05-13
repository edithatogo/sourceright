# Security, Publication, and Contract Governance Plan

1. Create the Conductor track and record the security/publication scope.
2. Inspect live Dependabot and code-scanning alerts.
3. Upgrade the vulnerable docs-site dependency set and add a lockfile.
4. Pin remaining unpinned GitHub Actions references by commit SHA.
5. Tune Renovate for quiet safe automerge while keeping majors manual.
6. Add the canonical feature contract matrix and design document to both docs
   surfaces.
7. Add conservative live-provider runtime controls and opt-in real demo smoke
   scripts for external-test readiness.
8. Add policy tests for the new contract docs and docs-site parity.
9. Run local validation, then check GitHub Actions after push.
10. Enable GitHub Copilot cloud-agent support through repository instructions,
    a least-privilege setup workflow, and a focused security-remediation issue
    template that can be assigned to `copilot-swe-agent[bot]` when the GitHub
    feature is enabled.

## Deferred

- GitHub notification email preferences are account-level, not repo-local.
- Copilot autofix/merge requires GitHub-side entitlement, repository settings,
  and branch-protection decisions; this track prepares the repo for that path
  but does not bypass review gates.
- OJS, Zotero, and live MCP client smoke tests should be opt-in external tests
  with fixtures and credentials handled outside default CI.
