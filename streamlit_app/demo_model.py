from __future__ import annotations

import json
from pathlib import Path
from typing import Any


METRIC_KEYS = [
    ("References", "total_references"),
    ("Verified", "verified_references"),
    ("Review queue", "review_queue_count"),
    ("AI-risk signals", "ai_risk_issue_count"),
    ("Warnings", "warning_count"),
    ("Errors", "error_count"),
]


def load_sample_payloads(sample_dir: Path | None = None) -> tuple[dict[str, Any], dict[str, Any]]:
    root = sample_dir or Path(__file__).parent / "sample_workspace"
    report = json.loads((root / "reference-report.json").read_text(encoding="utf-8"))
    journal = json.loads((root / "journal-screening.json").read_text(encoding="utf-8"))
    return report, journal


def metric_rows(report: dict[str, Any]) -> list[tuple[str, int]]:
    summary = report["summary"]
    return [(label, int(summary[key])) for label, key in METRIC_KEYS]


def journal_summary(journal: dict[str, Any]) -> dict[str, str]:
    return {
        "submission_id": journal["submission_id"],
        "platform": journal["platform"],
        "status": journal["status"],
    }


def explanation_lines() -> list[str]:
    return [
        (
            "The sample report card summarizes reference health at a glance. The "
            "metrics show overall coverage, while the issue list points to the "
            "specific reference that still needs manual follow-up."
        ),
        (
            "In this fixture, one reference is verified and one remains in the "
            "review queue, which is why the page shows both a warning and an AI "
            "risk signal."
        ),
    ]
