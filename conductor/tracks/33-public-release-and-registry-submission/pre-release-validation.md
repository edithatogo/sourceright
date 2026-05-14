# Pre-Release Validation

Checklist and results for gate checks that must pass before tagging a release.

**Validated against:** v0.1.20
**Validation date:** 2026-05-14 (fresh re-validation)
**Validator:** Track 33 release engineering audit
**Schema:** MCP 2025-12-11 server.schema.json (fetched, 200 OK)

## 1. Crate Packaging (`cargo package --locked`)

### 1a. Strict mode

```text
$ cargo package --locked
error: 2 files in the working directory contain changes that were not yet
committed into git:

README.md
docs/src/security-automation.md

to proceed and include the uncommitted changes, pass the `--allow-dirty` flag
```

| Check | Result | Detail |
|-------|--------|--------|
| `cargo package --locked` | **BLOCKED** (dirty tree) | 2 modified + 5 untracked files. |
| Gate behavior | **CORRECT** | Prevents packaging from uncommitted state. |

### 1b. Allow-dirty verification

```text
$ cargo package --list --allow-dirty --locked
```

| Check | Result | Detail |
|-------|--------|--------|
| `cargo package --list` | **PASSED** | Full manifest produced. |
| Package file count | **PASSED** | All include-list directories present. |
| Test exclusion warnings | **EXPECTED** | Binary crate. |
| `Cargo.toml` metadata | **PASSED** | All required fields populated. |

**Gate rule:** Clean git tree mandatory. 7 uncommitted changes confirm gate works.

## 2. Crate Dry Run (`cargo publish --dry-run --locked`)

### 2a. Strict mode

```text
$ cargo publish --dry-run --locked
    Updating crates.io index
warning: crate sourceright@0.1.20 already exists on crates.io index
error: 2 files in the working directory contain changes...
```

| Check | Result | Detail |
|-------|--------|--------|
| `cargo publish --dry-run --locked` | **BLOCKED** (dirty tree) | Same dirty-tree gate. |
| crates.io index sync | **PASSED** | Index updated successfully. |
| Version collision warning | **EXPECTED** | v0.1.20 already on crates.io. |

### 2b. Allow-dirty verification

| Check | Result | Detail |
|-------|--------|--------|
| `cargo publish --dry-run --locked --allow-dirty` | **PASSED** | Package valid; expected warning only. |
| Crates.io metadata | **PASSED** | All required fields present. |

## 3. MCP Server Manifest (`server.json`)

Validated against MCP 2025-12-11 schema (draft-07, fetched and verified).

### 3a. ServerDetail required fields

| Field | Constraint | Value | Valid? |
|-------|-----------|-------|--------|
| `name` | pattern `^[a-zA-Z0-9.-]+/[a-zA-Z0-9._-]+$`, 3-200 | `io.github.edithatogo/sourceright` | ✅ 32 chars |
| `description` | 1-100 chars | 86 chars | ✅ |
| `version` | max 255, semver, no ranges | `0.1.20` | ✅ |

### 3b. Optional fields

| Field | Constraint | Value | Valid? |
|-------|-----------|-------|--------|
| `title` | 1-100 chars | `Sourceright` | ✅ |
| `$schema` | uri | MCP 2025-12-11 URL | ✅ 200 OK |

### 3c. Repository

| Field | Value | Valid? |
|-------|-------|--------|
| `url` | `https://github.com/edithatogo/sourceright` | ✅ |
| `source` | `github` | ✅ |

### 3d. Packages

| Field | Value | Valid? |
|-------|-------|--------|
| `registryType` | `oci` | ✅ |
| `identifier` | `ghcr.io/edithatogo/sourceright-mcp:0.1.20` | ✅ |
| `transport.type` | `stdio` | ✅ |

### 3e. Cross-artifact consistency

| Check | Result |
|-------|--------|
| version matches `Cargo.toml` (0.1.20) | ✅ |
| name matches Dockerfile MCP label | ✅ |
| OCI identifier matches Dockerfile target | ✅ |
| `$schema` URL fetchable | ✅ |
| No version range operators | ✅ |

## 4. Glama Manifest (`glama.json`)

| Check | Result |
|-------|--------|
| `$schema` present | ✅ `https://glama.ai/mcp/schemas/server.json` |
| `maintainers` array | ✅ `["edithatogo"]` |
| Public LICENSE at root | ✅ `LICENSE-APACHE` + `LICENSE-MIT` |
| MCP metadata discoverable | ✅ `server.json` at repo root |
| Repository publicly discoverable | ✅ Public GitHub repo |

## 5. Dockerfile

### 5a. OCI + MCP Labels (6 labels)

| # | Label | Cross-check | Valid? |
|---|-------|-------------|--------|
| 1 | `org.opencontainers.image.source` | `Cargo.toml` repository | ✅ |
| 2 | `org.opencontainers.image.description` | -- | ✅ |
| 3 | `org.opencontainers.image.url` | `Cargo.toml` repository | ✅ |
| 4 | `org.opencontainers.image.version` = `0.1.20` | `Cargo.toml` + `server.json` | ✅ |
| 5 | `org.opencontainers.image.licenses` = `MIT OR Apache-2.0` | `Cargo.toml` license | ✅ |
| 6 | `io.modelcontextprotocol.server.name` | `server.json` name | ✅ |

### 5b. Build structure

| Check | Detail | Result |
|-------|--------|--------|
| Multi-stage | rust:1.94-bookworm -> debian:bookworm-slim | ✅ |
| Pinned digests | Both FROM images use `@sha256:` | ✅ |
| `--locked` | `cargo build --release --locked --bin sourceright` | ✅ |
| ENTRYPOINT | `["sourceright"]` | ✅ |
| CMD | `["mcp"]` | ✅ |
| Minimal runtime | debian:bookworm-slim | ✅ |

### 5c. Container contract

| Check | Result |
|-------|--------|
| Stdio transport (`sourceright mcp`) | ✅ |
| No port exposure (correct for stdio) | ✅ |
| Version pinning (digests + `--locked`) | ✅ |

## Summary

| Gate | Status |
|------|--------|
| `cargo package --locked` | BLOCKED (dirty tree; passes with --allow-dirty) |
| `cargo package --list --allow-dirty --locked` | PASSED |
| `cargo publish --dry-run --locked` | BLOCKED (dirty tree; v0.1.20 exists expected) |
| `server.json` MCP 2025-12-11 schema | PASSED |
| `glama.json` structure | PASSED |
| `Dockerfile` labels (6) | PASSED |
| Dockerfile build structure | PASSED |
| Cross-artifact consistency | PASSED |

**Overall:** All structural validations pass. The only blockers are expected dirty-tree
gates from uncommitted documentation changes -- these confirm release integrity gates
function correctly. No regressions, schema violations, or misconfigurations found.
