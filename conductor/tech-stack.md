# Sourceright Tech Stack

## Default architecture

- Core language: Rust.
- Primary binary: `sourceright`.
- Distribution targets: native CLI, MCP server mode, GitHub Releases, crates.io, and later thin adapter packages for AI/coding tools.
- Legacy import language: JavaScript from `humanizer-next`, retained under `legacy/` only.

## Rust direction

- Keep reference extraction orchestration, CSL JSON modelling, provider clients, verification sidecars, export generation, cache policy, and CLI/MCP execution in the Rust core.
- Prefer strongly typed internal models and deterministic outputs suitable for agent consumption.
- Keep provider integrations behind traits so tests can use recorded fixtures or mocks.
- Avoid making legal citations fit CSL when a separate model is needed.
- Keep claim/source/provenance work as a later model layer above the reference-verification core.

## Provider direction

- Academic providers start with Crossref and DOI resolution.
- Follow-on academic providers include DataCite, OpenAlex, PubMed/NCBI, and ORCID where they add reliable enrichment.
- Legal providers are deferred to a future track and should evaluate public legal citation APIs separately.
- Provider responses should never overwrite canonical records without recording provenance and conflicts.

## Adapter direction

- Use TypeScript or JavaScript only where specific plugin ecosystems require it.
- Adapters should shell out to or embed the Rust core instead of reimplementing verification logic.
- MCP should be served by the Rust core unless a later planning track proves a different split is necessary.

## Public project direction

- Public repository: `edithatogo/sourceright`.
- License: MIT OR Apache-2.0.
- Documentation site: GitHub Pages built from `docs/`.
- CI/CD: GitHub Actions for formatting, linting, tests, docs, security scanning, release dry-runs, and tagged releases.
- Release targets: crates.io package, GitHub Release binaries, checksums, SBOM/provenance artifacts, and later package-manager formulas/manifests.

## Current scaffold

- `Cargo.toml` defines the initial Rust package.
- `src/main.rs` provides a minimal CLI placeholder with planned `mcp` mode.
- `legacy/humanizer-next/` preserves the imported reference workflow.
