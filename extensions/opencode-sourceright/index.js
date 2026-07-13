/** OpenCode plugin: documents Sourceright MCP wiring (stdio). */
export default async function sourcerightPlugin({ client }) {
  client.on("session.created", () => {
    // No-op hook; MCP servers are configured via opencode.json `mcp` block.
  });
}
