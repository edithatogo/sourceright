# Pre-Release Validation

Checklist and results for gate checks that must pass before tagging a release.

**Validated against:** v0.1.20
**Validation date:** 2026-05-15
**Validator:** Track 33 release engineering audit
**Schema:** MCP 2025-12-11 server.schema.json

## 1. Crate Packaging (`cargo package --locked`)

### 1a. Strict mode

```text
$env:CARGO_TARGET_DIR='C:\tmp\sourceright-track33-gnu-target'
$ cargo +stable-x86_64-pc-windows-gnu package --locked
Finished `dev` profile [unoptimized + debuginfo] target(s) in 1m 36s
```

| Check | Result | Detail |
|-------|--------|--------|
| `cargo package --locked` | **PASSED** | Packaged 242 files, 976.6 KiB (205.0 KiB compressed). |
| Package verification | **PASSED** | Tarball verification completed under the GNU stable toolchain. |
| Test exclusion warnings | **EXPECTED** | Binary crate test files are not included in the published package. |
| Clean-tree gate | **PASSED** | `git status --short --branch` reported `## main...origin/main` before validation. |

## 2. Crate Dry Run (`cargo publish --dry-run --locked`)

```text
$env:CARGO_TARGET_DIR='C:\tmp\sourceright-track33-gnu-target'
$ cargo +stable-x86_64-pc-windows-gnu publish --dry-run --locked
warning: crate sourceright@0.1.20 already exists on crates.io index
error: failed to write query cache ... There is not enough space on the disk. (os error 112)
```

| Check | Result | Detail |
|-------|--------|--------|
| `cargo publish --dry-run --locked` | **ENVIRONMENT-BLOCKED** | Dry run reached package verification, then failed because local `C:\` had zero free bytes. |
| crates.io index sync | **PASSED** | Index updated successfully. |
| Version collision warning | **EXPECTED** | `sourceright@0.1.20` is already accepted on crates.io. |
| Publish rule | **UNCHANGED** | A future version still requires a successful dry run before publish. |

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

## Accepted Public Release Evidence

| Surface | Status | Evidence |
|---------|--------|----------|
| GitHub Release | **ACCEPTED** | `v0.1.20` release with platform binaries and SHA-256 checksums. |
| crates.io | **ACCEPTED** | `sourceright` version `0.1.20` listed publicly. |
| docs.rs | **ACCEPTED** | `sourceright` version `0.1.20` documentation built from crates.io. |
| Official MCP Registry | **ACCEPTED** | `io.github.edithatogo/sourceright` version `0.1.20` listed as active/latest. |

## Prepared/Deferred Boundary Evidence

| Surface | Status | Boundary |
|---------|--------|----------|
| GHCR MCP image | **PREPARED** | OCI image is referenced by the MCP Registry; direct GHCR package page visibility remains unverified. |
| Glama | **PREPARED** | `glama.json` is present and structurally valid; no accepted external listing is recorded. |
| Smithery | **PREPARED** | MCPB template and builder exist; no accepted external listing is recorded. |
| Package managers and host marketplaces | **DEFERRED** | Release-status docs record blocking requirements and revisit triggers. |

## Summary

| Gate | Status |
|------|--------|
| `cargo package --locked` | PASSED with GNU stable toolchain and temp target directory |
| `cargo publish --dry-run --locked` | ENVIRONMENT-BLOCKED by local disk exhaustion after verification began; future-version dry run still required |
| `server.json` MCP 2025-12-11 schema | PASSED |
| `glama.json` structure | PASSED |
| `Dockerfile` labels (6) | PASSED |
| Dockerfile build structure | PASSED |
| Cross-artifact consistency | PASSED |

**Overall:** Track 33 is complete for the core public release path because
GitHub Release, crates.io, docs.rs, and the official MCP Registry are accepted
for `v0.1.20`, and remaining surfaces are explicitly prepared or deferred.
Future releases must still run a successful publish dry-run before publishing a
new crate version.
