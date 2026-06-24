---
title: Plugin authoring
description: Guidance for authoring Sourceright plugin manifests.
---

Sourceright plugin manifests are contract documents first. They explain what a
plugin can do without implying that Sourceright will execute it by default.

## Manifest Rules

Every manifest should include:

- a stable `id` using a category prefix, for example `provider.crossref`;
- `plugin_api = "sourceright.plugin.v1"`;
- a `category` from the registry category list;
- `status`, using conservative values such as `core_normalizer`,
  `planned`, `planned_byo_key`, or `planned_adapter`;
- authentication and network requirements;
- cache, licence, and data-retention expectations;
- output contracts that match the schemas in `schemas/`.

Provider plugins must not overwrite canonical CSL fields silently. They should
emit candidates, conflicts, provenance, confidence inputs, or review issues
that can be stored in the verification sidecar or derived reports.

Citation-manager and export plugins should start with dry-run manifests and file
contracts before any direct sync implementation. Journal plugins should consume
or produce `sourceright.journal_screening.v1` reports without making claims
about author intent.

## Packaging Policy

Plugins must not be split into git submodules by default. Keep all plugin
manifests and immature adapters in this repository while the plugin API is
changing. This is the **no-submodule default**.

### Split Criteria

A plugin may be extracted into a separate repository or package only when it
meets **at least one** of these criteria:

1. **Independent release lifecycle** — the plugin needs its own version
   track, release cadence, and deprecation policy separate from Sourceright.
2. **Separate maintainers** — the plugin is maintained by a different team
   or organisation with its own review process.
3. **Host-specific packaging** — the plugin requires platform-specific
   distribution (e.g. a Zotero extension, OJS gallery package, VS Code
   extension) that cannot live in the Sourceright monorepo.
4. **Stable compatibility contract** — the plugin API surface has stabilised
   and the plugin declares a formal compatibility contract that must not
   break across Sourceright releases.

Before extracting, the plugin must have:

- A full evidence-ledger entry (see below).
- Provenance documentation for installable artifacts (see below).
- A deprecation plan for the monorepo manifest placeholder.

Track 63 maintains the supply-chain criteria that gate this decision.

## Evidence-Ledger Requirements

Every plugin tracked in the registry must satisfy the following evidence
requirements at its claimed `status` level:

| Requirement | `core_normalizer` / `core_exporter` | `planned_*` |
|---|---|---|
| **Fixture-backed tests** | Required | Not required (planned only) |
| **Documented install path** | Required in manifest or companion docs | Required at manifest level |
| **Explicit version/compatibility** | Declared via `plugin_api` and changelog | Declared via `plugin_api` |
| **Signed or pinned artifact** | Required for installable distributions | Documented expectation |
| **CI validation hook** | Manifest validation runs in CI | Manifest structure validated |

Any plugin without fixtures, docs, install path, version, and compatibility
declared must use a `planned_*` status. Moving from `planned_*` to
`core_normalizer` or `core_exporter` requires all evidence requirements to be
met first.

## Provenance Requirements

Installable plugins (those distributed outside the monorepo) must meet these
provenance expectations:

### Signing

- Distributable artifacts **must be signed** with a code-signing certificate
  or GPG key that is published and traceable to the maintainer.
- Signature verification instructions must be documented in the plugin's
  install path.
- Unsigned artifacts must be clearly labelled as development-only builds.

### Pinning

- Where a plugin depends on remote resources (schemas, binaries,
  containers), dependency references must be pinned to a specific
  immutable digest (SHA-256 or stronger) or a signed version tag.
- Range-based version pins are allowed only when the plugin declares a
  compatibility contract that is tested in CI.

### SBOM

- Every installable plugin release should produce an SBOM (SPDX or CycloneDX
  format) listing all runtime dependencies.
- The SBOM must be published alongside the release artifact.
- For monorepo plugins, the SBOM is generated at CI packaging time.

### Deprecation

- Deprecated plugins must document the replacement or migration path in
  their manifest under a `[deprecated]` section.
- A plugin with a `deprecated` notice retains its supply-chain commitments
  until the final planned removal date.

## Sandbox Policy

Every plugin manifest must explicitly declare its sandbox requirements. The
`[runtime]` section in the manifest controls:

| Field | Values | Meaning |
|---|---|---|
| `network` | `true` / `false` | Plugin requires outbound network access. |
| `default_enabled` | `true` / `false` | Plugin is enabled at startup without explicit user opt-in. |
| `live_tests_default` | `true` / `false` | Live API tests run in normal CI/test paths. |

The `[auth]` section declares authentication expectations:

| Field | Values | Meaning |
|---|---|---|
| `required` | `true` / `false` | Credentials must be configured before the plugin can operate. |
| `mode` | string | Authentication mode: `none`, `polite-mailto-recommended`, `public_api_or_byo_key`, `local_file`, `api_key`, `oauth`. |
| `env` | string[] | Environment variables that hold credentials. |

The `[cache]` section declares caching policy:

| Field | Values | Meaning |
|---|---|---|
| `policy` | string | Free-form cache behaviour description. |
| `retention` | string | `caller_controlled`, `session`, `persistent`, `no_cache`. |

**Rules:**

- Every plugin with `network = true` must document why network access is
  needed and what endpoints it contacts.
- Plugins with `default_enabled = true` require explicit security review
  and a written exception.
- No plugin may set `live_tests_default = true` without track-owner
  approval; the default for all plugins is `false`.
- Authentication credentials must never be hard-coded, checked into the
  repository, or stored in plugin source files.

## Status Taxonomy Reference

The registry in `plugins/registry.toml` defines these status values. See
[Plugin Registry](../plugin-registry/#status-matrix) for the full
market-readiness mapping.

| Status | Meaning |
|---|---|
| `core_normalizer` | Implemented core normalizer behaviour with fixture-backed tests. |
| `core_exporter` | Implemented export behaviour with fixture-backed tests. |
| `planned_public_api` | Public API target is described; implementation pending. |
| `planned_byo_key` | BYO-key or licensed-data target is described; implementation pending. |
| `planned_adapter` | Adapter integration is described; implementation pending. |
| `planned` | Concept is catalogued; no implementation-ready surface exists. |

## Overclaim Guards

Plugins with a `planned_*` or `planned` status must be described in public
communication as *roadmap items*, not as available functionality.

**Rules:**

- Status labels are implementation descriptors, not release promises.
- Do not describe a `planned_*` plugin as "available," "shipped," or
  "integrated" until its status moves to `core_normalizer` or
  `core_exporter`.
- Documentation and marketing copy for `planned_*` plugins must include
  a visible disclaimer: *"This integration is planned and not yet
  available. Specifications may change."*
- The evidence-ledger `allowed_claims` field for each track defines which
  specific statements are permitted about its plugins.

## Test Expectations

Default plugin tests should be fixture-backed and deterministic. Live API checks
can exist later, but they must be opt-in and excluded from normal local and CI
test paths.

Runtime loading validates manifests before they are exposed. A manifest that
fails registry validation should be fixed in the plugin files rather than
worked around in code.
