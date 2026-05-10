# Citation Manager Live Sync Plan

1. Define sync contracts.
   - Model local records, remote records, match keys, planned actions, and conflicts.
   - Reuse export-suite serialization where practical.
   - Align preview/apply semantics with MCP write-tool audit behavior.

2. Add Zotero preview.
   - Authenticate with user-provided credentials or tokens.
   - Build create, update, skip, and conflict plans.
   - Do not write remote data during preview.

3. Add Zotero apply.
   - Require explicit apply intent.
   - Apply only validated plans.
   - Record audit logs for remote actions and resulting identifiers.

4. Add conflict handling.
   - Detect local/remote field conflicts.
   - Preserve reviewable diffs.
   - Refuse ambiguous updates without user resolution.

5. Add tests and documentation.
   - Use fixtures for default tests.
   - Gate live Zotero smoke tests behind credentials.
   - Document preview, apply, audit, and rollback limitations.
