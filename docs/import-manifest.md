# Import manifest

Source repo: `..\humanizer-next`

Destination: `legacy/humanizer-next/`

## Imported surfaces

- `experiments/citation_ref_manager/`
- `skills/humanizer-cite/`
- `conductor/tracks/archive/citation_ref_20260216/`
- `conductor/tracks/archive/source-verification_20260131/`
- `docs/citation-manager-boundary.md`
- `docs/SOURCE_REFRESH_COMMANDS.md`
- `scripts/research/citation-normalize.js`
- `src/references.json`
- `src/research_references.md`
- `src/ai_features_sources_table.md`
- `test/sample-citations.json`
- `archive/sources_manifest.json`
- `archive/sources/`

## Import notes

- The import deliberately keeps reference and citation material broad so pruning can happen after Conductor planning.
- OneDrive hydration caused timeouts while copying `experiments/citation_ref_manager/integration_test.js`, but a later copy pass completed and the file is present in the imported tree.
- Dependency and build folders were not copied: `.git`, `node_modules`, `dist`, package locks, and local cache directories remain out of scope.
