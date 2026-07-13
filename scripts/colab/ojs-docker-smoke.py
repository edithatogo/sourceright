#!/usr/bin/env python3
"""OJS/PKP disposable smoke preflight for Linux (Colab VM, Azure Cloud Shell, WSL).

Runs fixture-backed journal-screen proof and optional PKP Docker preflight when
docker is available. Full OJS browser install remains operator-gated.
"""

from __future__ import annotations

import json
import os
import shutil
import subprocess
import sys
import tarfile
import tempfile
import urllib.request
from datetime import datetime, timezone
from pathlib import Path

REPO = os.environ.get("SOURCERIGHT_REPO", "https://github.com/edithatogo/sourceright.git")
REF = os.environ.get("SOURCERIGHT_REF", "main")
PLUGIN_VERSION = os.environ.get("OJS_PLUGIN_VERSION", "0.1.0")
RELEASE_TAG = os.environ.get("SOURCERIGHT_RELEASE", "v0.1.20")
RELEASE_ASSET = os.environ.get(
    "SOURCERIGHT_RELEASE_ASSET",
    "sourceright-linux-x86_64",
)
WORK_ROOT = Path(os.environ.get("OJS_SMOKE_WORK", tempfile.mkdtemp(prefix="sourceright-ojs-")))


def run(cmd: list[str], *, cwd: Path | None = None, timeout: int = 600) -> dict[str, object]:
    proc = subprocess.run(
        cmd,
        cwd=str(cwd) if cwd else None,
        capture_output=True,
        text=True,
        timeout=timeout,
        check=False,
    )
    return {
        "cmd": cmd,
        "exit_code": proc.returncode,
        "stdout": proc.stdout[-4000:],
        "stderr": proc.stderr[-4000:],
    }


def clone_repo(dest: Path) -> dict[str, object]:
    if dest.exists():
        shutil.rmtree(dest)
    return run(["git", "clone", "--depth", "1", "--branch", REF, REPO, str(dest)])


def build_plugin_archive(repo: Path, out_dir: Path) -> dict[str, object]:
    source = repo / "plugins" / "ojs" / "sourceright"
    stage = out_dir / "stage"
    plugin_dir = stage / "sourceright"
    archive = out_dir / f"sourceright-ojs-generic-plugin-{PLUGIN_VERSION}.tar.gz"
    if stage.exists():
        shutil.rmtree(stage)
    plugin_dir.mkdir(parents=True)
    shutil.copytree(source, plugin_dir, dirs_exist_ok=True)
    with tarfile.open(archive, "w:gz") as tar:
        tar.add(plugin_dir, arcname="sourceright")
    return {
        "archive": str(archive),
        "sha256": shutil.which("sha256sum") and run(["sha256sum", str(archive)]) or None,
    }


def find_sourceright_binary(repo: Path, bin_dir: Path) -> Path | None:
    for candidate in [
        shutil.which("sourceright"),
        bin_dir / "sourceright",
        bin_dir / "sourceright-linux-x86_64",
        repo / "target" / "release" / "sourceright",
    ]:
        if candidate and Path(str(candidate)).exists():
            return Path(str(candidate))
    return None


def download_release_binary(bin_dir: Path) -> dict[str, object]:
    bin_dir.mkdir(parents=True, exist_ok=True)
    dest = bin_dir / RELEASE_ASSET
    url = (
        f"https://github.com/edithatogo/sourceright/releases/download/"
        f"{RELEASE_TAG}/{RELEASE_ASSET}"
    )
    try:
        with urllib.request.urlopen(url, timeout=120) as response:
            dest.write_bytes(response.read())
    except Exception as exc:  # noqa: BLE001
        return {"status": "failed", "url": url, "error": str(exc)}
    dest.chmod(dest.stat().st_mode | 0o111)
    return {"status": "downloaded", "path": str(dest), "url": url}


