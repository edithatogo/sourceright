# Plugin Packaging And Supply-Chain Maturity Spec

## Goal

Harden plugin/module handling before external plugin publication.

## Policy

Do not use git submodules by default. Keep plugins in-repo while APIs are
moving. Split to separate repositories only when a plugin has an independent
release lifecycle, separate maintainers, or a host ecosystem requiring separate
packaging.

## SOTA Handling

- Plugin status taxonomy and evidence ledger.
- Signed or pinned plugin artifacts where installable.
- SBOM/provenance for release artifacts.
- Compatibility matrix by host and Sourceright version.
- Deprecation policy and migration notes.
- Network/auth/cache/sandbox policy per plugin.
