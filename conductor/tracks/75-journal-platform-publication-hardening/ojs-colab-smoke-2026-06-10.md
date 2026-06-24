# OJS/PKP Colab-equivalent smoke — fixture + plugin archive

Date: 2026-06-10  
Runner: `windows-python-fixed-v2` via `scripts/colab/ojs-docker-smoke.py`  
Colab CLI: **not runnable on native Windows** (`ModuleNotFoundError: fcntl`); use WSL, Linux, or the same Python entrypoint.

## Preconditions

- `sourceright` on PATH (`v0.1.20` local build) or release binary download on Linux
- No Docker daemon required for this smoke tier

## Command

```powershell
$env:RUNNER = "windows-python-fixed-v2"
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
python scripts/colab/ojs-docker-smoke.py
```

On WSL/Linux after `colab auth`:

```bash
colab run scripts/colab/ojs-docker-smoke.py
```

## Result

| Check | Status |
| --- | --- |
| Git clone `edithatogo/sourceright` @ `main` | Pass |
| Plugin archive `sourceright-ojs-generic-plugin-0.1.0.tar.gz` | Pass |
| `journal-screen --platform ojs` on `fixtures/journal/ojs-submission.json` | Pass (`screened_with_errors`, 5 references) |
| PKP `docker compose` disposable install | Skipped — no Docker daemon |

Exit code: **0** (`overall`: `passed`)

## Claim boundary

This records fixture-backed OJS screening and installable plugin packaging. It does **not** claim PKP Plugin Gallery acceptance or a live OJS browser install.

## Next steps

- Disposable OJS: WSL + Docker, or Azure VM with Docker, then `pkp/containers`
- Gallery: open PKP Plugin Gallery PR when operator-ready
