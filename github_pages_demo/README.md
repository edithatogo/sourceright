# Sourceright GitHub Pages Demo

This is a static, sample-data-only demonstrator. It reads local JSON artifacts
from `sample/` and renders the current `sourceright.reference_report.v1` and
`sourceright.journal_screening.v1` contracts.

It is not a live verification service. It does not call providers, require API
keys, or mutate repository state.

## Quick start

1. Open [index.html](./index.html) directly in a browser, or serve this
   directory with any static file server.
2. Review the sample report card, open issues, and journal screening summary.
3. Use [demo-checklist.md](./demo-checklist.md) for a quick pre-share check.

Example local server:

```text
python -m http.server 8080
```

## Smoke checks

The default automated check runs without a browser:

```text
node github_pages_demo/render-smoke.mjs
```

The real browser smoke is opt-in because it requires Playwright:

```text
SOURCERIGHT_DEMO_BROWSER_SMOKE=1 node github_pages_demo/browser-smoke.mjs
```

Missing Node fails in CI through the Rust `demo_policy` test. Missing
Playwright only fails when the browser smoke is explicitly enabled.

## What the sample report card means

- `References` is the total number of references in the sample payload.
- `Verified` shows how many references already have provider evidence.
- `Review queue` shows how many items are still waiting on manual review.
- `AI-risk signals` counts issues flagged as potentially higher risk.
- `Warnings` and `Errors` show the current issue severity mix.

The repository already has an mdBook Pages workflow, so this demo is kept as a
standalone directory rather than adding another Pages deployment workflow.
