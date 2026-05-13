from __future__ import annotations
from pathlib import Path

import streamlit as st

from demo_model import explanation_lines, journal_summary, load_sample_payloads, metric_rows

st.set_page_config(page_title="Sourceright demo", layout="wide")

sample_dir = Path(__file__).parent / "sample_workspace"
report, journal = load_sample_payloads(sample_dir)

st.title("Reference integrity report")
st.caption(f"{report['report_type']} / {report['schema_version']}")
st.info(
    "Synthetic sample data only. The demo does not call live providers and does not require API keys."
)

summary = report["summary"]
cols = st.columns(6)
for col, (label, value) in zip(cols, metric_rows(report)):
    col.metric(label, value)

left, right = st.columns([2, 1])
with left:
    st.subheader("Open issues")
    st.dataframe(report["issues"], use_container_width=True, hide_index=True)

with right:
    st.subheader("Journal screening")
    st.write(journal_summary(journal))
    st.write(journal["author_action_checklist"])

with st.expander("What this report card means", expanded=True):
    for line in explanation_lines():
        st.write(line)
