#!/usr/bin/env python3
"""Scaffold conductor tracks 83-90 for host plugin submission and acceptance."""

from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TRACKS = ROOT / "conductor" / "tracks"

TRACK_DEFS = [
    {
        "num": 83,
        "id": "vscode-open-vsx-submission-and-acceptance",
        "title": "VS Code and Open VSX Submission and Acceptance",
        "priority": "high",
        "owners": ["editor-extension", "publication"],
        "deps": [
            "66-vscode-extension-packaging",
            "77-vscode-open-vsx-publication-hardening",
            "72-submission-requirements-contracts",
            "82-self-improving-submission-and-health-loop",
        ],
        "owned_paths": [
            "extensions/vscode-sourceright/",
            "conductor/tracks/83-vscode-open-vsx-submission-and-acceptance/",
            "conductor/submission-packets/vscode-open-vsx.md",
        ],
        "surface_id": "vscode-open-vsx",
        "goal": "Ship a feature-complete thin VS Code extension and complete VS Code Marketplace and Open VSX submissions.",
        "artifact": "VSIX `edithatogo.sourceright` with diagnostics/commands, publisher tokens, and public listing URLs.",
        "submission": "VS Code Marketplace via `vsce publish`; Open VSX via `ovsx publish`.",
        "phases": [
            "Complete extension commands (validate, report, journal-screen bridge) beyond read-only smoke.",
            "Refresh VSIX build, Workspace Trust, and install/uninstall smoke on win32/linux.",
            "Record publisher credentials, pricing/category, and changelog gates.",
            "Execute Marketplace and Open VSX publish after approval; capture listing URLs in live evidence.",
        ],
        "matrix": [
            ("Feature completeness", "Extension exposes Sourceright diagnostics and stable JSON contracts.", "Extension integration tests + VSIX smoke", "Default-CI"),
            ("Marketplace requirements", "vsce/Open VSX metadata, icons, README, and trust declarations validated.", "requirements-evidence.md", "Default-CI"),
            ("Submission", "Marketplace and Open VSX listings return public URLs.", "live-evidence.json + submission receipt", "Opt-in-live"),
        ],
    },
    {
        "num": 84,
        "id": "claude-desktop-package-submission-and-acceptance",
        "title": "Claude Desktop Package Submission and Acceptance",
        "priority": "high",
        "owners": ["ai-clients", "mcp", "publication"],
        "deps": [
            "65-ai-client-mcp-packaging",
            "76-ai-client-extension-publication-hardening",
            "73-mcp-directory-submission-hardening",
            "82-self-improving-submission-and-health-loop",
        ],
        "owned_paths": [
            "examples/mcp-clients/claude-desktop.json",
            "smithery/mcpb/",
            "conductor/tracks/84-claude-desktop-package-submission-and-acceptance/",
        ],
        "surface_id": "claude-cowork",
        "goal": "Deliver a Claude Desktop/Cowork install path and complete host package or connector submission.",
        "artifact": "Claude-compatible MCPB bundle or Desktop MCP config with install smoke and connector metadata.",
        "submission": "Claude Desktop MCP config and/or MCPB connector submission per Anthropic docs.",
        "phases": [
            "Reconcile Claude Desktop stdio config vs MCPB connector requirements.",
            "Build/sign MCPB if required; run Desktop install smoke on macOS/Windows.",
            "Draft connector listing metadata and support boundaries.",
            "Submit connector/package after approval; record accepted listing or install receipt.",
        ],
        "matrix": [
            ("Desktop install smoke", "Claude Desktop launches Sourceright stdio MCP and lists tools.", "mcp-client-smoke log", "Opt-in-live"),
            ("MCPB parity", "MCPB bundle matches Smithery release semantics.", "MCPB build + hash", "Default-CI"),
            ("Submission", "Claude listing or documented accepted install path recorded.", "live-evidence.json", "Opt-in-live"),
        ],
    },
    {
        "num": 85,
        "id": "codex-app-package-submission-and-acceptance",
        "title": "Codex App Package Submission and Acceptance",
        "priority": "high",
        "owners": ["ai-clients", "mcp", "publication"],
        "deps": [
            "65-ai-client-mcp-packaging",
            "76-ai-client-extension-publication-hardening",
            "82-self-improving-submission-and-health-loop",
        ],
        "owned_paths": [
            "examples/mcp-clients/codex-config.toml",
            "examples/mcp-clients/codex-mcp.json",
            "conductor/tracks/85-codex-app-package-submission-and-acceptance/",
        ],
        "surface_id": "codex-app",
        "goal": "Explore Codex app MCP/plugin distribution and complete any supported package submission.",
        "artifact": "Codex MCP config package or documented registry artifact with install smoke.",
        "submission": "Codex MCP server registration or app plugin path per OpenAI Codex docs.",
        "phases": [
            "Search Codex app MCP/plugin registry requirements and supported package shapes.",
            "Harden `codex-config.toml` / `codex-mcp.json` and add install transcript smoke.",
            "Create distributable package or publish template if a registry exists.",
            "Submit after approval; record listing URL or maintainer acceptance.",
        ],
        "matrix": [
            ("Requirements recon", "Codex package vs MCP-only path documented.", "requirements-evidence.md", "Default-CI"),
            ("Install smoke", "Codex client discovers Sourceright tools/resources.", "transcript log", "Opt-in-live"),
            ("Submission", "Public Codex install/listing evidence recorded.", "live-evidence.json", "Opt-in-live"),
        ],
    },
    {
        "num": 86,
        "id": "github-copilot-extension-submission-and-acceptance",
        "title": "GitHub Copilot Extension Submission and Acceptance",
        "priority": "medium",
        "owners": ["ai-clients", "github", "publication"],
        "deps": [
            "64-github-side-governance-additions",
            "76-ai-client-extension-publication-hardening",
            "82-self-improving-submission-and-health-loop",
        ],
        "owned_paths": [
            "examples/mcp-clients/vscode-mcp.json",
            "examples/mcp-clients/github-copilot-coding-agent.md",
            "conductor/tracks/86-github-copilot-extension-submission-and-acceptance/",
        ],
        "surface_id": "github-copilot",
        "goal": "Complete a Copilot-facing package (extension or MCP distribution) and submit to the supported GitHub path.",
        "artifact": "Copilot extension, MCP config, or coding-agent bundle with entitlement proof.",
        "submission": "GitHub Marketplace/Copilot extension path or verified MCP config distribution.",
        "phases": [
            "Determine Copilot extension vs VS Code MCP vs coding-agent instructions path.",
            "Implement feature-complete surface (extension commands or agent workflow).",
            "Record entitlement/settings proof before enabled claims.",
            "Submit listing or publish MCP config package after approval.",
        ],
        "matrix": [
            ("Entitlement proof", "Copilot enabled path documented with screenshots or API evidence.", "entitlement-evidence.md", "Opt-in-live"),
            ("MCP smoke", "Copilot/VS Code MCP config launches Sourceright.", "transcript log", "Opt-in-live"),
            ("Submission", "Marketplace or accepted distribution URL recorded.", "live-evidence.json", "Opt-in-live"),
        ],
    },
    {
        "num": 87,
        "id": "gemini-cli-extension-submission-and-acceptance",
        "title": "Gemini CLI Extension Submission and Acceptance",
        "priority": "medium",
        "owners": ["ai-clients", "publication"],
        "deps": [
            "76-ai-client-extension-publication-hardening",
            "82-self-improving-submission-and-health-loop",
        ],
        "owned_paths": [
            "extensions/gemini-sourceright/",
            "conductor/tracks/87-gemini-cli-extension-submission-and-acceptance/",
        ],
        "surface_id": "gemini-cli-extensions",
        "goal": "Build a feature-complete Gemini CLI extension (including Antigravity-compatible flows) and publish it.",
        "artifact": "`gemini-extension.json` npm package with install smoke and MCP/CLI bridge.",
        "submission": "Gemini CLI extension install path (npm/git) and optional OCX registry bundle.",
        "phases": [
            "Pin Gemini CLI extension schema and scaffold `extensions/gemini-sourceright/`.",
            "Wire extension to `sourceright` CLI/MCP with preview-first writes.",
            "Run `gemini extensions install` smoke locally.",
            "Publish npm/git extension and record public install evidence after approval.",
        ],
        "matrix": [
            ("Extension schema", "gemini-extension.json validates against host schema.", "schema validation test", "Default-CI"),
            ("Install smoke", "Gemini CLI loads extension and invokes Sourceright.", "install-smoke log", "Opt-in-live"),
            ("Submission", "Public extension URL/version recorded.", "live-evidence.json", "Opt-in-live"),
        ],
    },
    {
        "num": 88,
        "id": "qwen-cli-extension-submission-and-acceptance",
        "title": "Qwen CLI Extension Submission and Acceptance",
        "priority": "medium",
        "owners": ["ai-clients", "publication"],
        "deps": [
            "76-ai-client-extension-publication-hardening",
            "82-self-improving-submission-and-health-loop",
        ],
        "owned_paths": [
            "extensions/qwen-sourceright/",
            "conductor/tracks/88-qwen-cli-extension-submission-and-acceptance/",
        ],
        "surface_id": "qwen-cli-extensions",
        "goal": "Build a feature-complete Qwen Code extension and publish it.",
        "artifact": "`qwen-extension.json` npm package with install smoke.",
        "submission": "Qwen Code extension registry/npm path per official docs.",
        "phases": [
            "Pin Qwen extension schema and scaffold `extensions/qwen-sourceright/`.",
            "Bridge to Sourceright CLI/MCP with deterministic JSON outputs.",
            "Run Qwen extension install smoke.",
            "Publish npm package and record listing evidence after approval.",
        ],
        "matrix": [
            ("Extension schema", "qwen-extension.json validates.", "schema validation test", "Default-CI"),
            ("Install smoke", "Qwen CLI loads extension.", "install-smoke log", "Opt-in-live"),
            ("Submission", "Public extension URL recorded.", "live-evidence.json", "Opt-in-live"),
        ],
    },
    {
        "num": 89,
        "id": "opencode-plugin-submission-and-acceptance",
        "title": "OpenCode Plugin Submission and Acceptance",
        "priority": "medium",
        "owners": ["ai-clients", "publication"],
        "deps": [
            "76-ai-client-extension-publication-hardening",
            "82-self-improving-submission-and-health-loop",
        ],
        "owned_paths": [
            "extensions/opencode-sourceright/",
            "examples/mcp-clients/opencode.json",
            "conductor/tracks/89-opencode-plugin-submission-and-acceptance/",
        ],
        "surface_id": "opencode",
        "goal": "Ship OpenCode MCP config and/or npm plugin and publish through OpenCode/OCX distribution paths.",
        "artifact": "OpenCode `mcp` config block, local plugin, or npm `plugin` entry with install smoke.",
        "submission": "npm plugin publish and/or OCX registry packument; MCP config template in repo.",
        "phases": [
            "Document OpenCode MCP vs npm plugin vs OCX registry requirements.",
            "Scaffold plugin under `extensions/opencode-sourceright/` and example `opencode.json`.",
            "Run OpenCode install/load smoke.",
            "Publish npm/OCX bundle after approval; record install URL.",
        ],
        "matrix": [
            ("MCP config", "opencode.json launches Sourceright stdio MCP.", "config smoke", "Opt-in-live"),
            ("Plugin package", "npm plugin validates and loads in OpenCode.", "plugin build log", "Default-CI"),
            ("Submission", "npm or OCX listing evidence recorded.", "live-evidence.json", "Opt-in-live"),
        ],
    },
    {
        "num": 90,
        "id": "cline-mcp-marketplace-submission-and-acceptance",
        "title": "Cline MCP Marketplace Submission and Acceptance",
        "priority": "medium",
        "owners": ["ai-clients", "mcp", "publication"],
        "deps": [
            "73-mcp-directory-submission-hardening",
            "76-ai-client-extension-publication-hardening",
            "82-self-improving-submission-and-health-loop",
        ],
        "owned_paths": [
            "conductor/tracks/90-cline-mcp-marketplace-submission-and-acceptance/",
            "docs/src/mcp.md",
        ],
        "surface_id": "cline",
        "goal": "Prepare Cline MCP Marketplace submission assets and complete listing acceptance.",
        "artifact": "400×400 logo, `llms-install.md`, stable README install path, marketplace issue/PR.",
        "submission": "GitHub issue to `cline/mcp-marketplace` with repo URL, logo, and install proof.",
        "phases": [
            "Author `llms-install.md` and verify Cline one-click/manual install from README.",
            "Produce 400×400 PNG marketplace logo and metadata.",
            "Run Cline MCP install smoke (stdio config).",
            "File marketplace submission after approval; record accepted listing URL.",
        ],
        "matrix": [
            ("Install doc", "README + llms-install.md enable Cline setup without extra steps.", "install-smoke log", "Opt-in-live"),
            ("Marketplace assets", "Logo and metadata meet cline/mcp-marketplace template.", "submission-drafts.md", "Default-CI"),
            ("Submission", "Accepted marketplace listing URL recorded.", "live-evidence.json", "Opt-in-live"),
        ],
    },
]


