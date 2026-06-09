# `journal-screen` smoke — arXiv submit-ce fixture

Date: 2026-06-10

## Command

```powershell
$bin = "C:\tmp\sourceright-target-live\x86_64-pc-windows-gnu\release\sourceright.exe"
$root = "C:\tmp\sourceright-journal-test"
# init workspace, copy fixture CSL + verification sidecar from
# fixtures/journal/arxiv-submit-ce-submission.json
& $bin journal-screen --platform arxiv-submit-ce `
  --submission-id ARXIV-CE-2026-0001 `
  --manuscript source-package.tar.gz `
  .sourceright
```

## Result

Exit code `0`. Emitted `sourceright.journal_screening.v1`:

- `platform`: `arxiv_submit_ce`
- `status`: `screened_with_warnings`
- `reference_report.summary`: 2 references, 1 verified, 1 review-queue item, 1 AI-risk citation signal
- Issues: missing DOI (`nguyen-2025-missing`), manual review queued

## Boundary

Local, read-only screening from fixture-backed workspace data. No `submit-ce` API
calls, no arXiv credentials, no writeback to canonical CSL or arXiv submission state.

## Proposed upstream hook (submit-ce)

Primary integration surface in active `arXiv/submit-ce` repo:

1. **Post-extraction / pre-submit** — after the source bundle is unpacked and
   bibliography metadata is available, export a small JSON bundle and invoke
   `sourceright journal-screen --platform arxiv-submit-ce` as an external
   read-only step; surface `author_action_checklist` in the submit UI.
2. **TeX log pipeline** — `submit_ce/ui/filters/tex_filters.py` already flags
   `Citation.*undefined` on the final TeX run; reference integrity screening
   complements (does not replace) those compile-time warnings with structured
   CSL + verification evidence.

`arXiv/arxiv-submission-core` is legacy/inactive per maintainer on issue #88;
this smoke targets **submit-ce** only.
