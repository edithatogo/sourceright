from __future__ import annotations

import http.client
import os
import socket
import subprocess
import sys
import time
from pathlib import Path


def _truthy(name: str) -> bool:
    return os.environ.get(name, "").lower() in {"1", "true", "yes"}


def _skip(message: str) -> None:
    if _truthy("CI") or _truthy("GITHUB_ACTIONS") or _truthy("SOURCERIGHT_DEMO_SERVER_SMOKE"):
        raise SystemExit(message)
    print(f"skipped: {message}")
    raise SystemExit(0)


if not _truthy("SOURCERIGHT_DEMO_SERVER_SMOKE"):
    _skip("set SOURCERIGHT_DEMO_SERVER_SMOKE=1 to run the Streamlit server smoke")

try:
    import streamlit  # noqa: F401
except ImportError as exc:
    _skip(f"Streamlit is not installed: {exc}")


def _free_port() -> int:
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
        sock.bind(("127.0.0.1", 0))
        return int(sock.getsockname()[1])


root = Path(__file__).resolve().parent
port = _free_port()
command = [
    sys.executable,
    "-m",
    "streamlit",
    "run",
    str(root / "app.py"),
    "--server.headless=true",
    "--server.address=127.0.0.1",
    f"--server.port={port}",
    "--browser.gatherUsageStats=false",
]

process = subprocess.Popen(
    command,
    cwd=root,
    stdout=subprocess.PIPE,
    stderr=subprocess.STDOUT,
    text=True,
)

try:
    deadline = time.monotonic() + 30
    last_error = ""
    while time.monotonic() < deadline:
        if process.poll() is not None:
            output = process.stdout.read() if process.stdout else ""
            raise SystemExit(
                "Streamlit server exited before responding\n"
                f"command: python -m streamlit run app.py\n{output}"
            )
        try:
            conn = http.client.HTTPConnection("127.0.0.1", port, timeout=2)
            conn.request("GET", "/")
            response = conn.getresponse()
            body = response.read().decode("utf-8", errors="replace")
            if response.status == 200 and "streamlit" in body.lower():
                raise SystemExit(0)
            last_error = f"HTTP {response.status}: {body[:200]}"
        except OSError as exc:
            last_error = str(exc)
        finally:
            try:
                conn.close()
            except Exception:
                pass
        time.sleep(0.5)
    raise SystemExit(f"Streamlit server did not respond in time: {last_error}")
finally:
    process.terminate()
    try:
        process.wait(timeout=5)
    except subprocess.TimeoutExpired:
        process.kill()
