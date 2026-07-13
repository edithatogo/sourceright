# VS Code Marketplace / Open VSX Metadata Draft

Date: 2026-05-18

## Extension Identity

- Name: `sourceright`
- Display name: `Sourceright Reference Screening`
- Publisher: `edithatogo`
- Categories: `Other`, `Linters`
- License: `MIT OR Apache-2.0`
- Repository: `https://github.com/edithatogo/sourceright`

## Description

Sourceright Reference Screening is a thin editor integration for the
Sourceright CLI and MCP server. It surfaces deterministic reference-integrity
diagnostics from local Sourceright workspaces without reimplementing provider
verification in TypeScript.

## Submission Boundary

This draft is not a marketplace submission. VS Code Marketplace and Open VSX
publication remain blocked until a VSIX builds, Workspace Trust behavior is
tested, diagnostics fixtures are covered, and install/uninstall smoke is
recorded.

## Required Evidence Before Submission

- VSIX package artifact.
- Workspace Trust test showing write commands stay disabled or preview-only in
  untrusted workspaces.
- Diagnostics fixture mapping from CLI JSON to editor diagnostics.
- Install/uninstall smoke in VS Code or compatible extension host.
- Separate Marketplace and Open VSX submission receipts or listing URLs.
