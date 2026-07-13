# Smithery republish evidence

Date: 2026-07-13

## Repository-side blocker addressed

The Linux MCPB builder now uses `scripts/create-mcpb-zip.mjs`, a dependency-free
ZIP writer that sets Unix creator metadata and executable mode `0755` for
`bin/sourceright`. Non-executable bundle files are emitted with mode `0644`.

Verified artifact: `.tmp/sourceright-smithery-linux-posix.mcpb`

The archive inspection reports:

- `bin/sourceright`: Unix file attributes `000755` (`rwxr-xr-x`)
- manifest, README, and license files: Unix file attributes `000644`

The canonical MCPB template also now includes the optional
`user_config.workspace_root` schema present in the last successful Smithery
release. It is not required and defaults to the current working directory.

## External residual blocker

The Smithery CLI publish attempt was repeated with a newly created transient
API key and the corrected POSIX-aware bundle before the manifest parity fix.
Smithery returned:

```text
Deployment failed: 400 {"error":"No values to set"}
```

The transient key was stored only in `C:/tmp/smithery-release-key.tmp` during
the attempt and was deleted afterwards. No credential is recorded here or in
the repository.

The manifest parity fix is now in the repository, but a fresh external publish
is still required to confirm the deployment state. If the response persists,
the remaining action is Smithery support/API investigation of `No values to
set` or a registry-side republish path.

## Checks

- `node --check scripts/create-mcpb-zip.mjs`
- `cargo fmt --check`
- `cargo test --locked --test dependency_policy`
- `git diff --check -- scripts/build-smithery-mcpb.ps1`
- `unzip -Z -v .tmp/sourceright-smithery-linux-posix.mcpb`
