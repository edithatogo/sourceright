---
title: Journal workflow integrations
description: Editorial workflow integrations and screening contracts.
---

Journal integrations support editorial screening contracts for OJS, arXiv
submission platforms, and related editorial workflows.

- Keep the contract platform-neutral where possible.
- Preserve the editorial screening boundary.
- Treat agentic workflow outputs as decision-support evidence, not editorial decisions.

## Agentic submission management

Track 88 extends journal integrations from citation screening toward a structured submission-management and validation layer.

The target submission package can include article text, references, key claims, datasets, code, ethics metadata, reporting checklists, author contributions, conflicts, AI-use declarations, review history, and persistent identifiers.

| Stage | Sourceright-facing role |
| --- | --- |
| Pre-submission guidance | Check scope, article type, reporting guideline, ethics, data, code, contribution, funding, and AI-use readiness. |
| Structured intake | Normalize manuscript text, references, claims, datasets, code, protocols, checklists, conflicts, and identifiers. |
| Triage and routing | Classify submissions by topic, method, article type, computational dependency, ethical risk, and reporting requirements. |
| Reviewer selection support | Provide conflict-aware reviewer suggestions while preserving editor authority. |
| Technical validation | Run citation, source, recency, link, data, code, reporting, and integrity checks. |
| Briefing packs | Summarize claims, methods, datasets, code, reporting status, missing items, versions, and review history. |
| Decision support | Synthesize reviews and draft decision letters without taking the final decision. |
| Post-publication monitoring | Monitor broken links, data or code access, corrections, retractions, public comments, and update triggers. |

Editors remain accountable for editorial decisions, reviewers remain accountable for reviews, and authors remain responsible for submissions.

## Future research object models

Journal adapters should be compatible with living evidence records, executable research objects, refereed preprint network papers, machine-actionable claim graphs, and experimental community-governed research ledgers. The detailed roadmap is in the Future scientific publishing page and Track 88.

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

## Janeway Next

Janeway is the next open-source journal platform to map because it is plugin-oriented and keeps the extension story inside a documented open-source project rather than a vendor-only API surface.

The Janeway tracks should start with:

- plugin hook discovery and install packaging boundaries;
- a Sourceright adapter contract that keeps screening logic in the Rust core;
- disposable-instance or local-package smoke evidence before any publication claim.

The target integration shapes are:

- plugin-first where Janeway exposes stable hooks and packaging;
- sidecar-service where the journal installs a thin bridge and Sourceright remains the screening engine;
- hybrid where plugin metadata launches the CLI and a report artifact returns to the journal UI.

## Enterprise Adapters

ScholarOne, Editorial Manager, eJournalPress, Manuscript Manager, and similar editorial systems should initially be handled through adapter contracts or generic batch/webhook workflows. Live integrations should wait for platform API access, vendor documentation, or publisher test environments.

The generic adapter contract should define:

- Submission id.
- Manuscript file or extracted text location.
- Optional reference-list text or CSL JSON.
- Report destination.
- Severity thresholds.
- Visibility rules for editors, reviewers, and authors.

## Self-Improving Registry

The platform roadmap should stay machine-readable and self-improving:

- agents can survey official docs and repo fixtures to propose new platform tracks;
- skills can encode repeatable discovery, packaging, and smoke workflows;
- human review stays required before any new platform claim moves past reconnaissance.

That means the adapter registry should prefer exact surfaces, evidence links, and bounded claims over broad "supported platform" language.
