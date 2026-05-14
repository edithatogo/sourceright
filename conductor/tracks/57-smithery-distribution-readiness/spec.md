# Smithery Distribution Readiness Spec

## Goal

Add a real Smithery publication path for Sourceright MCP rather than treating
Smithery as a vague future registry.

## Contract

Smithery support is complete only when one of these paths is validated:

- Streamable HTTP MCP endpoint with Smithery URL publishing requirements met.
- MCPB/local package path for stdio/local execution with install and validation
  evidence.

Until then, docs must say Smithery is prepared or deferred, not accepted.

## Parallelization

- Subagent A: Smithery package/runtime requirements.
- Subagent B: Sourceright MCP runtime gaps.
- Subagent C: metadata/docs/release evidence.
