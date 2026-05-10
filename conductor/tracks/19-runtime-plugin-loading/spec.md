# Runtime Plugin Loading Spec

## Goal

Move Sourceright from static plugin manifests to safe runtime plugin discovery and loading.

## Scope

- Discover plugin registry manifests at runtime.
- Validate manifests before capabilities are exposed.
- Make plugin capabilities discoverable through CLI and MCP surfaces.
- Gate execution through trust, provenance, and policy decisions.
- Preserve deterministic behavior for default tests and local fixture workflows.

## Outputs

- Runtime plugin registry loader.
- Manifest validation errors and diagnostics.
- Capability discovery output.
- Trust/provenance policy checks.
- Documentation for local plugin installation and loading.

## Boundaries

Runtime plugin loading must not allow untrusted code execution by default. Provider and citation-manager plugins must remain governed by explicit capability and trust policy.

Plugin output must respect existing data boundaries: provider evidence belongs in sidecars, not silent canonical CSL overwrites.
