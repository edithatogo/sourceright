# Sourceright GitHub Copilot Coding-Agent Boundary

Sourceright has repository preparation for GitHub Copilot coding-agent work:

- `.github/copilot-instructions.md`
- `.github/workflows/copilot-setup-steps.yml`
- `.github/ISSUE_TEMPLATE/copilot_security_remediation.yml`

The prepared repository workflow still delegates changes against the same
`sourceright` CLI and MCP boundaries as other clients.

This is not a Copilot extension or Marketplace package. Copilot support remains
issue/PR workflow preparation until GitHub-side entitlement is enabled and a
separate extension package exists.

## Safe Use Pattern

1. Create a focused issue that references the relevant Conductor track.
2. Assign Copilot only if the repository/org entitlement is active.
3. Require the normal CI, security, coverage, and review gates before merge.
4. Treat any MCP write path as dry-run first unless `apply: true` is explicit.
