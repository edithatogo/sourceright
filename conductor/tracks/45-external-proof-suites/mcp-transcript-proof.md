# MCP Stdio Transcript Proof — External Proof Suite

## Purpose

This document defines how to verify the MCP (Model Context Protocol) stdio
endpoint. It provides a **human-readable proof transcript** and expected JSON
responses for the MCP Stdio Transcript Smoke proof family.

## Prerequisites

- A built `sourceright` binary
- The repository root containing `plugins/` manifests for discovery tests
- A JSON-aware terminal or `jq` for formatting

## Proof Sections

### 1. MCP Server Status (Readiness Check)

**Command:**
```text
sourceright mcp status
```

**Expected text output:**
```
Sourceright MCP status
server_mode: stdio
transport: stdio
server_started: false
available_tools: 14
available_resources: 8
available_prompts: 5
implemented_read_only_surfaces:
  - sourceright validate-csl <references.csl.json>
  - sourceright report --json [.sourceright-directory]
  - sourceright report --mcp-resource [.sourceright-directory]
  - sourceright conflicts [.sourceright-directory]
  - sourceright citations <manuscript.txt> [.sourceright-directory]
  - sourceright review queue|partitions|import-decisions
  - sourceright journal-screen [.sourceright-directory]
  - sourceright legal <legal-text.txt>
  - sourceright provenance <document-text.txt>
  - sourceright policy <references.csl.json>
  - sourceright plugins [validate] [--json]
  - sourceright export --all [.sourceright-directory]
resource_uris:
  - sourceright://reports/reference-integrity
  - sourceright://reports/citation-reconciliation
  - sourceright://workspaces/local/review-queue
  - sourceright://reports/journal-screening
  - sourceright://reports/legal-citations
  - sourceright://reports/claim-source-provenance
  - sourceright://reports/policy
  - sourceright://plugins/registry
message: MCP server mode is implemented; run `sourceright mcp` to start the stdio server.
```

**Command (JSON):**
```text
sourceright mcp status --json
```

**Expected JSON output:**
```json
{
  "server_mode": "stdio",
  "transport": "stdio",
  "server_started": false,
  "available_tools": 14,
  "available_resources": 8,
  "available_prompts": 5,
  "implemented_read_only_surfaces": [...],
  "resource_uris": [...],
  "message": "MCP server mode is implemented; run `sourceright mcp` to start the stdio server."
}
```

**Exit code:** `0`

---

### 2. MCP Tool Discovery

**Command:**
```text
sourceright mcp tools --json
```

**Expected JSON output (abridged):**
```json
{
  "schema_version": "sourceright.mcp_tools.v1",
  "server_status": "local_stdio_server",
  "tools": [
    {"name": "mcp.status", "read_only": true},
    {"name": "references.validate_csl", "read_only": true},
    {"name": "references.report", "read_only": true},
    {"name": "references.review_queue", "read_only": true},
    {"name": "references.citations", "read_only": true},
    {"name": "journal.screen_submission", "read_only": true},
    {"name": "legal.analyze_citations", "read_only": true},
    {"name": "provenance.analyze_claim_sources", "read_only": true},
    {"name": "references.policy", "read_only": true},
    {"name": "exports.preview", "read_only": true},
    {"name": "plugins.list", "read_only": true},
    {"name": "workspace.init", "read_only": false},
    {"name": "review.import_decisions", "read_only": false},
    {"name": "exports.write", "read_only": false}
  ]
}
```

**Key assertions:**
- `schema_version` is `sourceright.mcp_tools.v1`
- Contains exactly 14 tools (11 read-only + 3 write-plan)
- `plugins.list` is present with empty input schema

**Exit code:** `0`

---

### 3. MCP Resource Discovery

**Command:**
```text
sourceright mcp resources --json
```

**Expected JSON output:**
```json
{
  "schema_version": "sourceright.mcp_resources.v1",
  "resources": [
    {"uri": "sourceright://reports/reference-integrity", "mime_type": "application/json"},
    {"uri": "sourceright://reports/citation-reconciliation", "mime_type": "text/markdown"},
    {"uri": "sourceright://workspaces/local/review-queue", "mime_type": "application/jsonl"},
    {"uri": "sourceright://reports/journal-screening", "mime_type": "application/json"},
    {"uri": "sourceright://reports/legal-citations", "mime_type": "application/json"},
    {"uri": "sourceright://reports/claim-source-provenance", "mime_type": "application/json"},
    {"uri": "sourceright://reports/policy", "mime_type": "application/json"},
    {"uri": "sourceright://plugins/registry", "mime_type": "application/json"}
  ]
}
```

