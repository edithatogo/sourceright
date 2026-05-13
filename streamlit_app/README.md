# Sourceright Streamlit Demo

This optional local demo renders sample Sourceright JSON artifacts. It is not
part of the Rust build, is not a live verification service, does not call
providers, and is intended for market-readiness review of the sample report-card
layout only.

## Quick start

1. Install dependencies:

   ```text
   pip install -r streamlit_app/requirements.txt
   ```

2. Launch the demo:

   ```text
   streamlit run streamlit_app/app.py
   ```

3. Use [demo-checklist.md](./demo-checklist.md) to confirm the sample copy and
   payloads.

## What the sample report card means

- `References` is the total number of references in the sample payload.
- `Verified` shows how many references already have provider evidence.
- `Review queue` shows how many items are still waiting on manual review.
- `AI-risk signals` counts issues flagged as potentially higher risk.
- `Warnings` and `Errors` show the current issue severity mix.

The sample JSON files under `sample_workspace/` are synthetic and read-only.
This demo does not connect to live providers or require API keys.
