Proceed with Slice 6: provider expansion only if it fits the current provider architecture.

First inspect existing provider modules and tests.

Rules:
- Do not add live network tests.
- Do not introduce mandatory API keys.
- Do not let provider metadata silently overwrite canonical CSL.
- Normalize provider evidence into sidecar candidates.
- Preserve conflicts and uncertainty.
- Use fixtures and deterministic tests.

Possible provider additions or docs/manifests:
- Crossref Retraction Watch/Crossmark-style status evidence
- Unpaywall
- OpenCitations
- arXiv
- Europe PMC
- Zenodo/OSF/Figshare/Dataverse repository references
- Dimensions/Scopus/Web of Science as BYO-key licensed plugins only

If implementation is too large, add provider fixture plans and manifests only.
