# Colab CLI path (Linux / macOS / WSL)

[google-colab-cli](https://github.com/googlecolab/google-colab-cli) provisions a
Linux VM for headless automation. **It does not run natively on Windows**; use WSL
or the Azure scripts in `scripts/azure/` from this machine.

## Install (WSL or Linux)

```bash
uv tool install google-colab-cli
# or: pip install google-colab-cli
colab auth   # first-time OAuth / ADC setup
```

## Directory listing probes

From the repository root on Linux or inside WSL:

```bash
colab run scripts/colab/directory-probes.py > /tmp/sourceright-directory-probes.json
```

Or without Colab, any Python 3:

```bash
python3 scripts/colab/directory-probes.py
```

Save output under
`conductor/tracks/73-mcp-directory-submission-hardening/` before updating
`live-evidence.json`.

## OJS/PKP fixture smoke

```bash
colab run scripts/colab/ojs-docker-smoke.py
```

Without Colab (any Python 3):

```bash
python3 scripts/colab/ojs-docker-smoke.py
```

The script clones the repo, builds the OJS plugin `.tar.gz`, runs
`journal-screen --platform ojs` against `fixtures/journal/ojs-submission.json`, and
optionally probes Docker when a daemon is available.

On **native Windows**, `colab` fails (`fcntl`); run the Python script directly or
use `scripts/azure/run-ojs-smoke-aci.ps1` for a Linux ACI runner.

## What Colab does not replace

- **Glama Add Server** — one-time signed-in browser at https://glama.ai/mcp/servers
- **OJS disposable install** — needs Docker (`pkp/containers`); fixture smoke does not
- **Smithery install smoke** — use `smithery mcp add` (recorded 2026-06-10)

## Windows operator default

Use `scripts/azure/run-aci-directory-probes.ps1` when WSL is not installed.