def write_track(defn: dict) -> None:
    num = defn["num"]
    slug = defn["id"]
    base = TRACKS / f"{num}-{slug}"
    base.mkdir(parents=True, exist_ok=True)

    import json

    metadata = {
        "id": slug,
        "status": "planned",
        "priority": defn["priority"],
        "owners": defn["owners"],
        "dependencies": defn["deps"],
        "owned_paths": defn["owned_paths"],
    }
    (base / "metadata.json").write_text(
        json.dumps(metadata, indent=2) + "\n", encoding="utf-8"
    )

    phases = list(defn["phases"]) + [
        "Update submission inventory and live evidence after acceptance.",
        "Do not submit externally until approval is recorded in submission inventory.",
    ]
    phases_md = "\n".join(f"{i}. [ ] {step}" for i, step in enumerate(phases, 1))
    matrix_rows = "\n".join(
        f"| {name} | {accept} | {evidence} | {ci} |"
        for name, accept, evidence, ci in defn["matrix"]
    )

    (base / "spec.md").write_text(
        f"""# Track {num}: {defn['title']}

## Goal

{defn['goal']}

## User Outcome

Operators can install Sourceright through the host's supported package or listing
path with feature-complete behavior, install smoke, and public acceptance evidence.

## Scope

- Feature-complete host package or marketplace listing assets for `{defn['surface_id']}`.
- Official requirements reconnaissance and submission mechanism documentation.
- Approval-gated external submission and `live-evidence.json` promotion.

## Out Of Scope

- Reimplementing Sourceright verification in host-native languages.
- Claiming acceptance before listing URL, API evidence, or maintainer receipt exists.

## Data Contracts

Host packages call `sourceright` CLI/MCP with stable JSON outputs. Write paths
remain preview-first with audit evidence.

## Submission Target

{defn['submission']}

## Required Artifact

{defn['artifact']}

## Claim Boundary

No `{defn['surface_id']}` acceptance claim until submission evidence is recorded
with URL, date, version, and install metadata.

## Evidence Level Target

Feature-complete local package → submission_ready → submitted → publicly_accepted.

## Parallelization Plan

Requirements recon, package scaffold, and install smoke can run in parallel after
shared MCP contracts are frozen.

## Maturity, Stability, And Testing

Maturity requires requirements search, package validation, install smoke,
submission drafts, approval gate, and live evidence verifier pass.
""",
        encoding="utf-8",
    )

    (base / "plan.md").write_text(
        f"""# Plan

{phases_md}
""",
        encoding="utf-8",
    )

    (base / "test-matrix.md").write_text(
        f"""# Test Matrix

| Scenario | Acceptance | Evidence | Default-CI or opt-in-live |
| --- | --- | --- | --- |
{matrix_rows}
""",
        encoding="utf-8",
    )

    (base / "review.md").write_text(
        f"""# Track {num} — {defn['title']} — Review

## Status

**Planned** — track scaffolded {defn['surface_id']} submission lane.

## Dependencies

{', '.join(defn['deps'])}

## Next actions

1. Complete requirements-evidence.md from official host docs.
2. Implement feature-complete package artifacts under owned paths.
3. Run install smoke and capture submission drafts.
4. Execute external submission only after approval.
""",
        encoding="utf-8",
    )

    (base / "requirements-evidence.md").write_text(
        f"""# Requirements Evidence — {defn['title']}

Date: 2026-06-10  
Surface: `{defn['surface_id']}`

## Submission mechanism (to verify)

{defn['submission']}

## Local baseline

Tracks 65–77 hardening and `examples/mcp-clients/` configs exist.
This track closes the gap to **feature-complete package + public submission**.

## Evidence still required

- Official requirements URLs with retrieval dates
- Package/build logs
- Install smoke transcript
- Submission receipt or public listing URL
""",
        encoding="utf-8",
    )

    (base / "submission-drafts.md").write_text(
        f"""# Submission Drafts — {defn['title']} (Not Submitted)

Date: 2026-06-10

**Approval gate:** `{defn['surface_id']}` has `external_submission_allowed: false` until this track completes readiness review.

## Draft target

{defn['submission']}

## Artifact

{defn['artifact']}

## Rollback

Do not promote `submitted` or `publicly_accepted` gates without URL evidence in `live-evidence.json`.
""",
        encoding="utf-8",
    )


def main() -> None:
    for defn in TRACK_DEFS:
        write_track(defn)
    print(f"Scaffolded {len(TRACK_DEFS)} tracks under {TRACKS}")


if __name__ == "__main__":
    main()
