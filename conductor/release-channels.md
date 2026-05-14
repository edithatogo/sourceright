# Release Channels

Sourceright has separate release channels. A channel may be public only when
its channel-specific evidence exists.

| Channel | Artifact | Registry/site | Owning tracks | Required evidence |
| --- | --- | --- | --- | --- |
| Core CLI | Rust crate and native binaries | crates.io, GitHub Releases, later package managers | 25, 32, 33, 43 | Dry run, checksums, attestation, release notes, install smoke. |
| MCP | OCI/MCP metadata and local stdio runtime | Official MCP Registry, GHCR, Smithery later | 27, 56, 57 | `server.json`, image labels, registry listing, transcript smoke. |
| Zotero plugin | `.xpi` or adapter package | GitHub Release, Zotero forum/listing path | 58 | Package validation, install notes, disposable-library smoke. |
| OJS plugin | OJS/PKP plugin package | GitHub Release, PKP/OJS Plugin Gallery readiness | 60 | Package install, permission docs, fixture and optional test-instance smoke. |
| AI client configs | MCP client snippets and optional packaged configs | Claude Desktop local config, Codex CLI/MCP workflow guidance, GitHub Copilot coding-agent prep, generic MCP clients | 65 | Transcript smoke, client config examples, dry-run write proof, Copilot entitlement notes, no host-plugin overclaim. |
| VS Code extension | VSIX package | VS Code Marketplace, Open VSX, GitHub Release | 66, 69 | Explicitly deferred until package build, install smoke, Workspace Trust docs, diagnostics fixture, and accepted listing evidence exist. |
| Microsoft Word add-in | Office Add-in manifest and taskpane package | AppSource or sideload-only release | 67, 69 | Explicitly deferred until manifest validation, sideload smoke, range provenance, reversible write plan, and accepted listing evidence exist. |
| LibreOffice extension | `.oxt` package | LibreOffice Extensions site or GitHub Release | 68, 69 | Package build, install/uninstall smoke, Writer fixture, reversible write plan. |
| Streamlit demo | Streamlit app | Streamlit Community Cloud or local-only docs | 61 | Server smoke, synthetic data, privacy limits. |
| Static demo | GitHub Pages assets | GitHub Pages | 54 | Render/browser smoke and synthetic-data limits. |