**Key assertions:**
- `schema_version` is `sourceright.mcp_resources.v1`
- Contains exactly 8 resources
- All URIs use the `sourceright://` scheme

**Exit code:** `0`

---

### 4. MCP Prompt Discovery

**Command:**
```text
sourceright mcp prompts --json
```

**Expected JSON output:**
```json
{
  "schema_version": "sourceright.mcp_prompts.v1",
  "prompts": [
    {"name": "manual_reference_review", "purpose": "Guide manual review of queued references using CSL and sidecar evidence."},
    {"name": "citation_integrity_explanation", "purpose": "Explain reference report issues without claiming author intent."},
    {"name": "provider_conflict_explanation", "purpose": "Explain provider/canonical conflicts and the no-silent-overwrite rule."},
    {"name": "legal_citation_review", "purpose": "Review separate legal citation records and jurisdiction/provider issues."},
    {"name": "claim_source_provenance_review", "purpose": "Review claim/source linkage without claim-truth scoring."}
  ]
}
```

**Key assertions:**
- `schema_version` is `sourceright.mcp_prompts.v1`
- Contains exactly 5 prompts
- No prompt claims to assess truth, author intent, or legal validity

**Exit code:** `0`

---

### 5. MCP Server Startup (Stdio)

**Command:**
```text
sourceright mcp
```

Starts the MCP server in stdio mode. It handles JSON-RPC messages on stdin:

| Method | Direction | Description |
|--------|-----------|-------------|
| `initialize` | Client → Server | Protocol handshake; returns server capabilities |
| `tools/list` | Client → Server | Lists available tools |
| `resources/list` | Client → Server | Lists available resources |
| `prompts/list` | Client → Server | Lists available prompts |
| `tools/call` | Client → Server | Invokes a named tool with input arguments |

Runs until stdin is closed or SIGTERM. Stdio transport only.

## Transcript Template

```bash
#!/usr/bin/env bash
set -euo pipefail

echo "=== MCP Stdio Transcript Smoke ==="

echo "1. Status (text)"
sourceright mcp status | grep -q "server_mode: stdio"

echo "2. Status (JSON)"
sourceright mcp status --json | grep -q '"server_mode":"stdio"'

echo "3. Tools"
sourceright mcp tools --json > /tmp/mcp-tools.json
grep -q '"schema_version":"sourceright.mcp_tools.v1"' /tmp/mcp-tools.json
grep -q '"references.report"' /tmp/mcp-tools.json

echo "4. Resources"
sourceright mcp resources --json > /tmp/mcp-resources.json
grep -q '"schema_version":"sourceright.mcp_resources.v1"' /tmp/mcp-resources.json
grep -q '"sourceright://plugins/registry"' /tmp/mcp-resources.json

echo "5. Prompts"
sourceright mcp prompts --json > /tmp/mcp-prompts.json
grep -q '"schema_version":"sourceright.mcp_prompts.v1"' /tmp/mcp-prompts.json
grep -q '"provider_conflict_explanation"' /tmp/mcp-prompts.json

echo "=== ALL PASSED ==="
```

## Proof Family Status

| Surface | CI Coverage | Transcript Evidence | Status |
|---------|-------------|-------------------|--------|
| `mcp status` | ✅ `cli_end_to_end.rs` | ✅ This document | ✅ Proven |
| `mcp status --json` | ✅ `cli_end_to_end.rs` | ✅ This document | ✅ Proven |
| `mcp tools --json` | ✅ `cli_end_to_end.rs` | ✅ This document | ✅ Proven |
| `mcp resources --json` | ✅ `cli_end_to_end.rs` | ✅ This document | ✅ Proven |
| `mcp prompts --json` | ✅ `cli_end_to_end.rs` | ✅ This document | ✅ Proven |
| `mcp` (server startup) | 🔶 Manual only | 🔶 Manual transcript | 🔄 Opt-in |

All discovery surfaces pass in CI. Server startup is opt-in and requires
a real MCP client to exercise JSON-RPC methods.
