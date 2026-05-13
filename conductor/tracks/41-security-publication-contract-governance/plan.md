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

## Deferred

- GitHub notification email preferences are account-level, not repo-local.
- Copilot autofix requires GitHub-side entitlement and settings; this track only
  documents the feasible repo-local posture.
- OJS, Zotero, and live MCP client smoke tests should be opt-in external tests
  with fixtures and credentials handled outside default CI.
