# Demo Public Surface Inventory

## GitHub Pages Static Demo (`github_pages_demo/`)

### Files

| File | Size | Purpose |
|------|------|---------|
| `README.md` | 1,657 B | Demo instructions, smoke check docs, report card explanation |
| `index.html` | 1,712 B | Main HTML page rendering the report card |
| `app.js` | 2,187 B | JavaScript that fetches sample JSON and renders the DOM |
| `styles.css` | 1,981 B | CSS for the demo page |
| `render-smoke.mjs` | 1,476 B | Node.js headless render smoke test |
| `browser-smoke.mjs` | 2,526 B | Playwright browser smoke (opt-in via `SOURCERIGHT_DEMO_BROWSER_SMOKE=1`) |
| `demo-checklist.md` | 471 B | Pre-share checklist |
| `sample/reference-report.json` | 641 B | Sample reference report payload |
| `sample/journal-screening.json` | 1,132 B | Sample journal screening payload |

### Smoke test results
- `render-smoke.mjs` — ✅ Can run (Node required)
- `browser-smoke.mjs` — ✅ Opt-in (requires Playwright + env var)

### Plugin manifest
- `plugins/manifests/demo.github-pages.toml` exists with status `planned`

### Claim boundary
- README explicitly states: "It is not a live verification service. It does not call providers, require API keys, or mutate repository state."

---

## Streamlit Local Demo (`streamlit_app/`)

### Files

| File | Size | Purpose |
|------|------|---------|
| `README.md` | 1,588 B | Demo instructions, smoke check docs, report card explanation |
| `app.py` | 1,131 B | Main Streamlit app |
| `demo_model.py` | 1,632 B | Data model: `load_sample_payloads()`, `metric_rows()`, `journal_summary()` |
| `test_demo_model.py` | 4,388 B | Unit tests for demo model (fake Streamlit module) |
| `server_smoke.py` | 2,526 B | Streamlit server startup smoke (opt-in via `SOURCERIGHT_DEMO_SERVER_SMOKE=1`) |
| `demo-checklist.md` | 460 B | Pre-share checklist |
| `requirements.txt` | 19 B | Python dependencies (streamlit) |
| `__init__.py` | 59 B | Package init |
| `sample_workspace/reference-report.json` | 641 B | Sample reference report payload |
| `sample_workspace/journal-screening.json` | 1,132 B | Sample journal screening payload |

### Smoke test results
- `test_demo_model.py` — ✅ Unit tests pass (Python + unittest, no Streamlit needed)
- `server_smoke.py` — ✅ Opt-in (requires Streamlit + env var)

### Claim boundary
- README explicitly states: "It is not a live verification service, does not call providers, and is intended for market-readiness review of the sample report-card layout only."

---

## Shared Sample Data

Both demos use identical sample JSON files:
- `reference-report.json`: 2 references, 1 verified, 1 review queue, 1 AI-risk signal, 1 warning, 0 errors
- `journal-screening.json`: submission DEMO-001, OJS platform, screened_with_warnings

The `.json` files are identical between `github_pages_demo/sample/` and `streamlit_app/sample_workspace/`.

---

## CI Integration

The Rust `demo_policy` test (in `tests/`) runs:
1. Node.js render smoke (`render-smoke.mjs`)
2. Python unit tests (`test_demo_model.py`)

Both are required to pass in CI. Browser and Streamlit server smokes are opt-in only.

---

## Gaps

| Requirement | Status |
|------------|--------|
| Static demo exists | ✅ |
| Streamlit demo exists | ✅ |
| Render smoke (headless) | ✅ |
| Browser smoke (Playwright) | ✅ opt-in |
| Streamlit unit tests | ✅ |
| Streamlit server smoke | ✅ opt-in |
| Plugin manifest | ✅ `demo.github-pages.toml` exists (status: `planned`) |
| CI integration | ✅ Both smokes run in `demo_policy` test |
| Claim boundary docs | ✅ Both READMEs have clear disclaimers |
