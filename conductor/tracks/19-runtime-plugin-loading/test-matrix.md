# Runtime Plugin Loading Test Matrix

| Scenario | Expected result |
| --- | --- |
| Valid manifest | Runtime loader validates the manifest and reports its capabilities. |
| Invalid manifest | Loader rejects the manifest with actionable diagnostics. |
| Missing trust policy | Capability execution is refused even if discovery succeeds. |
| Capability discovery | CLI and MCP discovery surfaces show validated plugin capabilities. |
| Provider plugin | Provider capability is discoverable without making live network calls by default. |
| Citation-manager plugin | Sync capability is discoverable without modifying external libraries by default. |
| Provenance fields | Loaded manifests preserve source, version, and trust metadata. |
| Deterministic tests | Default tests use fixture manifests and do not require installed third-party plugins. |
