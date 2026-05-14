# Publication Registry Completion Spec

## Goal

Turn publication status from a one-off audit into a repeatable registry
completion process that separates accepted, submitted, prepared, deferred, and
not-applicable surfaces.

## Scope

- Verify accepted listings for GitHub Release, crates.io, docs.rs, official MCP
  Registry, and GHCR evidence.
- Complete or explicitly defer Glama and Smithery.
- Decide whether package-manager registries are useful now: Homebrew, Scoop,
  Chocolatey, winget, npm launcher, PyPI launcher.
- Record version, URL, evidence date, and blocking requirements.

## Out Of Scope

- Publishing wrappers that reimplement the Rust core.
- Claiming registry acceptance from prepared metadata alone.

## Parallelization

- Subagent A: Rust/package registries.
- Subagent B: MCP/AI-tool registries.
- Subagent C: package-manager feasibility.
- Subagent D: docs/release evidence parity.
