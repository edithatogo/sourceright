# Sourceright market-readiness Codex plan v6

This pack is a **planning and prompt kit**, not a bulk code overlay.

It is current-repo-aware as of the public Sourceright repo state reviewed on 2026-05-12:
- the repo has moved beyond the original scaffold;
- the current package is `sourceright` v0.1.20;
- the core now includes CSL handling, verification sidecars, cleaning, conflict handling, reconciliation, reporting, exports, journal/legal/provenance surfaces, schemas, plugins, demo directories, and a deterministic benchmark scaffold;
- the benchmark exists, but is still described as `runner_status: scaffold_only` in `sourceright-bench/tasks.yaml`.

## Main conclusion

Sourceright is ready for a **technical preview / pilot** push, not yet a full “production-ready institutional product” launch.

The next work should focus on:
1. benchmark maturity and truthful benchmark claims;
2. documentation depth and worked examples;
3. Rust API/rustdoc coverage;
4. public demo/landing-page polish;
5. plugin/provider status clarity;
6. case-study fixtures;
7. read-only MCP hardening and examples;
8. launch checklist and positioning.

## How to use this pack

Do not copy every file into the repository root.

Recommended:
1. Extract this ZIP under `.codex-plan/v6/`.
2. Copy or merge `AGENTS.md.template` into the repo root as `AGENTS.md`.
3. Use the prompts in `codex/prompts/` one at a time.
4. Let Codex inspect the current repo before implementing anything.
5. Keep every implementation slice small and reviewable.

The all-in-one bootstrap prompt is in:

```text
USE_THIS_PROMPT.md
```

The preferred sequence is:

```text
codex/prompts/00-bootstrap-audit.md
codex/prompts/01-baseline-quality-gates.md
codex/prompts/02-benchmark-maturity.md
codex/prompts/03-docs-and-rustdoc.md
codex/prompts/04-demo-and-landing-page.md
codex/prompts/05-plugin-provider-status.md
codex/prompts/06-case-studies.md
codex/prompts/07-mcp-readonly-hardening.md
codex/prompts/08-launch-packaging.md
codex/prompts/09-final-review.md
```

## Important non-goals

Codex should not:
- bulk-apply any older overlay;
- split the single crate into a workspace unless explicitly asked later;
- add live-network benchmark requirements;
- claim SOTA performance before external comparative benchmarks exist;
- imply paid provider integrations are implemented if they are only planned or BYO-key;
- silently overwrite canonical CSL with provider metadata;
- conflate legal citations with CSL bibliography records.
