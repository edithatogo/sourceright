# Runtime Plugin Loading Plan

1. Audit static plugin manifests.
   - Identify current registry schema and capability fields.
   - Identify which manifests can become runtime-discoverable first.

2. Add manifest discovery and validation.
   - Define search paths and workspace-local registry behavior.
   - Validate manifest structure, version, capabilities, and provenance fields.
   - Report invalid manifests without loading them.

3. Add capability discovery.
   - Expose loaded plugin capabilities through CLI status/manifest commands.
   - Expose read-only discovery through MCP once runtime support is available.
   - Keep tests fixture-backed and deterministic.

4. Add trust and execution gates.
   - Require explicit trust policy before plugin execution.
   - Separate discovery from execution.
   - Record provenance for loaded plugin metadata.

5. Document and test.
   - Add examples for valid and invalid manifests.
   - Cover discovery, validation failure, trust refusal, and capability reporting.
