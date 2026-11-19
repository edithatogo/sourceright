"""Exercise the release workflow's manifest selector using synthetic manifests."""

import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import textwrap
import unittest

ROOT = Path(__file__).resolve().parents[1]
INDEX = "application/vnd.oci.image.index.v1+json"
MANIFEST = "application/vnd.oci.image.manifest.v1+json"
PARENT = "sha256:" + "a" * 64
CHILD = "sha256:" + "b" * 64


class ImagePromotionTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.workflow = (ROOT / ".github/workflows/release.yml").read_text()
        block = cls.workflow.split("      - name: Resolve the candidate's only runnable platform manifest\n", 1)[1]
        cls.selector = textwrap.dedent(block.split("          python3 - <<'PY'\n", 1)[1].split("          PY\n", 1)[0])

    def select(self, document):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "sourceright-image-index.json").write_text(json.dumps(document))
            output = root / "output"
            environment = os.environ | {"RUNNER_TEMP": directory, "GITHUB_OUTPUT": str(output), "CANDIDATE_IMAGE": "ghcr.io/edithatogo/sourceright-mcp@" + PARENT}
            result = subprocess.run([sys.executable, "-c", self.selector], env=environment, text=True, capture_output=True)
            return result.returncode, output.read_text() if output.exists() else ""

    def test_index_selects_child_and_preserves_attestation(self):
        document = {"mediaType": INDEX, "manifests": [
            {"digest": CHILD, "platform": {"os": "linux", "architecture": "amd64"}},
            {"digest": "sha256:" + "c" * 64, "platform": {"os": "unknown", "architecture": "unknown"}, "annotations": {"vnd.docker.reference.type": "attestation-manifest"}},
        ]}
        status, output = self.select(document)
        self.assertEqual(status, 0)
        self.assertEqual(output, "image=ghcr.io/edithatogo/sourceright-mcp@" + CHILD + "\n")

    def test_single_manifest_keeps_original_digest(self):
        status, output = self.select({"mediaType": MANIFEST})
        self.assertEqual(status, 0)
        self.assertIn(PARENT, output)

    def test_extra_missing_unknown_or_malformed_platform_is_rejected(self):
        valid = {"digest": CHILD, "platform": {"os": "linux", "architecture": "amd64"}}
        invalid_sets = [[], [valid, valid], [valid, {"digest": CHILD, "platform": {"os": "linux", "architecture": "arm64"}}],
                        [valid, {"digest": CHILD, "platform": {"os": "unknown", "architecture": "unknown"}}],
                        [{"digest": "latest", "platform": valid["platform"]}]]
        for manifests in invalid_sets:
            with self.subTest(manifests=manifests):
                status, output = self.select({"mediaType": INDEX, "manifests": manifests})
                self.assertNotEqual(status, 0)
                self.assertEqual(output, "")

    def test_promotion_waits_for_scan_and_checks_original_digest(self):
        scan = self.workflow.index("      - name: Scan the exact image digest before release tags")
        promotion = self.workflow.index("      - name: Promote the exact scanned digest without rebuilding")
        self.assertLess(scan, promotion)
        promoted = self.workflow[promotion:].split("      - name: Generate image attestation", 1)[0]
        self.assertIn("steps.build-image.outputs.digest", promoted)
        self.assertIn("--prefer-index=false", promoted)
        self.assertIn('test "$actual" = "$expected"', promoted)
        self.assertNotIn("continue-on-error", self.workflow[scan:promotion])


if __name__ == "__main__":
    unittest.main()
