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
- `legal.analyze_citations` - extract legal citations into the separate legal
  model.
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
- `legal_citation_review` - review separate legal citation records and
  jurisdiction/provider issues.
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

The server speaks stdio and starts from the current workspace root unless the
client wrapper overrides the working directory. A client should initialize the
server, then call `tools/list`, `resources/list`, and `prompts/list` before any
`tools/call`.

## Transcript Snippets

Initialize and inspect the surface:

```text
Client -> Server
{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"example","version":"0.1.0"}}}

Server -> Client
{"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2025-11-25","serverInfo":{"name":"sourceright","version":"0.1.0"},"capabilities":{"tools":{},"resources":{},"prompts":{}},"instructions":"Read-only local reference verification server"}}

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

The hardening goal is simple: read-only calls stay inspectable, dry-run calls
stay non-destructive, and applied writes are explicit enough to audit later.
