#!/usr/bin/env bash
set -euo pipefail
if [[ $# -ne 1 ]]; then
  echo "Usage: $0 /path/to/sourceright-repo" >&2
  exit 2
fi
REPO="$1"
PACK_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if [[ ! -f "$REPO/Cargo.toml" ]]; then
  echo "Target does not look like a Rust repo: $REPO" >&2
  exit 1
fi
if [[ -f "$REPO/AGENTS.md" ]]; then
  echo "AGENTS.md already exists in $REPO. Review manually before replacing." >&2
  exit 1
fi
cp "$PACK_DIR/AGENTS.md.template" "$REPO/AGENTS.md"
echo "Copied AGENTS.md into $REPO"
echo "Now run: cd '$REPO' && codex"
echo "Then paste: $PACK_DIR/codex/prompts/00-inspect-current-repo.md"
