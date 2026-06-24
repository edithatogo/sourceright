---
title: Journal workflow integrations
description: Editorial workflow integrations and screening contracts.
---

Journal integrations support editorial screening contracts for OJS, arXiv
submission platforms, and related editorial workflows.

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

## arXiv platform adapter contracts

arXiv support is split into two boundaries:

- `provider.arxiv` is sidecar-only preprint identity and version evidence.
- `journal.arxiv-submit-ce` and `journal.arxiv-submission-core` are supported journal-screening platform labels with adapter contract fixtures.

The platform labels reuse `sourceright journal-screen` and emit
`sourceright.journal_screening.v1`. They are fixture-backed technical-preview
contract fixtures only: no upstream arXiv patch, live arXiv credential, platform
writeback, or canonical CSL writeback is claimed.

Supported platform values:

- `arxiv-submit-ce` / `arxiv_submit_ce`
- `arxiv-submission-core` / `arxiv_submission_core`

The track is split for parallel work: shared schema/CLI/MCP contracts are owned
centrally, while the current `submit-ce` fixture lane and the legacy
`arxiv-submission-core` fixture lane can be implemented and reviewed
independently.

Track 79 submit-ce maturity hardening pins contract drift checks in
`conductor/tracks/79-arxiv-submit-ce-maturity-hardening/schema-drift-check-2026-06-09.md`
with `submit-ce-contract-snapshot.json` and `security-boundaries.md`.

Track 80 submission-core maturity hardening pins migration mapping checks in
`conductor/tracks/80-arxiv-submission-core-maturity-hardening/migration-mapping-check-2026-06-09.md`
with `submission-core-contract-snapshot.json` and `security-boundaries.md`.
