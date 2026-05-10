# Citation Manager Integrations

Sourceright already exports YAML, XML, RIS, ENW, and BibLaTeX. Citation-manager
integration should build on those file surfaces first, then add dry-run API sync
contracts before any live mutation.

Profiles under `examples/citation-manager-profiles/` describe expected adapter
behavior for:

- Zotero
- Mendeley
- EndNote
- Papers/ReadCube
- JabRef
- RevMan
- Rayyan
- Covidence

## Boundaries

Canonical CSL remains in `references.csl.json`. Provider evidence, review
decisions, sync decisions, and conflicts stay outside CSL in sidecars or sync
manifests.

Direct API adapters should default to dry-run, declare auth requirements, avoid
credential storage, and cache only caller-approved metadata. File-format
adapters should record exactly which files they would write or import.

The initial sync manifest schema is `sourceright.sync_manifest.v1`.

## Zotero Preview And Apply

The first live sync target is Zotero. Sourceright models Zotero sync as a
preview-first contract:

- preview plans create, update, skip, and conflict actions without writes;
- explicit apply is required before any remote mutation;
- applied runs append an audit log;
- ambiguous updates are reported as conflicts rather than silently overwritten.

The CLI surface is:

```text
sourceright citation-sync [--preview|--apply] [--remote-fixture <remote.json>] [--audit-log <audit.jsonl>] [.sourceright-directory]
```

By default the command reads `.sourceright`, runs in preview mode, and prints
`sourceright.citation_sync.v1` JSON. Live Zotero sync is disabled unless the
caller supplies the relevant `SOURCERIGHT_ZOTERO_*` environment variables.
