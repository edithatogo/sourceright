# Glama listing acceptance — 2026-06-10

Operator added Sourceright via **Add Server** on Glama. Slug-style URLs (`edithatogo/sourceright`) still 404; the accepted listing uses Glama's server id.

## Public evidence

| Probe | URL | HTTP |
| --- | --- | --- |
| Listing | https://glama.ai/mcp/servers/c7qsbvekc1 | 200 |
| API | https://glama.ai/api/mcp/v1/servers/c7qsbvekc1 | 200 |

## API body (2026-06-10)

```json
{
  "id": "c7qsbvekc1",
  "name": "sourceright",
  "namespace": "edithatogo",
  "slug": "sourceright",
  "repository": { "url": "https://github.com/edithatogo/sourceright" },
  "url": "https://glama.ai/mcp/servers/c7qsbvekc1"
}
```

## Notes

- Legacy probe URLs `https://glama.ai/mcp/servers/edithatogo/sourceright` return **404** — use id-based listing URL for acceptance evidence.
- Local `glama.json` metadata was already validated in Track 73.
