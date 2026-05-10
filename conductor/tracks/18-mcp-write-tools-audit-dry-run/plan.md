# MCP Write Tools Audit Dry-Run Plan

1. Define write-tool contracts.
   - Identify workspace update operations that are safe to expose.
   - Separate canonical CSL updates from verification sidecar and derived queue updates.
   - Define required request fields for dry-run and apply modes.

2. Add dry-run change planning.
   - Generate structured plans before writes.
   - Include file paths, record ids, before/after summaries, conflicts, and validation status.
   - Default all tool calls to dry-run.

3. Add schema validation and audit logs.
   - Validate planned writes before apply.
   - Write audit records for applied changes.
   - Include tool name, request provenance, timestamps, affected files, and result.

4. Add explicit apply semantics.
   - Require a clear apply flag or equivalent protocol field.
   - Refuse ambiguous write requests.
   - Preserve failure atomicity where practical.

5. Add tests and documentation.
   - Cover dry-run, rejected apply, successful apply, schema failure, and audit output.
   - Document safe MCP write usage and rollback expectations.
