Proceed with Slice 2: plugin manifest and registry, as documentation/config only.

Add:

```text
schemas/sourceright.plugin-manifest.schema.json
plugins/registry.toml
plugins/manifests/*.toml
docs/plugin-registry.md
docs/plugin-authoring.md
```

Important:
- Do not implement runtime dynamic loading yet.
- Do not refactor existing provider modules.
- Use plugin manifests to document capabilities, auth, cache/licence policy, and output contracts.
- Include categories for provider, citation-manager, journal, repository, legal, matcher, recency, relevance, extraction, export, and demo plugins.
- Mark paid providers such as Scopus, Web of Science, and Dimensions as BYO-key/licensed-data plugins.
- Keep tests deterministic and avoid live network calls.
- Summarize what remains needed for runtime plugin support.
