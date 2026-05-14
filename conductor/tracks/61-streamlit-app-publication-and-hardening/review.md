# Track 61 — Streamlit App Publication and Hardening: Review

## Current State

The Streamlit demo app at `streamlit_app/` is already well-implemented:

### Files
| File | Status | Purpose |
|------|--------|---------|
| `app.py` | ✅ | Main Streamlit app (report card + journal screening display) |
| `demo_model.py` | ✅ | Data loading, metrics, explanation logic |
| `test_demo_model.py` | ✅ | 2 unit tests with fake Streamlit module |
| `server_smoke.py` | ✅ | Opt-in server startup smoke (env: `SOURCERIGHT_DEMO_SERVER_SMOKE=1`) |
| `demo-checklist.md` | ✅ | Pre-share checklist |
| `requirements.txt` | ✅ | Only dependency: `streamlit` |
| `sample_workspace/` | ✅ | Synthetic sample data (reference-report.json, journal-screening.json) |
| `README.md` | ✅ | Demo instructions + claim boundary disclaimer |

### Test Coverage
- **Unit tests** (`test_demo_model.py`): 2 tests — sample payload validation and app import rendering. Passes without Streamlit installed.
- **Server smoke** (`server_smoke.py`): Starts Streamlit server, checks HTTP 200 with "streamlit" in body. Opt-in.

### CI Integration
- Rust `demo_policy` test in `tests/` runs `python -m unittest streamlit_app.test_demo_model`
- Server smoke is opt-in only (not run in default CI)

### Claim Boundary
- README and app page both state: synthetic data only, no live providers, no API keys needed

---

## Hardening Assessment

| Requirement | Status | Notes |
|------------|--------|-------|
| Server startup smoke | ✅ | `server_smoke.py` works, opt-in |
| Sample data is synthetic | ✅ | `sample_workspace/` with 2 references |
| No live provider calls | ✅ | All data from local JSON files |
| Unit tests without Streamlit | ✅ | `test_demo_model.py` uses fake module |
| Deployment docs | ❌ | No Streamlit Community Cloud docs |
| Privacy notes | ❌ | README has claim boundary but no privacy policy |
| CI requirement | ✅ | Unit tests run in CI via `demo_policy` |

---

## Recommendations

### 1. Add Streamlit Community Cloud Deployment Docs
Create `streamlit_app/deploy-docs.md` with:
- `secrets.toml` template (empty — no secrets needed for synthetic demo)
- Resource limits (free tier: 1 GB RAM, 10 GB storage)
- Environment variables (none required)
- Privacy note: "This demo uses only synthetic sample data. No user data is collected, stored, or transmitted."

### 2. Add `streamlit_app/DEPLOY.md`
Short deployment steps for Cloud or local hosting.

### 3. Consider Hardening
- Add `streamlit_app/security.md` noting the demo does not handle authentication or sensitive data
- Add error boundary in `app.py` for missing/corrupt sample data

### 4. Plugin Manifest
`plugins/manifests/demo.github-pages.toml` only covers the static demo. Consider whether a separate Streamlit manifest is needed, or update the existing one to cover both surfaces.

---

## Status

- **Previous status**: planned
- **New status**: in_progress (core implementation exists; deployment docs and hardening remain)
