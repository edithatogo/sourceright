# MCP

Sourceright's MCP surface is read-first and local-stdio oriented. The checked-in
manifests are the source of truth:

- `mcp/tools.v1.json`
- `mcp/resources.v1.json`
- `mcp/prompts.v1.json`

The server advertises the same surface in `src/mcp.rs`. Read-only tools are
safe to call over stdio without enabling any write path. The write-capable tools
default to dry-run planning and only mutate when `apply: true` is supplied.

Distribution metadata is tracked separately from the runtime manifests:

- `server.json` describes the MCP Registry package, including the stdio OCI
  image target.
- `glama.json` carries the Glama MCP directory metadata and maintainer
  ownership.
- `Dockerfile` and the release workflow provide the packaged MCP server image.

## Tools

Read-only tools:

- `mcp.status` - inspect server readiness and the read-only MCP surface.
- `references.validate_csl` - validate canonical CSL JSON and return
  deterministic diagnostics.
- `references.report` - generate the reference integrity report from a local
  workspace.
- `references.review_queue` - return the derived review queue as JSONL.
- `references.citations` - reconcile in-text citations against the workspace
  references.
- `journal.screen_submission` - generate a journal screening report from
  workspace references.
- `legal.analyze_citations` - audit legal citations into the separate legal
  model with jurisdiction/provider hints and attorney-review flags; it does not
  provide legal advice.
- `provenance.analyze_claim_sources` - build a claim/source provenance report
  from document text.
- `references.policy` - evaluate deterministic style and recency policy checks.
- `exports.preview` - preview export artifacts without writing files.
- `plugins.list` - discover validated plugin manifests and runtime execution
  gates.

Write-capable tools with dry-run/apply semantics:

- `workspace.init` - create the local workspace structure.
- `review.import_decisions` - import review decisions.
- `exports.write` - write export files.

The explicit contract is:

- omit `apply` or set `apply: false` to request a plan only;
- set `apply: true` only when a mutation is intentionally requested;
- treat the returned `changes` array as the dry-run plan;
- expect `audit_log` only when the write actually applies.

## Resources

The server exposes these read-only resources:

- `sourceright://reports/reference-integrity`
- `sourceright://reports/citation-reconciliation`
- `sourceright://workspaces/local/review-queue`
- `sourceright://reports/journal-screening`
- `sourceright://reports/legal-citations`
- `sourceright://reports/claim-source-provenance`
- `sourceright://reports/policy`
- `sourceright://plugins/registry`

## Prompts

The prompt surface is read-only and explanatory:

- `manual_reference_review` - guide manual review of queued references using
  CSL and sidecar evidence.
- `citation_integrity_explanation` - explain reference report issues without
  claiming author intent.
- `provider_conflict_explanation` - explain provider/canonical conflicts and
  preserve the no-silent-overwrite rule.
- `legal_citation_review` - review separate legal citation records,
  jurisdiction/provider issues, and attorney-review flags without giving legal
  advice.
- `claim_source_provenance_review` - review claim/source linkage without
  claim-truth scoring.

## Local stdio use

For a local MCP client, the minimal stdio launcher is:

```json
{
  "mcpServers": {
    "sourceright": {
      "command": "sourceright",
      "args": ["mcp"]
    }
  }
}
```

### Generic MCP clients

Generic clients should use the same local stdio launcher and treat the checked-in
MCP manifests as the discovery contract:

```json
{
  "mcpServers": {
    "sourceright": {
      "command": "sourceright",
      "args": ["mcp"]
    }
  }
}
```

### Claude Desktop

Claude Desktop uses client configuration over the local stdio contract. This is
not a Claude plugin package:

```json
{
  "mcpServers": {
    "sourceright": {
      "command": "sourceright",
      "args": ["mcp"]
    }
  }
}
```

### Codex

Codex-oriented workflows should launch the same local stdio MCP server from the
repository or installed CLI path. This is Codex MCP configuration, not a Codex
plugin package:

```toml
[mcp_servers.sourceright]
command = "sourceright"
args = ["mcp"]
```

### GitHub Copilot

GitHub Copilot support is prepared as a repository coding-agent workflow, not as
an MCP client package or Copilot extension. VS Code MCP settings can expose the
local server to Copilot agent mode when MCP servers are enabled:

```json
{
  "servers": {
    "sourceright": {
      "type": "stdio",
      "command": "sourceright",
      "args": ["mcp"]
    }
  }
}
```

The repo-local Copilot preparation files remain separate from that MCP client
configuration:

```text
.github/copilot-instructions.md
.github/workflows/copilot-setup-steps.yml
.github/ISSUE_TEMPLATE/copilot_security_remediation.yml
```

The server speaks stdio and starts from the current workspace root unless the
client wrapper overrides the working directory. A client should initialize the
server, then call `tools/list`, `resources/list`, and `prompts/list` before any
`tools/call`.

## Client Configuration Examples

Concrete host configuration examples live under `examples/mcp-clients/`:

- `claude-desktop.json` uses the Claude Desktop `mcpServers` local stdio shape.
- `codex-config.toml` uses a Codex `[mcp_servers.sourceright]` stanza.
- `vscode-mcp.json` uses the VS Code `servers` shape used by GitHub Copilot
  agent mode when MCP servers are enabled.
