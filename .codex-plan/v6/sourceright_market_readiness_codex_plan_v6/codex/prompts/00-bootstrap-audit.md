# 00 — Bootstrap and audit

Inspect the current Sourceright repo and use the market-readiness plan pack only as guidance.

Do not modify source code except for `.codex-plan/` and `AGENTS.md` setup if this has not already been done.

Tasks:
1. Run:
   - `git status`
   - `git branch --show-current`
2. Inspect:
   - `Cargo.toml`
   - `README.md`
   - `conductor/tracks.md` if present
   - `src/`
   - `schemas/`
   - `plugins/`
   - `sourceright-bench/`
   - `docs/`, `docs-site/`, or docs source directories
   - `github_pages_demo/`
   - `streamlit_app/`
   - `mcp/`
3. Produce a concise audit:
   - current product surfaces;
   - benchmark status;
   - docs/demo status;
   - plugin/provider status;
   - market-readiness gaps;
   - safest next implementation slices.
4. Do not commit.
5. Wait for instruction before implementation.
