#!/usr/bin/env bash
set -u
failures=0
run_check() {
  echo "\n==> $*"
  if "$@"; then
    echo "PASS: $*"
  else
    echo "FAIL: $*" >&2
    failures=$((failures + 1))
  fi
}
run_check cargo fmt --check
run_check cargo clippy --all-targets -- -D warnings
run_check cargo test
run_check cargo check --locked
if command -v python3 >/dev/null 2>&1; then
  echo "\n==> JSON syntax check"
  while IFS= read -r -d '' file; do
    python3 -m json.tool "$file" >/dev/null || failures=$((failures + 1))
  done < <(find . -path './target' -prune -o -name '*.json' -print0)
fi
if [[ $failures -gt 0 ]]; then
  echo "\n$failures validation step(s) failed." >&2
  exit 1
fi
echo "\nAll validation steps passed."
