# Prompt to paste into Codex

Run this from PowerShell first:

```powershell
cd "C:\Users\60217257\OneDrive - Flinders\repos\sourceright"
codex --add-dir "C:\Users\60217257\OneDrive - NSW Health Department\Downloads"
```

Then paste this into Codex:

```text
You are working in this repo:

C:\Users\60217257\OneDrive - Flinders\repos\sourceright

The market-readiness Codex plan ZIP is expected to be here:

C:\Users\60217257\OneDrive - NSW Health Department\Downloads\sourceright_market_readiness_codex_plan_v6.zip

Task: bring the planning pack into the repo safely, then use it to audit the current repo and create an implementation plan. This is a planning pack, not a code overlay.

Important:
- Do not bulk-apply the ZIP into the project root.
- Do not overwrite existing Rust modules.
- Do not split the single crate into a workspace.
- Treat the current repository as the source of truth.
- Use the pack only as guidance for staged market-readiness work.
- Do not commit.

Steps:

1. Check the current state:
   - git status
   - git branch --show-current

2. If the working tree has uncommitted user changes, stop and summarize them before making changes.

3. Create:
   - .codex-plan/
   - .codex-plan/v6/

4. Copy the ZIP from Downloads to:
   - .codex-plan/sourceright_market_readiness_codex_plan_v6.zip

   Prefer copy over destructive move.

5. Extract the ZIP into:
   - .codex-plan/v6/

6. Locate:
   - .codex-plan/v6/sourceright_market_readiness_codex_plan_v6/AGENTS.md.template

7. If AGENTS.md does not exist, copy AGENTS.md.template to AGENTS.md.
   If AGENTS.md already exists:
   - make AGENTS.md.backup
   - merge the new guidance carefully
   - preserve existing repo-specific guidance
   - avoid duplicate sections

8. Read:
   - .codex-plan/v6/sourceright_market_readiness_codex_plan_v6/README.md
   - .codex-plan/v6/sourceright_market_readiness_codex_plan_v6/PLAN.md
   - .codex-plan/v6/sourceright_market_readiness_codex_plan_v6/MARKET_READINESS.md
   - .codex-plan/v6/sourceright_market_readiness_codex_plan_v6/BENCHMARK_MATURITY.md
   - .codex-plan/v6/sourceright_market_readiness_codex_plan_v6/codex/prompts/00-bootstrap-audit.md

9. Now perform inspection only:
   - inspect Cargo.toml
   - inspect README.md
   - inspect conductor/tracks.md if present
   - inspect docs/ and docs-site/ if present
   - inspect src/
   - inspect schemas/
   - inspect plugins/
   - inspect sourceright-bench/
   - inspect github_pages_demo/ and streamlit_app/ if present
   - inspect mcp/ and server.json if present

10. Produce a concise audit:
    - what is already implemented;
    - whether benchmark is scaffold, runnable, or externally comparable;
    - whether documentation is enough for technical preview marketing;
    - gaps before a serious launch;
    - recommended implementation slices;
    - what should not be done yet.

11. Do not modify source code beyond:
    - creating .codex-plan/
    - copying/extracting the plan ZIP
    - creating or carefully merging AGENTS.md

12. Wait for my instruction before proceeding to baseline checks or implementation slices.
```
