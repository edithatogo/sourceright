# Publishing Governance And Provenance Spec

## Goal

Make release and registry publication auditable, reproducible, and maintainable
with stronger dependency governance and provenance controls.

## Scope

- Track release provenance, checksums, and release notes as first-class outputs.
- Preserve manual registry publication gates while reducing operational risk.
- Keep dependency automation policy explicit and reviewable.
- Document the public release and registry sequence in one canonical place.

## Outputs

- Provenance and release-governance policy documents.
- Cleaner release/publish workflows with stronger preflight gates.
- Dependency automation rules that match the repo’s release policy.

## Boundaries

This track must not auto-publish from unreviewed changes. Publication remains gated by clean-tree checks, release validation, and explicit manual approval where registries require it.
