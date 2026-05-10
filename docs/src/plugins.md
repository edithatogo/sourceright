# Runtime Plugin Loading

Sourceright now discovers the repository plugin registry at runtime through
`sourceright plugins` and the MCP `plugins.list` tool.

The runtime loader reads `plugins/registry.toml`, validates the listed manifest
files under `plugins/manifests/`, and reports validated capability metadata.
Discovery is deterministic and fixture-backed; it does not execute plugin code.

## CLI

- `sourceright plugins` prints a human-readable discovery summary.
- `sourceright plugins --json` prints compact
  `sourceright.plugin_registry_report.v1` JSON.
- `sourceright plugins validate` exits non-zero when any manifest fails
  validation.

## MCP

- `plugins.list` returns the same validated discovery report as JSON.
- `sourceright://plugins/registry` exposes the same report as a resource.

## Boundaries

Runtime loading is discovery-only for now. Plugins remain gated by explicit
trust policy, provenance checks, and the repo's existing no-silent-overwrite
rules. Provider evidence still belongs in sidecars, not canonical CSL JSON.
