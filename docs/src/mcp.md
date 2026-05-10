# MCP Plan

The Rust CLI uses `sourceright mcp` to start the local MCP server.

The public local install target is the same binary used by the CLI:

```text
sourceright mcp
```

For registry and marketplace distribution, Sourceright keeps stdio as the
first transport. The repository includes `server.json` metadata for the
official MCP Registry using the GitHub namespace
`io.github.edithatogo/sourceright` and an OCI package target for scanners that
need a reproducible build artifact.

The inspection paths remain available for scripts and adapters:

- `sourceright mcp status` prints status without changing server state.
- `sourceright mcp --status` is an alias for `status`.
- `sourceright mcp status --json` and `sourceright mcp --json` print machine-readable readiness status.
- `sourceright mcp tools --json`, `sourceright mcp resources --json`, and `sourceright mcp prompts --json` remain read-only and print compact copies of the checked-in MCP manifests.
- The MCP server also exposes validated plugin discovery through `plugins.list`
  and `sourceright://plugins/registry`.

Status output is intentionally conservative and is meant for inspection rather than mutation. The JSON status envelope includes `server_mode`, `transport`, `server_started`,
tool/resource/prompt counts, implemented read-only surfaces, resource URIs, and
a readiness message. It is intended for wrappers and agents that should not
parse the human-readable status text.

The manifest commands are intentionally inspection-only. They expose `mcp/tools.v1.json`,
`mcp/resources.v1.json`, and `mcp/prompts.v1.json` through the CLI so early
MCP adapters can bind to the declared contracts without requiring server
transport or direct repository file access.

The MCP server exposes the same Rust core as the CLI. The first contracts now available for reuse are:

- `validate-csl --json`, which returns a deterministic local-file validation envelope with `ok`, `path`, and `diagnostics` fields and uses exit code `1` for validation findings.
- `report --json`, which returns `sourceright.reference_report.v1`.
- `report --mcp-resource`, which returns the report resource envelope at `sourceright://reports/reference-integrity`.
- `export --preview --format <format>` and `export --preview --all`, which return `sourceright.export_manifest.v1` without writing export files.
- `workspace.init`, `review.import_decisions`, and `exports.write`, which default to dry-run change plans and only mutate when `apply: true` is provided.
- `plugins.list`, which returns the validated repository plugin catalog and its execution gates.

Planned tools:

- `references.extract`
- `references.normalize`
- `references.verify`
- `references.review_queue`
- `references.export`
- `references.report`
- `pipeline.run`

Planned resources:

- Canonical CSL JSON.
- Validation diagnostics.
- Verification sidecar.
- Review queue.
- Citation reconciliation report.
- Reference integrity report.
- Export manifest.

Planned prompts:

- Manual reference review.
- CSL validation explanation.
- Conflict explanation.
- Provider comparison.

The first useful MCP increment should remain local-file based and auditable: validate CSL JSON, generate reference integrity reports, report diagnostics, expose the derived review queue resource, and allow explicit dry-run or apply flows for workspace init, review decision imports, and export writes. Additional write-capable tools should continue to wait until their CLI pipeline counterparts have stable input and output contracts.

Smithery URL publishing requires a Streamable HTTP endpoint, so the current
stdio server is better suited to local MCPB-style distribution there until an
HTTP transport is added. Glama and similar indexes can inspect the open-source
repository and OCI packaging path, but default operation must remain local and
credential-free.

The public MCP package metadata lives in `server.json`, and the OCI image is
tagged to match that registry declaration. The Docker image labels are part of
the distribution contract, not decorative metadata, because they let public
scanners tie the container back to the source repository and server name.

## Distribution metadata status

The following files are the minimum metadata surface:

- `server.json`: Official MCP Registry metadata using
  `registryType: \"oci\"` with `ghcr.io/edithatogo/sourceright-mcp:<version>`.
- `Dockerfile`: OCI image label `io.modelcontextprotocol.server.name` and repository
  provenance labels.
- `glama.json`: Glama ownership metadata (`maintainers` + schema pointer).

### Smithery read-side readiness

- Smithery distribution paths are currently constrained by transport:
  - URL publishing requires Streamable HTTP.
  - Without HTTP transport, Smithery should use MCPB/local distribution flow.
- `/.well-known/mcp/server-card.json` can be used later as a fallback metadata
  endpoint for scanning where dynamic auth or session discovery is not available.

### Glama read-side readiness

- Add `glama.json` with at least:
  - `$schema: https://glama.ai/mcp/schemas/server.json`
  - `maintainers: ["edithatogo"]`
- Keep repository license and source scanability in place; Glama checks for both
  before install readiness.

### OCI readiness checks

- `server.json` `version` must match crate version.
- OCI image identifier in `server.json` must use the official GitHub Container Registry
  package path.
- Dockerfile must retain `io.modelcontextprotocol.server.name` and repository source
  labels for registry ownership verification.