def journal_screen_fixture(repo: Path, binary: Path, work: Path) -> dict[str, object]:
    fixture_path = repo / "fixtures" / "journal" / "ojs-submission.json"
    if not fixture_path.exists():
        return {"status": "skipped", "reason": f"missing fixture: {fixture_path}"}
    fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
    submission = fixture["submission"]
    screen_root = work / "journal-screen"
    if screen_root.exists():
        shutil.rmtree(screen_root)
    screen_root.mkdir(parents=True)

    init = run([str(binary), "init"], cwd=screen_root)
    if init["exit_code"] != 0:
        return {"status": "failed", "phase": "init", "result": init}

    workspace = screen_root / ".sourceright"
    (workspace / "references.csl.json").write_text(
        json.dumps(fixture["csl_references"]),
        encoding="utf-8",
    )
    sidecar = dict(fixture["verification_sidecar"])
    sidecar["schema_version"] = "sourceright.verification.v1"
    sidecar.pop("schema", None)
    (workspace / "references.verification.json").write_text(
        json.dumps(sidecar),
        encoding="utf-8",
    )

    result = run(
        [
            str(binary),
            "journal-screen",
            "--platform",
            submission["platform"],
            "--submission-id",
            submission["submission_id"],
            "--manuscript",
            submission["manuscript_label"],
            ".sourceright",
        ],
        cwd=screen_root,
        timeout=120,
    )
    ok = (
        result["exit_code"] == 0
        and "sourceright.journal_screening.v1" in result["stdout"]
        and submission["submission_id"] in result["stdout"]
    )
    return {"status": "passed" if ok else "failed", "result": result}


def docker_preflight(work: Path) -> dict[str, object]:
    docker = shutil.which("docker")
    if not docker:
        return {"status": "skipped", "reason": "docker not in PATH"}
    info = run([docker, "info", "--format", "{{.ServerVersion}}"])
    if info["exit_code"] != 0:
        return {"status": "skipped", "reason": "docker daemon unavailable", "detail": info}
    containers = work / "pkp-containers"
    clone = run(["git", "clone", "--depth", "1", "https://github.com/pkp/containers.git", str(containers)])
    return {
        "status": "preflight_only",
        "docker": info,
        "pkp_containers_clone": clone,
        "note": "docker compose up and browser OJS install are not automated in this script",
        "next": f"cd {containers} && docker compose up -d",
    }


def main() -> int:
    stamp = datetime.now(timezone.utc).isoformat()
    repo = WORK_ROOT / "sourceright"
    packages = WORK_ROOT / "packages"
    packages.mkdir(parents=True, exist_ok=True)

    report: dict[str, object] = {
        "schema": "sourceright.ojs_colab_smoke.v1",
        "stamp": stamp,
        "runner": os.environ.get("RUNNER", "colab-or-linux-python"),
        "work_root": str(WORK_ROOT),
        "repo": REPO,
        "ref": REF,
    }

    report["clone"] = clone_repo(repo)
    if report["clone"]["exit_code"] != 0:  # type: ignore[index]
        print(json.dumps(report, indent=2))
        return 1

    report["plugin_archive"] = build_plugin_archive(repo, packages)

    bin_dir = WORK_ROOT / "bin"
    binary = find_sourceright_binary(repo, bin_dir)
    if binary is None:
        report["release_binary"] = download_release_binary(bin_dir)
        if report["release_binary"].get("status") == "downloaded":  # type: ignore[union-attr]
            binary = find_sourceright_binary(repo, bin_dir)
    else:
        report["release_binary"] = {"status": "skipped", "reason": "binary already on PATH or in repo"}

    if binary is None:
        report["journal_screen_fixture"] = {
            "status": "skipped",
            "reason": "sourceright binary not found; set PATH, build release, or allow download",
        }
    else:
        report["journal_screen_fixture"] = journal_screen_fixture(repo, binary, WORK_ROOT)
    report["docker_preflight"] = docker_preflight(WORK_ROOT)

    passed = report["journal_screen_fixture"].get("status") == "passed"  # type: ignore[union-attr]
    report["overall"] = "passed" if passed else "partial"
    print(json.dumps(report, indent=2))
    return 0 if passed else 2


if __name__ == "__main__":
    raise SystemExit(main())
