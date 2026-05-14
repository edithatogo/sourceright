# Release Channels

Sourceright has separate release channels. A channel may be public only when
its channel-specific evidence exists.

| Channel | Artifact | Registry/site | Owning tracks | Required evidence |
| --- | --- | --- | --- | --- |
| Core CLI | Rust crate and native binaries | crates.io, GitHub Releases, later package managers | 25, 32, 33, 43 | Dry run, checksums, attestation, release notes, install smoke. |
| MCP | OCI/MCP metadata and local stdio runtime | Official MCP Registry, GHCR, Smithery later | 27, 56, 57 | `server.json`, image labels, registry listing, transcript smoke. |
| Zotero plugin | `.xpi` or adapter package | GitHub Release, Zotero forum/listing path | 58 | Package validation, install notes, disposable-library smoke. |
| OJS plugin | OJS/PKP plugin package | GitHub Release, PKP/OJS Plugin Gallery readiness | 60 | Package install, permission docs, fixture and optional test-instance smoke. |
| Streamlit demo | Streamlit app | Streamlit Community Cloud or local-only docs | 61 | Server smoke, synthetic data, privacy limits. |
| Static demo | GitHub Pages assets | GitHub Pages | 54 | Render/browser smoke and synthetic-data limits. |
