# Release And Registry Readiness Spec

## Goal

Prepare Sourceright for public crate and release distribution without performing irreversible publication from a dirty or unreviewed tree.

## Scope

- Make crate metadata suitable for crates.io and docs.rs.
- Keep GitHub Releases as the binary distribution surface.
- Add dry-run publication gates before any real registry publish.
- Document the exact manual and CI release sequence.

## Outputs

- Cargo package metadata and package contents policy.
- Release dry-run checks for crate packaging and publication.
- Manual publication workflow for crates.io.
- Updated publishing documentation.

## Boundaries

This track must not publish to crates.io, GitHub Releases, or package-manager registries until the working tree is committed, pushed, and green in GitHub Actions.
