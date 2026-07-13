# Open journal and preprint software trial map

Inventory date: 2026-07-12. The scope is open-source, self-hostable software
with an identifiable upstream repository or contribution surface. This is a
maintained trial inventory, not a claim that every scholarly publishing
project worldwide has been enumerated.

| Platform | Role | Upstream | Sourceright trial shape | Decision | Track |
| --- | --- | --- | --- | --- | --- |
| Open Journal Systems | Journal workflow/publication | https://github.com/pkp/ojs | Read-only metadata/report plugin or sidecar | Trial | 98 |
| Janeway | Journal and preprint workflow/publication | https://github.com/openlibhums/janeway | Sidecar first; plugin after hook audit | Trial | 99; builds on 83-85 |
| Episciences | Overlay journals over repositories | https://github.com/CCSDForge/episciences | Repository/overlay metadata bridge | Trial | 100 |
| Kotahi | Journal/preprint workflow and production | https://github.com/okotahi/kotahi | Read-only workflow/report bridge | Trial with governance gate | 101 |
| PubPub | Community publishing | https://github.com/knowledgefutures/pubpub | API/import metadata bridge | Trial | 102 |
| Open Preprint Systems | Preprint workflow/publication | https://github.com/pkp/ops | Read-only metadata/report plugin or sidecar | Trial | 103 |
| InvenioRDM | Repository and preprint-capable deposit platform | https://github.com/inveniosoftware/invenio-app-rdm | REST/OAI metadata adapter | Trial | 104 |
| DSpace | Institutional repository/preprint deposit | https://github.com/DSpace/DSpace | REST/OAI/SWORD metadata adapter | Trial | 105 |
| EPrints | Open repository/preprint deposit | https://github.com/eprints/eprints3 | OAI/API metadata adapter | Trial | 106 |
| Samvera Hyrax | Repository engine suitable for preprints | https://github.com/samvera/hyrax | API/OAI metadata adapter | Trial with deployment gate | 107 |

## Reconnaissance-only or excluded surfaces

- OSF Preprints is open infrastructure, but the official product surface is a
  hosted service rather than a single self-hostable upstream application;
  retain it as a future API/read-only reconnaissance target.
- Orvium is presented as open-source journal and preprint software, but a
  stable official source/contribution repository was not established in this
  inventory; do not open an upstream PR until that is verified.
- Preprints.org, bioRxiv, medRxiv, and similar services are platforms, not
  open-source software targets for upstream code contributions.
- Lodel, Ambra, and other historical publishing systems require a separate
  maintenance/licensing audit before a trial track is promoted.

## Shared trial contract

Every trial must begin read-only and fixture-backed. It must identify the
platform revision, deployment surface, API/OAI/SWORD endpoint, metadata
mapping, rate/resource limits, provenance, redaction rules, and rollback
boundary. No trial may silently overwrite canonical CSL or verification
sidecars. Optional writes require a separate approved track after audit logs,
dry-run semantics, and permissions are proven.
