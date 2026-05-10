# Updated Codex plan for the current Sourceright repository

## Core change from the previous plan

The previous ZIP was an implementation overlay. The current plan is different: the repository now appears to have substantial implemented functionality, so Codex should **not** bulk-apply stale scaffolding.

Treat the current repository as source of truth and add missing external surfaces around it.

## Updated priorities

### Phase 0: audit only

No file changes. Inspect `Cargo.toml`, `README.md`, `conductor/tracks.md`, `docs/`, `src/`, and `.github/`.

### Phase 1: baseline checks

Run:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo check --locked
```

Only fix baseline issues after explaining them.

### Phase 2: schemas and examples

Add external JSON/YAML contracts that reflect the current Rust models and docs. Do not invent incompatible fields.

### Phase 3: plugin manifests and registry

Add plugin metadata and docs only. Do not implement runtime dynamic loading yet.

### Phase 4: demos

Add a static GitHub Pages demo and optional Streamlit app that consume sample Sourceright artifacts. They must not call live providers or require API keys.

### Phase 5: benchmark harness

Add `sourceright-bench/` as a benchmark scaffold, not a Rust workspace crate yet.

### Phase 6: policy, style, and recency

Inspect current style/policy/journal modules first. Add minimal compatible policy files, schemas, docs, and possibly small Rust extensions.

### Phase 7: provider expansion

Provider expansion should be fixture-backed and deterministic: no live network tests, no canonical CSL overwrite, sidecar evidence only, and provider conflicts preserved.

### Phase 8: citation-manager profiles

Because exports already exist, add profiles and sync manifests before direct API sync.

### Phase 9: MCP server

Add contracts first. Read-only server first. No write tools until schemas, audit logs, and dry-run semantics are stable.

### Phase 10: modularisation later

Do not split the crate now. Evolve modularity in this order: keep single crate; add plugin manifest contracts; add internal plugin traits if useful; add optional feature flags; split into workspace only after APIs stabilize.
