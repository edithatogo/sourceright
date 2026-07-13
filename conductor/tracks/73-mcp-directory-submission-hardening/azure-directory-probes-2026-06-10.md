# Directory listing probes (2026-06-10)

Runner: local Python (scripts/colab/directory-probes.py) on Windows.
Azure context:  ()

| Label | HTTP | URL |
| --- | --- | --- |
| smithery_listing | 200 | https://smithery.ai/servers/edithatogo/sourceright |
| smithery_search | 200 | https://smithery.ai/search?q=sourceright |
| glama_listing | 404 | https://glama.ai/mcp/servers/edithatogo/sourceright |
| glama_api | 404 | https://glama.ai/api/mcp/v1/servers/edithatogo/sourceright |
| glama_search | 200 | https://glama.ai/mcp/servers?q=sourceright |
| mcp_registry | 404 | https://registry.modelcontextprotocol.io/v0.1/servers/edithatogo/sourceright |

Raw JSON: `directory-probes-2026-06-10.json`

## Next steps

- **Glama** listing/API 404 means Add Server is still required at https://glama.ai/mcp/servers (signed-in browser).
- **Colab CLI** (Linux/WSL): `colab run scripts/colab/directory-probes.py`; see `scripts/colab/README.md`.
- **Azure ACI** from Windows z has fragile --command-line quoting; prefer this script, Azure Cloud Shell, or WSL.
