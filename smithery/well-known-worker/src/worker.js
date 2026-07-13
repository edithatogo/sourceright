/**
 * Serves SEP-1649 MCP server card at /.well-known/mcp/server-card.json on an
 * origin that allows dot-prefixed paths (GitHub project Pages does not).
 *
 * Deploy: cd smithery/well-known-worker && npx wrangler deploy
 * Then publish: smithery mcp publish https://<worker-host> -n edithatogo/sourceright
 */

const JSON_HEADERS = {
  "Content-Type": "application/json; charset=utf-8",
  "Cache-Control": "public, max-age=300",
  "Access-Control-Allow-Origin": "*",
};

export default {
  async fetch(request, env) {
    const url = new URL(request.url);

    if (request.method === "OPTIONS") {
      return new Response(null, {
        status: 204,
        headers: {
          "Access-Control-Allow-Origin": "*",
          "Access-Control-Allow-Methods": "GET, HEAD, OPTIONS",
          "Access-Control-Allow-Headers": "Content-Type",
        },
      });
    }

    if (
      request.method === "GET" &&
      url.pathname === "/.well-known/mcp/server-card.json"
    ) {
      const cardUrl = env.CARD_URL;
      const upstream = await fetch(cardUrl, {
        headers: { Accept: "application/json" },
        cf: { cacheTtl: 300 },
      });
      if (!upstream.ok) {
        return new Response(
          JSON.stringify({
            error: "upstream_card_fetch_failed",
            status: upstream.status,
            card_url: cardUrl,
          }),
          { status: 502, headers: JSON_HEADERS },
        );
      }
      return new Response(upstream.body, { status: 200, headers: JSON_HEADERS });
    }

    if (request.method === "GET" && url.pathname === "/health") {
      return new Response(
        JSON.stringify({ ok: true, service: "sourceright-mcp-well-known" }),
        { status: 200, headers: JSON_HEADERS },
      );
    }

    return new Response("Not Found", { status: 404 });
  },
};
