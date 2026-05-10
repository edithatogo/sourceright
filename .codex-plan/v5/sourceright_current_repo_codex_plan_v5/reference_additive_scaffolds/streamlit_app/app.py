from pathlib import Path
import json
import streamlit as st

st.set_page_config(page_title="Sourceright demo", layout="wide")
st.title("Sourceright reference report demo")
st.caption("Sample-data-only demonstrator. No provider calls and no API keys.")
path = Path(__file__).parent / "sample_workspace" / "reference-report.json"
report = json.loads(path.read_text())
st.metric("References", report.get("summary", {}).get("references", 0))
st.metric("Verified", report.get("summary", {}).get("verified", 0))
st.metric("Needs review", report.get("summary", {}).get("needs_review", 0))
st.json(report)
