# MCP Server Runtime Plan

1. Audit the existing CLI/MCP contract.
   - Confirm current tool, resource, prompt, status, and manifest surfaces.
   - Identify the runtime transport crate or implementation pattern to use.

2. Add the local server command.
   - Implement `sourceright mcp` startup.
   - Keep JSON status and manifest behavior compatible with existing commands.
   - Return clear startup errors for invalid workspace state.

3. Register read-only contracts.
   - Expose existing read-only tools, resources, and prompts.
   - Preserve deterministic workspace reads and fixture-backed behavior.
   - Do not add write tools in this track.

4. Add documentation and examples.
   - Document server startup.
   - Document supported clients and contract discovery.
   - Document current read-only limitations and the deferred write-tool track.

5. Add validation.
   - Add runtime startup tests.
   - Add contract discovery tests.
   - Run the standard Cargo gates.
