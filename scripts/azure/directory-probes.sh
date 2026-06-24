#!/usr/bin/env bash
# Probe MCP directory listing URLs from a Linux environment (Azure ACI, VM, or CI).
set -uo pipefail

STAMP="${STAMP:-$(date -u +%Y-%m-%dT%H:%M:%SZ)}"
REPO="${REPO:-https://github.com/edithatogo/sourceright}"

probe() {
  local label="$1"
  local url="$2"
  local body_file
  body_file="$(mktemp)"
  local code
  code="$(curl -sS -L --max-time 30 -o "$body_file" -w "%{http_code}" "$url" || echo "000")"
  local snippet
  snippet="$(head -c 240 "$body_file" | tr '\n' ' ')"
  rm -f "$body_file"
  printf '%s\n' "---"
  printf 'label: %s\n' "$label"
  printf 'url: %s\n' "$url"
  printf 'http_code: %s\n' "$code"
  printf 'body_snippet: %s\n' "$snippet"
}

echo "directory_probe_run"
echo "stamp: ${STAMP}"
echo "repo: ${REPO}"
echo "runner: ${RUNNER:-linux}"

probe "smithery_listing" "https://smithery.ai/servers/edithatogo/sourceright"
probe "smithery_search" "https://smithery.ai/search?q=sourceright"
probe "glama_listing" "https://glama.ai/mcp/servers/edithatogo/sourceright"
probe "glama_api" "https://glama.ai/api/mcp/v1/servers/edithatogo/sourceright"
probe "glama_search" "https://glama.ai/mcp/servers?q=sourceright"
probe "mcp_registry" "https://registry.modelcontextprotocol.io/v0.1/servers/edithatogo/sourceright"

echo "done"
