"""The policy tool installer must refuse untrusted bytes and unsupported hosts."""

from pathlib import Path
import os
import subprocess
import tempfile
import unittest


INSTALLER = Path(__file__).resolve().parents[1] / "scripts/install-cargo-deny.sh"


class CargoDenyInstallTests(unittest.TestCase):
    def run_installer(self, commands):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        root = Path(temporary.name)
        shims = root / "shims"
        shims.mkdir()
        for name, content in commands.items():
            path = shims / name
            path.write_text("#!/usr/bin/env bash\nset -eu\n" + content)
            path.chmod(0o755)
        environment = os.environ.copy()
        environment.update(
            PATH=str(shims) + os.pathsep + environment["PATH"],
            RUNNER_TEMP=str(root),
            GITHUB_PATH=str(root / "runner-path"),
        )
        result = subprocess.run(
            ["bash", str(INSTALLER)],
            env=environment,
            cwd=root,
            text=True,
            capture_output=True,
            timeout=10,
            check=False,
        )
        return result, root

    def test_corrupt_download_is_rejected_before_extraction_or_path_export(self):
        result, root = self.run_installer(
            {
                "uname": 'case "$1" in -s) echo Linux;; -m) echo x86_64;; esac\n',
                "curl": 'while [[ "$1" != --output ]]; do shift; done\nprintf "corrupt archive" > "$2"\n',
                "tar": 'touch extraction-was-attempted\nexit 1\n',
            }
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("FAILED", result.stdout)
        self.assertFalse((root / "extraction-was-attempted").exists())
        self.assertFalse((root / "runner-path").exists())

    def test_unsupported_host_is_rejected_before_download(self):
        result, root = self.run_installer(
            {
                "uname": "echo Darwin\n",
                "curl": "touch download-was-attempted\nexit 1\n",
            }
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("Linux x86_64 only", result.stderr)
        self.assertFalse((root / "download-was-attempted").exists())
        self.assertFalse((root / "runner-path").exists())


if __name__ == "__main__":
    unittest.main()
