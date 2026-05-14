# MCP Registry Release Binding — Validation Checklist

## server.json
- [ ] Protocol version matches `server.json` declared version
- [ ] All tool names match `src/mcp.rs` tool registrations
- [ ] All resource URIs match `src/mcp.rs` resource handlers
- [ ] All prompt names match `src/mcp.rs` prompt definitions
- [ ] `server.json` schema validates against MCP registry expectations
- [ ] `server.json` lists correct capabilities (tools, resources, prompts)

## glama.json
- [ ] Metadata matches `server.json` (name, version, description)
- [ ] `glama.json` includes valid MCP registry metadata
- [ ] Category and tags reflect current feature scope

## OCI Image Binding
- [ ] `Dockerfile` exists and builds successfully
- [ ] Image is tagged with release version and `latest`
- [ ] Image is published to GHCR
- [ ] OCI labels match package metadata (version, description, licenses)

## Release Workflow
- [ ] `.github/workflows/publish-mcp-registry.yml` exists
- [ ] Workflow triggers on release tags
- [ ] Workflow produces valid registry submission
- [ ] GHA attestations or provenance metadata are generated

## Registry Acceptance
- [ ] Official MCP Registry submission is prepared
- [ ] Artifact checksums are recorded in release notes
- [ ] Registry listing URLs are included in docs/src/release-status.md
- [ ] Evidence date is recorded for each registry submission

## Glama Separation
- [ ] Glama metadata is validated independently of MCP Registry
- [ ] Glama listing URL is recorded in docs/src/release-status.md
