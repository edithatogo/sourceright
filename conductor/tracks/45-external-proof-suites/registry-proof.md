# Registry Proof — External Proof Suite

## Purpose

Document how to verify registry bindings for MCP distribution readiness.
This proof covers `server.json` validation, `glama.json` metadata, `Dockerfile`
MCP labels, and the release/publish workflow bindings that connect these
artifacts.

## Prerequisites

- Repository root containing `server.json`, `glama.json`, `Dockerfile`
- `.github/workflows/publish-mcp-registry.yml` and `release.yml`
- `jq` for JSON parsing (optional)

## Proof Sections

### 1. `server.json` Validates Against MCP Server Schema

**Purpose:** Confirm `server.json` is valid JSON with required fields.

**Command:**
```text
cat server.json | jq '{$schema, name, version, repository}'
```

**Expected output:**
```json
{
  "$schema": "https://static.modelcontextprotocol.io/schemas/2025-12-11/server.schema.json",
  "name": "io.github.edithatogo/sourceright",
  "version": "0.1.20",
  "repository": { "url": "https://github.com/edithatogo/sourceright" }
}
```

**Key assertions (verified by `mcp_distribution_checks.rs`):**
- `$schema` matches MCP server schema URL
- `name` is `io.github.edithatogo/sourceright`
- `version` matches `Cargo.toml`
- `packages` includes an OCI entry

**Exit code:** `0`

---

### 2. `server.json` OCI Distribution Target

**Command:**
```text
cat server.json | jq '.packages[] | select(.registryType == "oci") | {identifier, transport}'
```

**Expected output:**
```json
{
  "identifier": "ghcr.io/edithatogo/sourceright-mcp:0.1.20",
  "transport": { "type": "stdio" }
}
```

**Exit code:** `0`

---

### 3. `glama.json` Metadata

**Command:**
```text
cat glama.json | jq '{$schema, maintainers}'
```

**Expected output:**
```json
{
  "$schema": "https://glama.ai/mcp/schemas/server.json",
  "maintainers": ["edithatogo"]
}
```

**Exit code:** `0`

---

### 4. `Dockerfile` MCP Registry Labels

**Command:**
```text
head -15 Dockerfile
```

**Expected output includes:**
```
LABEL org.opencontainers.image.source="https://github.com/edithatogo/sourceright"
LABEL org.opencontainers.image.description="Sourceright MCP stdio server"
LABEL org.opencontainers.image.version="0.1.20"
LABEL io.modelcontextprotocol.server.name="io.github.edithatogo/sourceright"
```

**Exit code:** `0`

---

### 5. Dockerfile MCP Server Name Label (CI)

**Command:**
```text
cargo test --test mcp_distribution_checks dockerfile_has_mcp_registry_ownership_labels
```

**Key assertions:**
- Dockerfile has `io.modelcontextprotocol.server.name` matching `server.json`
- Dockerfile has `org.opencontainers.image.version` matching `Cargo.toml`
- Dockerfile has `org.opencontainers.image.source` URL

**Exit code:** `0`

---

### 6. Release Workflow Version Labels

**Command:**
```text
cargo test --test mcp_distribution_checks release_workflow_declares_oci_version_label
```

**Exit code:** `0`

---

### 7. Publish Workflow Registry Binding

**Command:**
```text
cargo test --test mcp_distribution_checks publish_workflow_binds_registry_submission_to_release_image
```

**Exit code:** `0`

---

### 8. Release Status Records MCP Registry

**Command:**
```text
cargo test --test mcp_distribution_checks release_status_records_mcp_registry_acceptance_and_ghcr_boundary
```

**Exit code:** `0`


## Transcript Template

```bash
#!/usr/bin/env bash
set -euo pipefail

echo "=== Registry Proof Transcript ==="
cd "${1:-.}"

echo "1. Validate server.json"
cat server.json | jq -e '.name == "io.github.edithatogo/sourceright"' > /dev/null
cat server.json | jq -e '.packages[] | select(.registryType == "oci") | .identifier' > /dev/null
echo "  server.json: PASS"

echo "2. Validate glama.json"
cat glama.json | jq -e '.maintainers | index("edithatogo") >= 0' > /dev/null
echo "  glama.json: PASS"

echo "3. Validate Dockerfile MCP labels"
grep -q 'io.modelcontextprotocol.server.name' Dockerfile || { echo "FAIL"; exit 1; }
grep -q 'org.opencontainers.image.source' Dockerfile || { echo "FAIL"; exit 1; }
echo "  Dockerfile labels: PASS"

echo "4. Run MCP distribution checks"
cargo test --test mcp_distribution_checks 2>&1 | grep -q "test result: ok"
echo "  CI distribution checks: PASS"

echo "=== ALL PASSED ==="
```

## Overclaim Guard

Must not claim:

- **"Published to MCP Registry"** — submission is gated on workflow_run
- **"GHCR image verified"** — release-status.md lists GHCR as "prepared"
- **"All registries published"** — Smithery tracked in Track 57

Only claim: **"MCP distribution artifacts (server.json, glama.json,
Dockerfile, workflows) are CI-validated and version-aligned. The MCP Registry
submission pipeline is gated on release completion."**

## Proof Family Status

| Surface | Status | Evidence |
|---------|--------|----------|
| `server.json` schema/version | ✅ Proven | `mcp_distribution_checks.rs` |
| `server.json` OCI target | ✅ Proven | `mcp_distribution_checks.rs` |
| `glama.json` metadata | ✅ Proven | `mcp_distribution_checks.rs` |
| Dockerfile OCI labels | ✅ Proven | `mcp_distribution_checks.rs` |
| Release workflow version label | ✅ Proven | `mcp_distribution_checks.rs` |
| Publish workflow gating | ✅ Proven | `mcp_distribution_checks.rs` |
| Release status boundary | ✅ Proven | `mcp_distribution_checks.rs` |
| Smithery MCPB packaging | 🔄 Other track | Track 57 |
| GHCR publication verification | ⏸️ Pre-release | Manual verification |

