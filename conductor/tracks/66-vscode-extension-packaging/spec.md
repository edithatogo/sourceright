# VS Code Extension Packaging Spec

## Goal

Define the editor-extension contract for inline Sourceright diagnostics without
reimplementing reference verification outside the Rust core.

## Contract

This track is complete only when a VSIX-ready extension or explicit deferral
exists with:

- manifest/package metadata and Marketplace or Open VSX submission notes;
- CLI or MCP adapter wiring that shells out to the existing core;
- diagnostic mapping from stable CLI/MCP JSON into editor ranges;
- local workspace privacy, dry-run writeback, and audit-log behavior; and
- install, smoke, and uninstall checks.

## Claim Boundary

`.vscode/settings.json` and rust-analyzer configuration are development
tooling, not a Sourceright VS Code extension.
