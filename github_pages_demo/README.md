# Sourceright GitHub Pages Demo

This is a static, sample-data-only demonstrator. It reads local JSON artifacts
from `sample/` and renders the current `sourceright.reference_report.v1` and
`sourceright.journal_screening.v1` contracts.

It does not call providers, require API keys, or mutate repository state.

To preview locally, serve this directory with any static file server, for
example:

```text
python -m http.server 8080
```

The repository already has an mdBook Pages workflow, so this demo is kept as a
standalone directory rather than adding another Pages deployment workflow.
