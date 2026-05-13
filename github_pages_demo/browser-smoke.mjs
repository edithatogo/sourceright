import http from "node:http";
import assert from "node:assert/strict";
import fs from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const enabled = process.env.SOURCERIGHT_DEMO_BROWSER_SMOKE === "1";
const ci = process.env.CI === "true" || process.env.CI === "1" || process.env.GITHUB_ACTIONS === "true";

function skip(message) {
  if (ci || enabled) {
    throw new Error(message);
  }
  console.log(`skipped: ${message}`);
  process.exit(0);
}

if (!enabled) {
  skip("set SOURCERIGHT_DEMO_BROWSER_SMOKE=1 to run the real browser smoke");
}

let chromium;
try {
  ({ chromium } = await import("playwright"));
} catch (error) {
  skip(`Playwright is not installed: ${error.message}`);
}

const root = path.dirname(fileURLToPath(import.meta.url));
const contentTypes = new Map([
  [".html", "text/html; charset=utf-8"],
  [".css", "text/css; charset=utf-8"],
  [".js", "text/javascript; charset=utf-8"],
  [".json", "application/json; charset=utf-8"],
]);

const server = http.createServer(async (request, response) => {
  try {
    const url = new URL(request.url ?? "/", "http://127.0.0.1");
    const pathname = url.pathname === "/" ? "/index.html" : url.pathname;
    const filePath = path.normalize(path.join(root, pathname));
    if (!filePath.startsWith(root)) {
      response.writeHead(403);
      response.end("forbidden");
      return;
    }
    const body = await fs.readFile(filePath);
    response.writeHead(200, {
      "content-type": contentTypes.get(path.extname(filePath)) ?? "application/octet-stream",
    });
    response.end(body);
  } catch {
    response.writeHead(404);
    response.end("not found");
  }
});

await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
const { port } = server.address();

let browser;
try {
  browser = await chromium.launch();
  const page = await browser.newPage();
  await page.goto(`http://127.0.0.1:${port}/`, { waitUntil: "networkidle" });
  await page.getByText("reference_integrity / sourceright.reference_report.v1").waitFor();
  const metricsText = await page.locator("#metrics").innerText();
  assert.match(metricsText, /2\s+References/);
  assert.match(metricsText, /1\s+Review queue/);
  await page.getByText("Reference has no provider evidence yet.").waitFor();
  await page.getByText("DEMO-001").waitFor();
  await page.getByText("Synthetic sample data only").waitFor();
} finally {
  if (browser) {
    await browser.close();
  }
  await new Promise((resolve) => server.close(resolve));
}
