Proceed with Slice 7: citation-manager profiles and sync manifests.

Because Sourceright already has export surfaces, start with file/API profile documentation and dry-run sync contracts before direct sync implementation.

Potential additive files:

```text
docs/citation-manager-integrations.md
schemas/sourceright.sync-manifest.schema.json
examples/citation-manager-profiles/zotero.yaml
examples/citation-manager-profiles/mendeley.yaml
examples/citation-manager-profiles/endnote.yaml
examples/citation-manager-profiles/papers-readcube.yaml
examples/citation-manager-profiles/jabref.yaml
examples/citation-manager-profiles/revman.yaml
examples/citation-manager-profiles/rayyan.yaml
examples/citation-manager-profiles/covidence.yaml
```

Requirements:
- Default direct sync concepts to dry-run.
- Separate file-format adapters from live API adapters.
- Include auth/privacy/cache notes.
- Preserve canonical CSL and sidecar evidence boundaries.
- Do not implement live sync unless explicitly asked later.
