import type { APIRoute } from 'astro';
import serverCard from '../../../data/mcp-server-card.json';

export const prerender = true;

export const GET: APIRoute = () =>
  new Response(JSON.stringify(serverCard, null, 2) + '\n', {
    headers: {
      'Content-Type': 'application/json; charset=utf-8',
      'Cache-Control': 'public, max-age=300',
    },
  });
