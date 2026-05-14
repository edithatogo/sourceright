# Sourceright Copilot Instructions

Use the current repository as the source of truth. Do not bulk-apply material
from `.codex-plan/`, do not overwrite existing Rust modules, and do not split
the crate into a workspace unless an issue explicitly asks for that after an
audit.

Preserve the project boundaries:

- `references.csl.json` is canonical clean bibliographic data.
- `references.verification.json` stores provider evidence, provenance,
  confidence, conflicts, and review state.
- `review-queue.jsonl` is derived operational review work.
- Provider data must not silently overwrite canonical CSL.
- Legal citations remain separate from academic CSL.
- Claim/source/provenance work must not assert claim truth.
- MCP write tools need stable read-only contracts, schemas, audit logs, and
  dry-run semantics before they write.

Prefer small, reviewable pull requests. For dependency or security work, keep
changes narrowly scoped to the vulnerable dependency, pinned action, pinned
image, workflow permission, or policy surface being remediated.

Use these checks when relevant:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo check --locked
cargo run --bin sourceright -- bench
```

For docs-site changes, also run:

```bash
cd docs-site
npm ci
npm run build
```

When touching GitHub Actions, keep permissions least-privilege, keep actions
pinned by full commit SHA, and avoid scheduled or notification-heavy workflows
unless the issue explicitly requests them.

## Conductor Track Workflow

Each implementation slice should reference a Conductor track by its ID
(e.g., `64-github-side-governance-additions`). Before editing files under a
track's owned paths, run `$conductor-review` (if available) or manually verify:

- The track spec and test matrix are up to date.
- Changes stay within declared owned paths.
- Evidence level in `conductor/evidence-ledger.json` accurately reflects what was done.

## Forbidden Claims

Sourceright code, docs, and PR metadata must never assert:

- "Production-ready institutional platform"
- "SOTA benchmarked performance"
- "Legal filing compliance system"
- "AI detector"
- "Fully verified" (without live-provider opt-in evidence)
- "Registry/platform supported" (without published registry packages)

Use accurate wording: "technical preview," "pilot-ready," "fixture-backed
regression benchmark," "deterministic benchmark scaffold."
