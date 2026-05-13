# 07 — MCP read-only hardening

Harden and document read-only MCP behavior.

Tasks:
1. Inspect current `mcp/`, `src/mcp.rs`, `server.json`, README/docs.
2. Add exact read-only resources/tool list.
3. Add sample configs if appropriate:
   - Codex;
   - Claude Desktop;
   - Cursor.
4. Add transcript examples.
5. Add MCP threat-model notes.
6. Explicitly defer write tools until audit/dry-run/apply semantics are stable.
7. Run tests.

Do not add write-capable MCP tools in this slice.
