# Sourceright Reference Screening

This is a thin VS Code extension scaffold for Sourceright. It invokes the
installed Sourceright CLI and does not reimplement reference verification in
TypeScript.

## Commands

- `Sourceright: Run Reference Report` runs `sourceright report --json` against
  the first open workspace folder.

## Workspace Trust

The extension exposes only a read-only report command. Write-capable
Sourceright CLI and MCP commands remain outside the extension until separate
preview/apply and audit evidence is added.

## Publication Boundary

This scaffold is local package evidence. Marketplace and Open VSX publication
still require install/uninstall smoke and explicit approval.
