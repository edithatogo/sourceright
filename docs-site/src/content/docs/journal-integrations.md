---
title: Journal workflow integrations
description: Editorial workflow integrations and screening contracts.
---

Journal integrations support editorial screening contracts for OJS and related
platforms.

- Keep the contract platform-neutral where possible.
- Preserve the editorial screening boundary.

## OJS plugin source skeleton

The repository includes a thin OJS generic-plugin source skeleton at
`plugins/ojs/sourceright/`. It is a wrapper around the Sourceright CLI/MCP core,
not a reimplementation of reference verification in PHP.

The skeleton:

- calls `sourceright journal-screen --platform ojs`;
- keeps export integration on `sourceright export --preview`;
- escapes command arguments before invoking the CLI;
- separates editor-facing report output from author-facing checklist output;
- keeps future write-capable flows behind explicit configuration.

This is fixture-backed and suitable for controlled pilot wiring. It is not PKP
Plugin Gallery accepted, and it still needs live OJS handler/settings-form and
workflow-template wiring before external Gallery review.

For local install testing, build the generic-plugin archive:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/build-ojs-plugin-package.ps1
```

Then extract it to `plugins/generic/sourceright` in a disposable OJS instance
and register the plugin with:

```bash
php lib/pkp/tools/installPluginVersion.php plugins/generic/sourceright/version.xml
```

Repo-local OJS checks that do not require Docker can be run with:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/ojs-plugin-lint.ps1
```
