from __future__ import annotations

import json
from pathlib import Path

import streamlit as st

st.set_page_config(page_title="Sourceright demo", layout="wide")

sample_dir = Path(__file__).parent / "sample_workspace"
report = json.loads((sample_dir / "reference-report.json").read_text(encoding="utf-8"))
journal = json.loads((sample_dir / "journal-screening.json").read_text(encoding="utf-8"))

st.title("Reference integrity report")
st.caption(f"{report['report_type']} / {report['schema_version']}")

summary = report["summary"]
cols = st.columns(6)
cols[0].metric("References", summary["total_references"])
cols[1].metric("Verified", summary["verified_references"])
cols[2].metric("Review queue", summary["review_queue_count"])
cols[3].metric("AI-risk signals", summary["ai_risk_issue_count"])
cols[4].metric("Warnings", summary["warning_count"])
cols[5].metric("Errors", summary["error_count"])

left, right = st.columns([2, 1])
with left:
    st.subheader("Open issues")
    st.dataframe(report["issues"], use_container_width=True, hide_index=True)

with right:
    st.subheader("Journal screening")
    st.write(
        {
            "submission_id": journal["submission_id"],
            "platform": journal["platform"],
            "status": journal["status"],
        }
    )
    st.write(journal["author_action_checklist"])
