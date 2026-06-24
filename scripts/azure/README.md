# Azure and Linux probe path

## Windows (this machine)

WSL is not installed and [google-colab-cli](https://github.com/googlecolab/google-colab-cli)
does not support native Windows. Use:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\azure\run-directory-probes.ps1
```

This runs `scripts/colab/directory-probes.py` locally and writes JSON + markdown under
`conductor/tracks/73-mcp-directory-submission-hardening/`.

Requires `az login` only for subscription metadata in the report (probes work without it).

## Colab CLI (Linux / macOS / WSL)

After `wsl --install` and `uv tool install google-colab-cli`:

```bash
colab run scripts/colab/directory-probes.py > /tmp/sourceright-directory-probes.json
```

See `scripts/colab/README.md`.

## Azure Container Instances (optional)

`run-aci-directory-probes.ps1` is experimental on Windows because `az container create
--command-line` quoting breaks on semicolons, pipes, and curl `-o` flags.

Preferred ACI path:

1. Open [Azure Cloud Shell](https://shell.azure.com) (bash).
2. Paste/run `scripts/azure/directory-probes.sh`.

Subscription used in this repo: **Azure for Students** (`2e50ce2e-791e-4c15-9799-44be1d5e4e53`).

## What automation cannot do yet

| Task | Blocker |
| --- | --- |
| Glama listing | Signed-in **Add Server** at https://glama.ai/mcp/servers |
| OJS fixture smoke | `scripts/azure/run-ojs-smoke-aci.ps1` (Linux ACI) or `python scripts/colab/ojs-docker-smoke.py` |
| OJS disposable smoke | Docker/OJS host (WSL, Azure VM, or Container Apps + Docker) |
| Smithery `publicly_accepted` | Registry install smoke through Smithery gateway |
