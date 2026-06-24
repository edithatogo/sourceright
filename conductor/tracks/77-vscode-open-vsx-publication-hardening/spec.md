# Track 77: VS Code and Open VSX Publication Hardening

## Goal

Turn the deferred VS Code/Open VSX contract into a mature VSIX package and
submission path when editor demand justifies it.

## User Outcome

Users can install a thin VS Code extension only after Workspace Trust,
diagnostics mapping, package build, install smoke, and Marketplace/Open VSX
submission evidence exist.

## Scope

- VSIX package contract and build pipeline.
- Workspace Trust, diagnostics fixture, and dry-run write semantics.
- VS Code Marketplace and Open VSX submission evidence.

## Out Of Scope

- Reimplementing Sourceright verification in TypeScript.
- Claiming Marketplace/Open VSX acceptance before listing evidence.

## Data Contracts

The extension must invoke CLI/MCP diagnostics and stable JSON outputs. Writes
are preview-only until explicit apply and audit evidence exists.

## Claim Boundary

No VS Code or Open VSX plugin claim exists until a VSIX package builds, installs,
and has submission/listing evidence.

## Evidence Level Target

Hardened local package, submitted, then publicly accepted.

## Parallelization Plan

Requirements discovery, extension scaffold, and Marketplace/Open VSX metadata
drafts can run in parallel after the extension data contract is frozen.

## Maturity, Stability, And Testing

Maturity requires package validation, local install/uninstall smoke, Workspace
Trust behavior, diagnostics fixture coverage, and listing evidence.
