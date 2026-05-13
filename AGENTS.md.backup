# Sourceright Codex instructions

The live Sourceright repository has advanced beyond the previous overlay plan. Treat the current repository as the source of truth.

Do not bulk-apply the old overlay ZIP.
Do not overwrite existing Rust modules.
Do not split the crate into a workspace unless explicitly asked after an audit.
Prefer additive changes: schemas, docs, examples, benchmark fixtures, demo apps, plugin manifests, validation scripts, and policy files.

Preserve these architectural boundaries:

- `references.csl.json` is canonical clean bibliographic data.
- `references.verification.json` stores provider evidence, provenance, confidence, conflicts, and review state.
- `review-queue.jsonl` is derived operational review work.
- Provider data must never silently overwrite canonical CSL.
- Legal citations remain separate from academic CSL.
- Claim/source/provenance work should not assert claim truth.
- MCP write tools must wait until read-only server contracts, schemas, audit logs, and dry-run semantics are stable.

Before modifying files:

1. Inspect the current repo.
2. Run existing checks where possible.
3. Use any older overlay only as design reference.
4. Produce a slice-by-slice implementation plan.

After each slice, run as many as apply:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo check --locked
```

If new validation scripts are added, run them too.

Do not commit automatically. Summarize changed files, checks run, failures, uncertainty, and deferred work.
