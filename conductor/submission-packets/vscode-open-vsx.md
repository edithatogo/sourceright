# VS Code and Open VSX Submission Packet

## Surfaces

- VS Code Marketplace
- Open VSX

## Requirements Evidence

| Surface | Source | Retrieved | Local impact |
| --- | --- | --- | --- |
| VS Code Marketplace | <https://code.visualstudio.com/api/working-with-extensions/publishing-extension> | 2026-05-18 | VSIX packaging and publishing are mediated through `vsce`; quality, package metadata, and Marketplace publishing credentials are required. |
| Open VSX | <https://github.com/eclipse/openvsx/wiki/Publishing-Extensions> | 2026-05-18 | Open VSX publication requires extension package metadata and registry publishing steps separate from VS Code Marketplace. |

## Local Gates

- Thin VSIX scaffold that calls Sourceright CLI/MCP.
- Workspace Trust behavior.
- Diagnostics fixture coverage.
- Package build.
- Install/uninstall smoke.
- Marketplace and Open VSX draft metadata.

## Blockers

None.

## Draft Submission Body

Marketplace/Open VSX metadata is drafted in
`conductor/tracks/77-vscode-open-vsx-publication-hardening/marketplace-metadata-draft.md`.
Local VSIX build and install/uninstall smoke are documented in
`conductor/tracks/77-vscode-open-vsx-publication-hardening/vsix-build-2026-05-18.md`
and refreshed in `vsix-smoke-2026-06-09.md`. Submission drafts live in
`submission-drafts.md`. Marketplace/Open VSX publication remains
approval-gated and requires submitted or accepted listing evidence before
public acceptance is claimed.

## Approval Gate

No VS Code Marketplace or Open VSX publication without explicit approval.
