# Pre-Release Validation

Checklist and results for gate checks that must pass before tagging a release.

**Validated against:** v0.1.20
**Validation date:** 2026-05-14
**Validator:** Track 33 release engineering audit

## 1. Crate Packaging (`cargo package --locked`)

| Check | Result | Detail |
|-------|--------|--------|
| `cargo package --locked` | **BLOCKED** (dirty tree) | 8+ uncommitted files detected. `cargo package` requires a clean git working tree when `--locked` is used. |
| `cargo package --locked --allow-dirty` | **PASSED** | Package assembles successfully. 21 test files are warned as excluded (not in `Cargo.toml` `include` list) — expected for a binary crate where tests aren't needed by consumers. |
| `Cargo.toml` metadata | **PASSED** | `name`, `version`, `edition`, `rust-version`, `description`, `license`, `repository`, `homepage`, `documentation`, `readme`, `keywords`, `categories` all populated. |

**Gate rule:** A clean git tree is mandatory before `cargo package --locked` will succeed. The current dirty state confirms the gate works as designed — release engineering must commit all changes before packaging.

## 2. Crate Dry Run (`cargo publish --dry-run --locked`)

| Check | Result | Detail |
|-------|--------|--------|
| `cargo publish --dry-run --locked` | **BLOCKED** (dirty tree) | Same dirty-tree gate as packaging. |
| `cargo publish --dry-run --locked --allow-dirty` | **PASSED** (warning only) | Warning: `crate sourceright@0.1.20 already exists on crates.io index`. Package structure is valid; the version collision is expected (v0.1.20 is already published). |
| crates.io index sync | **PASSED** | Index updated successfully during dry-run. |

## 3. MCP Server Manifest (`server.json`)

Validated against MCP 2025-12-11 schema.

### Required fields (ServerDetail)

| Field | Value | Valid? |
|-------|-------|--------|
| `name` | `io.github.edithatogo/sourceright` | ✅ Pattern match, ≤200 chars |
| `description` | 86 chars | ✅ ≤100 limit |
| `version` | `0.1.20` | ✅ Semver, ≤255 chars |

### Repository

| Field | Value | Valid? |
|-------|-------|--------|
| `url` | `https://github.com/edithatogo/sourceright` | ✅ HTTPS URI |
| `source` | `github` | ✅ |

### Packages

| Field | Value | Valid? |
|-------|-------|--------|
| `registryType` | `oci` | ✅ |
| `identifier` | `ghcr.io/edithatogo/sourceright-mcp:0.1.20` | ✅ |
| `transport.type` | `stdio` | ✅ |

### Cross-artifact consistency

| Check | Result |
|-------|--------|
| server.json version matches Cargo.toml (0.1.20) | ✅ |
| server.json name matches Dockerfile MCP label | ✅ |
| OCI identifier matches Dockerfile target | ✅ |
| $schema URL resolves (200 OK) | ✅ |

## 4. Glama Manifest (`glama.json`)

| Check | Result |
|-------|--------|
| `$schema` present | ✅ `https://glama.ai/mcp/schemas/server.json` |
| `maintainers` array | ✅ `["edithatogo"]` |
| Public LICENSE | ✅ Apache-2.0 + MIT dual |
| MCP metadata discoverable | ✅ server.json at root |

## 5. Dockerfile

### OCI Labels

| Label | Value | Valid? |
|-------|-------|--------|
| `org.opencontainers.image.source` | `https://github.com/edithatogo/sourceright` | ✅ |
| `org.opencontainers.image.description` | `Sourceright MCP stdio server` | ✅ |
| `org.opencontainers.image.url` | `https://github.com/edithatogo/sourceright` | ✅ |
| `org.opencontainers.image.version` | `0.1.20` | ✅ matches Cargo.toml |
| `org.opencontainers.image.licenses` | `MIT OR Apache-2.0` | ✅ matches Cargo.toml |
| `io.modelcontextprotocol.server.name` | `io.github.edithatogo/sourceright` | ✅ matches server.json |

### Build structure

| Check | Result |
|-------|--------|
| Multi-stage (rust:1.94-bookworm → debian:bookworm-slim) | ✅ |
| Pinned digests | ✅ |
| ENTRYPOINT + CMD for stdio | ✅ |
| `--locked` in cargo build | ✅ |

## Summary

| Gate | Status |
|------|--------|
| `cargo package --locked` | BLOCKED by dirty tree (passes with --allow-dirty) |
| `cargo publish --dry-run --locked` | BLOCKED by dirty tree (passes with --allow-dirty) |
| `server.json` schema validation | ✅ PASSED |
| `glama.json` structure | ✅ PASSED |
| `Dockerfile` labels | ✅ PASSED |
| Cross-artifact consistency | ✅ PASSED |

**Overall:** All structural validations pass. No regressions or misconfigurations found.
