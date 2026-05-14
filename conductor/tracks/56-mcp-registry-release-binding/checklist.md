# MCP Registry Release Binding — Validation Checklist

## server.json
- [x] `server.json` schema URL is recorded and guarded by `tests/mcp_distribution_checks.rs`.
- [x] `server.json` package version matches `Cargo.toml`.
- [x] `server.json` name and repository URL match the GitHub namespace.
- [x] `server.json` OCI package uses stdio transport.
- [x] `server.json` OCI image target matches `ghcr.io/edithatogo/sourceright-mcp:<Cargo version>`.
- [x] Tool/resource/prompt runtime contracts remain owned by the MCP runtime tests; this track only binds release metadata.

## glama.json
- [x] `glama.json` uses the Glama MCP schema.
- [x] `glama.json` records maintainer handle `edithatogo`.
- [x] Glama remains tracked separately from official MCP Registry acceptance in release-status docs.

## OCI Image Binding
- [x] `Dockerfile` exists and defines `sourceright mcp` as the default command.
- [x] Release workflow tags the image with the release version and `latest`.
- [x] Release workflow emits an OCI version label derived from the tag.
- [x] `Dockerfile` OCI source, version, license, and MCP server labels match repository metadata.
- [x] GHCR direct package visibility remains prepared, not accepted, until separately verified.

## Release Workflow
- [x] `.github/workflows/publish-mcp-registry.yml` exists.
- [x] MCP registry workflow follows successful `Release` workflow completion.
- [x] Workflow validates `server.json` before publishing.
- [x] Workflow waits for the release-versioned GHCR image before publishing.
- [x] Workflow authenticates to `mcp-publisher` through GitHub OIDC.
- [x] Release workflow generates binary, crate, and image attestations.

## Registry Acceptance
- [x] Official MCP Registry is recorded as accepted for `0.1.20` in both docs surfaces.
- [x] Registry URL, version, date, and install metadata are recorded in release-status docs.
- [x] Registry row binds accepted metadata to `server.json` and `ghcr.io/edithatogo/sourceright-mcp:0.1.20`.
- [x] Artifact checksums remain GitHub Release evidence, not registry metadata.

## Glama Separation
- [x] Glama metadata is validated independently of MCP Registry.
- [x] Glama remains `prepared` because no accepted external listing is recorded.
- [x] Smithery remains governed by Track 57 and is not used as MCP Registry evidence.
