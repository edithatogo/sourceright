# Citation Manager Live Sync Spec

## Goal

Implement direct citation-manager sync beyond dry-run manifests, with Zotero as the first live API target.

## Scope

- Add a Zotero-first live sync target.
- Preserve preview and dry-run behavior before apply.
- Emit audit logs for all applied sync actions.
- Handle conflicts between local canonical CSL and remote library records.
- Require explicit apply before remote writes.

## Outputs

- Citation-manager sync contracts.
- Zotero preview and apply implementation.
- Sync audit logs.
- Conflict reports and resolution workflow.
- Documentation for credentials, preview, and apply.

## Boundaries

Live sync must not become the default export path. File exports remain deterministic and local.

Remote writes require explicit apply, audit logging, and conflict handling. Provider evidence must not silently overwrite canonical CSL or remote records.
