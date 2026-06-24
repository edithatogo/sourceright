# OJS Docker Install Smoke Preflight (2026-06-10)

## Command

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/ojs-docker-install-smoke.ps1
```

## Environment

| Check | Result |
| --- | --- |
| Docker CLI | 29.5.3 installed |
| Docker Compose | v5.1.4 installed |
| Docker daemon | **Not running** (`npipe:////./pipe/docker_engine` missing) |
| WSL | **Not installed** |
| Podman | Not available |

## Artifacts produced

| Artifact | Path |
| --- | --- |
| Plugin archive | `C:\tmp\sourceright-ojs-smoke\packages\sourceright-ojs-generic-plugin-0.1.0.tar.gz` |
| SHA-256 sidecar | `...\sourceright-ojs-generic-plugin-0.1.0.tar.gz.sha256` |
| Smoke plan | `C:\tmp\sourceright-ojs-smoke\sourceright-ojs-docker-smoke-plan.md` |

## Blocker

Disposable OJS install smoke requires a running Docker daemon (or WSL/Podman alternative).
Plugin packaging succeeded; live OJS stack startup did not run.

## Operator action

1. Start Docker Desktop (or install WSL + enable Docker)
2. Re-run `scripts/ojs-docker-install-smoke.ps1 -FetchPkpContainers`
3. Follow `conductor/tracks/60-mature-ojs-plugin/ojs-install-smoke.md` for install transcript
4. Open PKP Plugin Gallery PR when smoke passes
