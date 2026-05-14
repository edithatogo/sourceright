# Track 54 — Demo Public Surface Proof: Review

## Current State

Both demo surfaces are fully implemented with:

- **Working sample data** (2 references, journal screening output)
- **Headless smoke tests** (Node render smoke, Python unit tests)
- **Opt-in browser/server smokes** (Playwright / Streamlit)
- **Clear claim boundaries** in READMEs (no live providers, no API keys, synthetic data only)
- **CI integration** via Rust `demo_policy` test

## Demo Maturity

| Dimension | GitHub Pages | Streamlit |
|-----------|-------------|-----------|
| Content | ✅ index.html + app.js + styles.css | ✅ app.py + demo_model.py |
| Sample data | ✅ `sample/` dir | ✅ `sample_workspace/` dir |
| Headless test | ✅ `render-smoke.mjs` | ✅ `test_demo_model.py` |
| Browser/server test | ✅ `browser-smoke.mjs` (opt-in) | ✅ `server_smoke.py` (opt-in) |
| Documentation | ✅ README + checklist | ✅ README + checklist |
| Claim boundary | ✅ Clear disclaimer | ✅ Clear disclaimer |
| CI presence | ✅ via `demo_policy` test | ✅ via `demo_policy` test |

## Plugin Manifest Status

`plugins/manifests/demo.github-pages.toml` has status `planned`. This is appropriate since the demo is not yet deployed to GitHub Pages (the mdBook site has its own Pages workflow).

## Gaps / Improvements Suggested

1. **No GitHub Pages deployment workflow** for this demo directory (mdBook has its own). Consider adding a separate Pages deploy or merging into the existing workflow.
2. **Plugin manifest status** could be updated from `planned` to `fixture_tested` once a deploy workflow exists.
3. **Streamlit Community Cloud** deployment instructions are absent — could add `deploy-docs.md`.
4. **Sample data parity** — the two sample JSON files are duplicated across both directories. Consider symlinking or sharing from a common location.

## Status

- **Previous status**: planned
- **New status**: completed (both demo surfaces are functional and tested)
