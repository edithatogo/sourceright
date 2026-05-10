# MCP Distribution Spec

## Goal

Make Sourceright's stdio MCP server ready for public MCP registry and marketplace distribution.

## Scope

- Add official MCP registry metadata for a public package target.
- Add an OCI container path for registry and sandbox scanners.
- Document Smithery and Glama publication constraints.
- Keep the local stdio command as the primary install surface.

## Outputs

- `server.json` metadata for MCP registry publication.
- OCI packaging metadata and image labels.
- Manual MCP registry publish workflow.
- MCP distribution documentation.

## Boundaries

This track must not publish to MCP registries automatically from an unreviewed tree. Registry submission requires a public package artifact and maintainer-authenticated workflow.