- `generic-mcp-client.json` keeps the portable `mcpServers` launcher for other
  local MCP clients.
- `host-manifest.json` records the Track 65 host status and claim boundary.
- `smoke-requests.jsonl` provides a manual stdio smoke sequence.

These files reference the existing `sourceright mcp` stdio server. They are not
host plugin packages, hosted HTTP endpoints, or marketplace acceptance evidence.

Before adding a host config, verify the binary and manifests:

```bash
sourceright --version
sourceright mcp status
sourceright mcp tools --json
sourceright mcp resources --json
sourceright mcp prompts --json
```

For a manual stdio smoke, start `sourceright mcp`, then send the JSON-RPC lines
from `examples/mcp-clients/smoke-requests.jsonl`. The dry-run write call should
return `apply_requested: false`, `applied: false`, and no `audit_log`.

## Acceptance Evidence

Track 65 accepts evidence only at the level it actually proves:

| Evidence | Proves | Does not prove |
| --- | --- | --- |
| `sourceright mcp status --json` | The installed binary exposes MCP status metadata. | Client-specific discovery or marketplace acceptance. |
| `sourceright mcp tools --json`, `resources --json`, and `prompts --json` | The advertised contract is inspectable before a client call. | That a host has enabled or approved Sourceright. |
| `initialize`, `tools/list`, `resources/list`, and `prompts/list` transcript smoke | Protocol-level local stdio compatibility. | Hosted HTTP support or remote execution. |
| Dry-run `workspace.init` call with `apply` omitted or `false` | Write-capable tools return a plan without mutating files. | That applied writes are safe without user review. |
| Official MCP Registry listing for `0.1.20` | Registry metadata and OCI package binding for that version. | Glama, Smithery, Claude, Codex, Copilot, or generic-client acceptance. |
| `.github/copilot-instructions.md` and setup workflow | Repository preparation for Copilot coding-agent work. | Copilot entitlement, extension packaging, or MCP client support. |

## Client Packaging Status

| Client or directory | Current status | Claim boundary |
| --- | --- | --- |
| Official MCP Registry | Accepted for `0.1.20` through `server.json` and the OCI image target. | Registry acceptance does not prove every downstream client configuration. |
| Glama | Prepared through `glama.json`; no accepted listing is recorded. | Do not claim Glama availability until the listing is verified. |
| Smithery | Prepared through the MCPB/local stdio package path. | Do not claim Smithery availability until a concrete bundle/listing is verified. |
| Claude Desktop | Prepared through local stdio configuration examples in `examples/mcp-clients/claude-desktop.json`. | This is client configuration, not a Claude plugin package. |
| Codex | Prepared through local stdio configuration examples in `examples/mcp-clients/codex-config.toml` and `codex-mcp.json`. | This is repo-agent or MCP configuration, not a Codex plugin package. |
| GitHub Copilot / VS Code | Repository coding-agent prep and local VS Code MCP configuration example in `examples/mcp-clients/vscode-mcp.json`. | This is not a Copilot extension, Copilot marketplace package, or proof of Copilot entitlement. |
| Generic MCP clients | Portable local stdio configuration example in `examples/mcp-clients/generic-mcp-client.json`. | Client syntax can vary; this is a launcher example, not a universal install package. |

## Transcript Snippets

Initialize and inspect the surface:

```text
Client -> Server
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"example","version":"0.1.20"}}}

Server -> Client
{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2025-11-25","serverInfo":{"name":"sourceright","version":"0.1.20"},"capabilities":{"tools":{},"resources":{},"prompts":{}},"instructions":"Read-only local reference verification server"}}

Client -> Server
{"jsonrpc":"2.0","id":2,"method":"tools/list"}
```

Dry-run versus apply:

```text
Client -> Server
{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"workspace.init","arguments":{"workspace":".sourceright"}}}

Server -> Client
{"jsonrpc":"2.0","id":3,"result":{"content":[{"type":"text","text":"{\"schema_version\":\"sourceright.mcp_write_plan.v1\",\"tool\":\"workspace.init\",\"apply_requested\":false,\"applied\":false,\"workspace\":\".sourceright\",\"changes\":[...]}"}],"isError":false}}

Client -> Server
{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"workspace.init","arguments":{"workspace":".sourceright","apply":true}}}
```

## Threat Model

The intended trust boundary is local stdio, local files, and explicit client
requests only.

- Read-only tools may read local repository files and derived workspace state.
- Write-capable tools must not mutate unless `apply: true` is present.
- The workspace path should be treated as sensitive if it contains unpublished
  content or mixed-trust artifacts.
- Prompts describe review workflows; they do not assert truth or provenance on
  their own.
- Clients that automate the server should treat unexpected write requests as
  user-visible approval points.
- After any applied write, inspect the returned `audit_log` and the affected
  workspace files.

## Legal Citation Connector Boundary

For legal workflows, Sourceright should be described as a citation audit and
enrichment connector. The MCP surface can extract candidate citations and
return jurisdiction/provider evidence, confidence, conflicts, and review
issues. It must not answer legal questions, predict outcomes, draft final legal
work product, or claim legal compliance. Missing or stale provider evidence is
a review issue, not a basis for a legal conclusion.

The hardening goal is simple: read-only calls stay inspectable, dry-run calls
stay non-destructive, and applied writes are explicit enough to audit later.
