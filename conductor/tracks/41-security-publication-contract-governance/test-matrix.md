# Security, Publication, and Contract Governance Test Matrix

| Area | Acceptance |
| --- | --- |
| Dependency alerts | Patched docs-site dependencies report zero local npm vulnerabilities. |
| Renovate | Routine patch/minor/pin/digest updates are grouped, scheduled, and eligible for safe automerge; majors remain manual. |
| GitHub Actions | Marketplace action references are pinned by full commit SHA unless a local exception is documented. |
| Feature contract | The canonical matrix lists MoSCoW status, evidence, contract obligations, tests, and exclusions. |
| Design document | Architecture diagrams cover data boundaries, CLI/MCP, plugins/providers, OJS, Zotero, and release flow. |
| Publication inventory | Accepted listings are separated from prepared or unverified registry surfaces. |
| External tests | OJS, CLI, MCP, citation-manager, and registry smoke tests are recommended as fixture-backed or opt-in live tests. |
| Provider runtime controls | Live-provider timeout, minimum interval, retry, and cache settings default to conservative values. |
| Demo smoke tests | GitHub Pages and Streamlit demo server smoke scripts are opt-in and fail in CI only when explicitly enabled or required. |
