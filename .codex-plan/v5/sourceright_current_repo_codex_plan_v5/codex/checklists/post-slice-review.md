# Post-slice review checklist

- [ ] Inspect `git diff --stat`.
- [ ] Inspect `git diff --name-status`.
- [ ] Confirm no existing modules were overwritten accidentally.
- [ ] Confirm no live network tests were added.
- [ ] Confirm provider data remains sidecar evidence.
- [ ] Confirm legal citations remain separate from CSL.
- [ ] Confirm docs match implementation.
- [ ] Run `cargo fmt --check`.
- [ ] Run `cargo clippy --all-targets -- -D warnings`.
- [ ] Run `cargo test`.
- [ ] Run `cargo check --locked`.
- [ ] Validate added JSON/YAML syntax.
- [ ] Decide whether to commit this slice before continuing.
