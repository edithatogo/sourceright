Proceed with Slice 3: demonstrator surfaces.

Add a static GitHub Pages demo and an optional Streamlit demo that render sample Sourceright artifacts.

Requirements:
- Demos use sample JSON only.
- No live provider calls.
- No API keys.
- No mutation of repository state.
- Use current report/schema/CLI output contracts where possible.
- Include README instructions.
- Add a GitHub Pages workflow only if it does not conflict with existing Pages or docs workflows.
- If there is an existing docs workflow, prefer a standalone demo directory and document deployment rather than modifying CI.

Suggested directories:

```text
github_pages_demo/
streamlit_app/
```

After adding, run JSON syntax checks and relevant tests.
