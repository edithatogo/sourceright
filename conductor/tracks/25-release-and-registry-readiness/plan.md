# Release And Registry Readiness Plan

1. Audit current package metadata and release workflows.
2. Add crates.io/docs.rs readiness metadata.
3. Add `cargo publish --dry-run` to the release gate.
4. Add a manual, environment-gated crates.io publish workflow.
5. Document the release sequence and post-release verification.
6. Run package validation locally before closeout.
