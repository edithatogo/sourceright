# Release Channels

Sourceright has separate release channels. A channel may be public only when
its channel-specific evidence exists.

| Channel | Artifact | Registry/site | Owning tracks | Required evidence |
| --- | --- | --- | --- | --- |
| Core CLI | Rust crate and native binaries | crates.io, GitHub Releases, later package managers | 25, 32, 33, 43, 70 | Dry run, checksums, attestation, release notes, install smoke, and release-surface refresh before publication wording changes. |
| MCP | OCI/MCP metadata and local stdio runtime | Official MCP Registry, GHCR, Smithery later | 27, 56, 57, 70 | `server.json`, image labels, registry listing, transcript smoke, and release-surface refresh before registry wording changes. |
| Zotero plugin | `.xpi` or adapter package | GitHub Release, Zotero forum/listing path | 58, 69, 70 | Prepared as CLI/Web API adapter; no `.xpi` or Plugin Gallery listing claimed until package validation, install notes, disposable-library smoke, accepted listing evidence, and release-surface refresh exist. |
| OJS plugin | OJS/PKP plugin package | GitHub Release, PKP/OJS Plugin Gallery readiness | 60, 69, 70 | Prepared as generic-plugin source skeleton; PKP Gallery acceptance requires package install, permission docs, fixture and optional test-instance smoke, accepted listing evidence, and release-surface refresh. |
| arXiv upstream modules | Upstream issue/PR or external-integration packet | `arXiv/submit-ce` and `arXiv/arxiv-submission-core` | 71, 78, 79, 80, 81 | Requirements reconnaissance, current/legacy hardening, fixture breadth, stability/security review, no-writeback proof, approval gate, and issue/PR evidence before submission wording changes. |
| AI client configs | MCP client snippets and optional packaged configs | Claude Desktop local config, Codex CLI/MCP workflow guidance, GitHub Copilot coding-agent prep, generic MCP clients | 65 | Transcript smoke, client config examples, dry-run write proof, Copilot entitlement notes, no host-plugin overclaim. |
| AI client extensions | Host-specific packages if supported | Claude Cowork, Codex app, GitHub Copilot, Gemini CLI extensions, Qwen CLI extensions | 65, 76 | Official requirements search, no-package decisions where needed, package manifest validation, install/client smoke, and listing evidence before plugin claims. |
| VS Code extension | VSIX package | VS Code Marketplace, Open VSX, GitHub Release | 66, 69, 70 | Explicitly deferred until package build, install smoke, Workspace Trust docs, diagnostics fixture, accepted listing evidence, and release-surface refresh exist. |
| Microsoft Word add-in | Office Add-in manifest and taskpane package | AppSource or sideload-only release | 67, 69, 70 | Explicitly deferred until manifest validation, sideload smoke, range provenance, reversible write plan, accepted listing evidence, and release-surface refresh exist. |
| LibreOffice extension | `.oxt` package | LibreOffice Extensions site or GitHub Release | 68, 69, 70 | Explicitly deferred until package build, install/uninstall smoke, Writer fixture, reversible write plan, accepted listing evidence, and release-surface refresh exist. |
| Streamlit demo | Streamlit app | Streamlit Community Cloud or local-only docs | 61 | Server smoke, synthetic data, privacy limits. |
| Static demo | GitHub Pages assets | GitHub Pages | 54 | Render/browser smoke and synthetic-data limits. |
