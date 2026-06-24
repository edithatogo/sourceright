# Glama Add Server Attempt (2026-06-10)

## Probes (automated)

| Check | URL | Result |
| --- | --- | --- |
| Listing | https://glama.ai/mcp/servers/edithatogo/sourceright | **404** |
| API | https://glama.ai/api/mcp/v1/servers/edithatogo/sourceright | **404** `Server not found` |
| Search | https://glama.ai/mcp/servers?q=sourceright | **200**, no match |

Raw probe JSON: `directory-probes-2026-06-10.json`

## Browser attempt

| Step | Result |
| --- | --- |
| Open https://glama.ai/mcp/servers | **Add Server** button visible |
| Click Add Server | No submission form without authentication |
| Navigate https://glama.ai/sign-in | Email login form shown — **operator must complete sign-in** |

## Submission draft (ready)

From `submission-drafts.md`:

- Repository: `https://github.com/edithatogo/sourceright`
- Metadata: root `glama.json`, maintainer `edithatogo`, MIT OR Apache-2.0

## Blocker

Glama listing submission requires a **signed-in browser session**. This step cannot be
completed from unattended automation.

## Operator action

1. Sign in at https://glama.ai/mcp/servers
2. Click **Add Server**
3. Paste `https://github.com/edithatogo/sourceright`
4. Claim via `glama.json` if prompted
5. Record listing or API URL in `live-evidence.json` and re-run probes
