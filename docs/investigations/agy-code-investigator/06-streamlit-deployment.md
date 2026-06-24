# Investigation Report: Streamlit App Structure and Deployment Surface

This report presents a comprehensive review of the Streamlit website/app and its deployment configuration within the **Sourceright** repository.

---

## 1. App Structure & Logic

The Streamlit app acts as a **synthetic-data demonstrator** showing reference health metrics and journal screening workflows. The codebase is clean, modular, and separates data loading from visual rendering.

### Codebase Organization
- **App Entry Point**: [app.py](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/sourceright/streamlit_app/app.py) handles the page configuration, component layout, and rendering loop.
- **Data Model & Processing**: [demo_model.py](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/sourceright/streamlit_app/demo_model.py) manages the internal metrics keys, static descriptions, and file-parsing helpers.
- **Synthetic Data Payloads**: Located under the `sample_workspace/` directory:
  - [reference-report.json](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/sourceright/streamlit_app/sample_workspace/reference-report.json): Standard schemas for reference health checks.
  - [journal-screening.json](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/sourceright/streamlit_app/sample_workspace/journal-screening.json): Submission-level screening checklist metadata.

### Data Loading Flow
In [app.py:L10-11](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/sourceright/streamlit_app/app.py#L10-L11), the app dynamically resolves the `sample_workspace` folder relative to the script location and passes it to the `load_sample_payloads` function:
```python
sample_dir = Path(__file__).parent / "sample_workspace"
report, journal = load_sample_payloads(sample_dir)
```
The implementation in [demo_model.py:L18-22](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/sourceright/streamlit_app/demo_model.py#L18-L22) reads the files synchronously with UTF-8 encoding, preventing any remote database dependencies:
```python
def load_sample_payloads(sample_dir: Path | None = None) -> tuple[dict[str, Any], dict[str, Any]]:
    root = sample_dir or Path(__file__).parent / "sample_workspace"
    report = json.loads((root / "reference-report.json").read_text(encoding="utf-8"))
    journal = json.loads((root / "journal-screening.json").read_text(encoding="utf-8"))
    return report, journal
```

---

## 2. UX & Layout Design

The dashboard uses Streamlit's built-in grid components to build a dashboard layout tailored to high-density medical/scientific reviews.

- **Wide Layout Configuration**: Configured at [app.py:L8](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/sourceright/streamlit_app/app.py#L8) to stretch elements across the monitor view:
  ```python
  st.set_page_config(page_title="Sourceright demo", layout="wide")
  ```
- **Information Banners**: Appended at [app.py:L15-17](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/sourceright/streamlit_app/app.py#L15-L17) using `st.info` to communicate that the data is synthetic.
- **Top Metrics Row**: An 6-way split columns layout displays high-level counts ([app.py:L20-22](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/sourceright/streamlit_app/app.py#L20-L22)):
  ```python
  cols = st.columns(6)
  for col, (label, value) in zip(cols, metric_rows(report)):
      col.metric(label, value)
  ```
- **Asymmetric Content Columns**: Split into `2/3` and `1/3` sections ([app.py:L24](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/sourceright/streamlit_app/app.py#L24)):
  - **Left (2/3 Width)**: Dedicated to listing the detailed reference issues table via `st.dataframe(...)` with index numbers hidden for a cleaner presentation ([app.py:L26-27](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/sourceright/streamlit_app/app.py#L26-L27)).
  - **Right (1/3 Width)**: Holds metadata like submission ID, platform info, status, and the Checklist items ([app.py:L29-32](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/sourceright/streamlit_app/app.py#L29-L32)).
- **Help Section**: A collapsible expander component ([app.py:L34-36](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/sourceright/streamlit_app/app.py#L34-L36)) details what the report card signifies.

---

## 3. Tabs & Interactivity

- **Tabs (`st.tabs()`)**: The current iteration does not leverage tab navigation. It organizes all information side-by-side or stacked in modules.
- **Interactive Elements**:
  - **Dataframes**: The `st.dataframe` component is interactive out-of-the-box, allowing sorting by columns (e.g. sorting severity or category), search lookups, cell selection, and full-screen view expansions.
  - **Expanders**: The explanations block relies on `st.expander` initialized as `expanded=True` for manual user toggling.

---

## 4. Smoke Testing Framework

The repository has a custom script designed to launch and verify the integrity of the Streamlit application runtime: [server_smoke.py](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/sourceright/streamlit_app/server_smoke.py).

### How the Smoke Test Works:
1. **Gate Guard**: Exits cleanly unless the environment variable `SOURCERIGHT_DEMO_SERVER_SMOKE=1` is present ([server_smoke.py:L23-24](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/sourceright/streamlit_app/server_smoke.py#L23-L24)).
2. **Dynamic Port Allocation**: To prevent collisions on build runners, it assigns a random, unused TCP port on the host interface ([server_smoke.py:L32-35](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/sourceright/streamlit_app/server_smoke.py#L32-L35)):
   ```python
   def _free_port() -> int:
       with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
           sock.bind(("127.0.0.1", 0))
           return int(sock.getsockname()[1])
   ```
3. **Headless Subprocess Execution**: Spawns the Streamlit server programmatically with telemetry analytics turned off ([server_smoke.py:L40-58](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/sourceright/streamlit_app/server_smoke.py#L40-L58)).
4. **HTTP Health Probing**: Polling logic uses `http.client.HTTPConnection` to issue requests to the landing page (`/`). It asserts that:
   - Response status matches `200`.
   - Output html body includes the term `"streamlit"`.
   - Maximum waiting window is capped at 30 seconds before asserting a timeout error.
5. **Process Lifecycle Hardening**: Leverages `try/finally` boundaries to terminate and kill the daemon process to prevent orphan tasks ([server_smoke.py:L87-92](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/sourceright/streamlit_app/server_smoke.py#L87-L92)).

---

## 5. Deployment Configuration & URL Contract

Details are documented in [DEPLOY.md](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/sourceright/streamlit_app/DEPLOY.md).

### Cloud Target: Streamlit Community Cloud
- **Launch path**: Configured to load `streamlit_app/app.py`.
- **Environment variables**: Zero secrets or server credentials required since all datasets are local, read-only JSON resources.
- **Resource Allocations**: Capped at **1 GB RAM** and **10 GB storage**, with active instances sleeping after 15 minutes of inactivity.

---

## 6. Accessibility & Performance Review

- **Performance**: Very high response times and fast loads because there are no API calls or remote SQL lookups. Everything runs from flat JSON memory arrays.
- **Accessibility**: Inherits the responsive CSS grids from Streamlit. Custom interactive tags like dataframes utilize standard browser accessibility attributes, although the page does not explicitly package custom ARIA landmarks or overrides.

---

## 7. Next Steps to Make it "Bleeding Edge"

To move this from a mock prototype into a production-grade visual application, the following enhancement checklist is proposed:

1. **Dynamic Citation Verification**: Integrate live REST resolvers (Crossref, DOI API, PubMed E-utilities) to run real-time checks on user-input citations.
2. **File Import Support**: Use `st.file_uploader` to accept `.ris`, `.bib`, or PDF uploads, running parsing and integrity checks on-the-fly.
3. **Advanced Visualizations**: Embed interactive visual graphs (e.g. citation network clusters, journals risk heatmaps) using Plotly or Pyvis.
4. **AI-Assisted Co-Pilot**: Introduce a chat interface (`st.chat_input`) powered by a RAG engine to let editors query source papers directly.
5. **OJS Integration & OAuth**: Add multi-tenant sign-in flow (Auth0/SSO) allowing editors to publish validated citation logs back to Open Journal Systems (OJS).
