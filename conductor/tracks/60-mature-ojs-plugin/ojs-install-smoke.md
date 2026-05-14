# OJS Install Smoke Path

This document records the install path Sourceright can test before claiming a
PKP Plugin Gallery listing.

## Current Installable Artifact

The repo can build a generic-plugin source archive:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/build-ojs-plugin-package.ps1
```

The script writes:

- `dist/ojs/sourceright-ojs-generic-plugin-0.1.0.tar.gz`
- `dist/ojs/sourceright-ojs-generic-plugin-0.1.0.tar.gz.sha256`

The archive contains a top-level `sourceright/` directory intended for:

```text
plugins/generic/sourceright
```

This is an install-test artifact, not PKP Plugin Gallery acceptance.

The repo also includes an opt-in Docker smoke harness:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/ojs-docker-install-smoke.ps1
```

The harness validates local Docker/Compose availability, builds the plugin
archive, and writes a concrete smoke plan under `C:\tmp\sourceright-ojs-smoke`.
Pass `-FetchPkpContainers` to clone `pkp/containers` into that work directory.
On the local Windows validation machine, Docker 29.4.2 and Docker Compose
v5.1.3 were detected, and `-FetchPkpContainers` successfully staged
`pkp/containers` under `C:\tmp\sourceright-ojs-smoke\containers`. The Docker
daemon was not running, so `docker compose up -d` could not be executed in this
session.

## Manual OJS Test

1. Install or start an OJS 3.x test instance.
2. Install the Sourceright CLI on the same host/container and verify:

   ```bash
   sourceright --version
   sourceright journal-screen --help
   ```

3. Extract the archive so the plugin exists at:

   ```text
   <ojs-root>/plugins/generic/sourceright
   ```

4. From the OJS root, register/upgrade the plugin metadata:

   ```bash
   php lib/pkp/tools/installPluginVersion.php plugins/generic/sourceright/version.xml
   ```

5. Log in as an OJS administrator, open the website/plugin settings, and enable
   the Sourceright generic plugin.
6. Configure `sourcerightCliPath` if `sourceright` is not on the web-server
   user's `PATH`.
7. Run a fixture-backed screen from the OJS host:

   ```bash
   sourceright journal-screen --platform ojs --submission-id OJS-SMOKE-1 /var/sourceright/workspace
   ```

8. Confirm OJS logs show no PHP fatal errors or notices from the plugin.

## Docker Test Candidate

PKP's current container path is the lowest-friction disposable test route. The
candidate test plan is:

1. Clone or vendor `pkp/containers` outside this repository, or run
   `scripts/ojs-docker-install-smoke.ps1 -FetchPkpContainers` to stage it under
   `C:\tmp\sourceright-ojs-smoke`.
2. Confirm the Docker daemon is running. The smoke script reports
   `dockerDaemonAvailable`; pass `-RequireDockerDaemon` when you want it to fail
   fast instead of generating a plan.
3. Start the OJS stack with the desired OJS version and a mounted plugin volume.
4. Mount or copy `plugins/ojs/sourceright` into the container as
   `/var/www/html/plugins/generic/sourceright`.
5. Install Sourceright into the container or mount a release binary into a
   known path.
6. Run `php lib/pkp/tools/installPluginVersion.php
   plugins/generic/sourceright/version.xml` inside the OJS container.
7. Enable the plugin in the administrator UI.
8. Run one editor-facing screening operation against a disposable fixture
   workspace.

The Docker path still needs local Docker availability and an OJS install
transcript before the track can claim live OJS compatibility.

`pkp/docker-ojs` is archived and points users to `pkp/containers`. It remains
useful historical context, but new smoke work should start with
`pkp/containers`.

## Evidence To Capture

- OJS version and PHP version.
- Archive name, SHA-256, and source commit.
- Install command transcript.
- Administrator UI enablement screenshot or log note.
- One successful `journal-screen --platform ojs` transcript.
- OJS PHP error log excerpt showing no plugin fatal errors.

## Current Boundary

Sourceright has an installable generic-plugin archive and a documented smoke
path. It still has no verified live OJS smoke transcript and no PKP Plugin
Gallery acceptance.
