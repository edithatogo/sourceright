from __future__ import annotations

import importlib
import sys
import types
import unittest
from pathlib import Path

from .demo_model import explanation_lines, journal_summary, load_sample_payloads, metric_rows


class _FakeColumn:
    def __init__(self, calls: list[tuple[str, object]]) -> None:
        self.calls = calls

    def __enter__(self) -> "_FakeColumn":
        return self

    def __exit__(self, *_args: object) -> None:
        return None

    def metric(self, label: str, value: int) -> None:
        self.calls.append(("metric", (label, value)))


class _FakeExpander(_FakeColumn):
    pass


class _FakeStreamlit(types.ModuleType):
    def __init__(self) -> None:
        super().__init__("streamlit")
        self.calls: list[tuple[str, object]] = []

    def set_page_config(self, **kwargs: object) -> None:
        self.calls.append(("set_page_config", kwargs))

    def title(self, value: str) -> None:
        self.calls.append(("title", value))

    def caption(self, value: str) -> None:
        self.calls.append(("caption", value))

    def info(self, value: str) -> None:
        self.calls.append(("info", value))

    def columns(self, spec: object) -> list[_FakeColumn]:
        count = spec if isinstance(spec, int) else len(spec)  # type: ignore[arg-type]
        self.calls.append(("columns", spec))
        return [_FakeColumn(self.calls) for _ in range(int(count))]

    def subheader(self, value: str) -> None:
        self.calls.append(("subheader", value))

    def dataframe(self, value: object, **kwargs: object) -> None:
        self.calls.append(("dataframe", {"value": value, **kwargs}))

    def write(self, value: object) -> None:
        self.calls.append(("write", value))

    def expander(self, label: str, expanded: bool = False) -> _FakeExpander:
        self.calls.append(("expander", (label, expanded)))
        return _FakeExpander(self.calls)


class StreamlitDemoModelTest(unittest.TestCase):
    def test_sample_payloads_drive_expected_report_card(self) -> None:
        report, journal = load_sample_payloads()

        self.assertEqual(report["schema_version"], "sourceright.reference_report.v1")
        self.assertEqual(journal["schema_version"], "sourceright.journal_screening.v1")
        self.assertIn(("References", 2), metric_rows(report))
        self.assertIn(("Review queue", 1), metric_rows(report))
        self.assertEqual(
            journal_summary(journal),
            {
                "submission_id": "DEMO-001",
                "platform": "ojs",
                "status": "screened_with_warnings",
            },
        )
        self.assertTrue(any("manual follow-up" in line for line in explanation_lines()))

    def test_app_import_renders_expected_streamlit_calls(self) -> None:
        fake_streamlit = _FakeStreamlit()
        previous_streamlit = sys.modules.get("streamlit")
        previous_app = sys.modules.pop("streamlit_app.app", None)
        sys.modules["streamlit"] = fake_streamlit
        sys.path.insert(0, str(Path(__file__).parent))
        try:
            importlib.import_module("streamlit_app.app")
        finally:
            sys.path.pop(0)
            sys.modules.pop("streamlit_app.app", None)
            if previous_app is not None:
                sys.modules["streamlit_app.app"] = previous_app
            if previous_streamlit is not None:
                sys.modules["streamlit"] = previous_streamlit
            else:
                sys.modules.pop("streamlit", None)

        self.assertIn(("title", "Reference integrity report"), fake_streamlit.calls)
        self.assertIn(
            ("caption", "reference_integrity / sourceright.reference_report.v1"),
            fake_streamlit.calls,
        )
        self.assertIn(("metric", ("References", 2)), fake_streamlit.calls)
        self.assertIn(("metric", ("Review queue", 1)), fake_streamlit.calls)
        self.assertIn(("subheader", "Open issues"), fake_streamlit.calls)
        self.assertIn(("subheader", "Journal screening"), fake_streamlit.calls)
        self.assertIn(
            (
                "write",
                {
                    "submission_id": "DEMO-001",
                    "platform": "ojs",
                    "status": "screened_with_warnings",
                },
            ),
            fake_streamlit.calls,
        )


if __name__ == "__main__":
    unittest.main()
