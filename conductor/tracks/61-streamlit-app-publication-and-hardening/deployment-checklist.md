# Streamlit App Deployment Checklist

## Overview

This checklist covers deployment of the Sourceright Streamlit demo app
(`streamlit_app/app.py`) to Streamlit Community Cloud. The app uses only
synthetic sample data and requires no authentication, secrets, or live
provider calls.

## Prerequisites

- [ ] **GitHub repository** containing the `streamlit_app/` directory
- [ ] **Streamlit Community Cloud account** at share.streamlit.io
- [ ] **Python 3.10+** (for local verification before deployment)
- [ ] **streamlit** package installed locally (for smoke testing)

## Pre-Deployment Checks

### 1. Local Verification

- [ ] Run unit tests: `python -m unittest streamlit_app.test_demo_model`
- [ ] Run the app locally: `streamlit run streamlit_app/app.py`
- [ ] Confirm the app opens at `http://localhost:8501`
- [ ] Confirm the UI shows sample synthetic data, not live data
- [ ] Confirm no broken pages, errors, or missing data

### 2. Repository Structure

Verify `streamlit_app/` contains:

- [ ] `app.py` — Main Streamlit application entry point
- [ ] `demo_model.py` — Data loading and metrics logic
- [ ] `test_demo_model.py` — Unit tests (passes without Streamlit installed)
- [ ] `server_smoke.py` — Opt-in server startup smoke test
- [ ] `requirements.txt` — Dependencies (only `streamlit`)
- [ ] `README.md` — Demo instructions with claim boundary disclaimer
- [ ] `demo-checklist.md` — Pre-share checklist for demo sessions
- [ ] `sample_workspace/` — Synthetic sample data directory
- [ ] No API keys, credentials, or secrets in any file

### 3. CI Checks

- [ ] `cargo test demo_policy` passes (runs Python unittest)

## Streamlit Cloud Deployment Steps

### Step 1: Push to GitHub

- [ ] Commit and push the latest `streamlit_app/` changes
- [ ] Confirm the repository is accessible from Streamlit Community Cloud

### Step 2: Connect Repository

- [ ] Go to [share.streamlit.io](https://share.streamlit.io)
- [ ] Click "New app"
- [ ] Select the GitHub repository and branch
- [ ] Set **Main file path** to `streamlit_app/app.py`
- [ ] Click "Deploy"

### Step 3: Wait for Build

- [ ] Monitor the build log for errors
- [ ] Expected output: "You can now view your Streamlit app in your browser."
- [ ] Confirm the app URL is generated

## Configuration

### Environment Variables

The app requires **no environment variables** or secrets for the synthetic
demo. The `requirements.txt` file only declares `streamlit` as a dependency.

### Secrets

No `secrets.toml` is needed. The app does not authenticate with any service.

**If a `secrets.toml` is added in the future**, use the Streamlit Cloud
secrets management console instead of checking secrets into the repo.

## Resource Limits (Free Tier)

| Resource | Limit | Notes |
|----------|-------|-------|
| RAM | 1 GB | Sufficient for synthetic demo |
| Storage | 10 GB | Shared across all deployed apps |
| Idle timeout | ~15 minutes | App sleeps; wakes on next request |
| Build timeout | ~15 minutes | |
| Concurrency | 1 simultaneous visitor | |

## Post-Deployment Smoke Test

### Basic Smoke

- [ ] Open the deployed URL in a browser
- [ ] Confirm page loads without errors
- [ ] Confirm sample data is displayed (reference report with 2 entries)
- [ ] Confirm the app does **not** show live provider data or API errors
- [ ] Confirm privacy note / disclaimer is visible on the page

### Functional Smoke

- [ ] Interact with each UI element (expand sections, click buttons)
- [ ] Confirm no JavaScript console errors

### CI Gate Smoke

- [ ] Run `cargo test demo_policy` against the deployed commit

## Maintenance Checklist

### After Each Update

- [ ] Run unit tests locally
- [ ] Run app locally to verify changes
- [ ] Push changes to GitHub (auto-deploys from connected branch)
- [ ] Verify auto-deployment succeeded in Streamlit Cloud dashboard

### Troubleshooting

| Issue | Likely Cause | Solution |
|-------|-------------|----------|
| App fails to build | Missing requirements.txt | Verify app.py path is correct |
| App shows error | Missing/corrupt sample data | Verify sample_workspace/*.json |
| App is slow | Free tier RAM limit | Reduce sample data size |
| App goes to sleep | Idle timeout (15 min) | Normal; reload to wake |

## Claim Boundary Reminder

Before sharing the deployed URL, ensure the following are communicated:

- [ ] This is a **synthetic data demonstrator** — not a production tool
- [ ] No live provider APIs are called
- [ ] No API keys or authentication are required
- [ ] The app does not handle real user data

## Checklist Summary

```
Pre-Deployment:
  [ ] Unit tests pass
  [ ] App runs locally
  [ ] Repository structure correct
  [ ] CI gate passes

Deployment:
  [ ] GitHub repo connected to Streamlit Cloud
  [ ] Main file set to streamlit_app/app.py
  [ ] App builds and deploys successfully

Post-Deployment:
  [ ] Basic smoke test passes
  [ ] Functional smoke test passes
  [ ] Claim boundary communicated
```
