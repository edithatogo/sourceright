import assert from "node:assert/strict";
import fs from "node:fs/promises";
import vm from "node:vm";

const elements = new Map();
for (const selector of ["#status", "#report-note", "#metrics", "#issues", "#journal"]) {
  elements.set(selector, { textContent: "", innerHTML: "" });
}

const document = {
  querySelector(selector) {
    const element = elements.get(selector);
    if (!element) {
      throw new Error(`Unexpected selector ${selector}`);
    }
    return element;
  },
};

async function fetch(path) {
  const body = await fs.readFile(new URL(path, import.meta.url), "utf8");
  return {
    ok: true,
    status: 200,
    async json() {
      return JSON.parse(body);
    },
  };
}

const app = await fs.readFile(new URL("app.js", import.meta.url), "utf8");
vm.runInNewContext(app, { document, fetch, Promise, Error });
await new Promise((resolve) => setTimeout(resolve, 25));

assert.equal(
  elements.get("#status").textContent,
  "reference_integrity / sourceright.reference_report.v1",
);
assert.match(elements.get("#metrics").innerHTML, /<strong>2<\/strong><span>References<\/span>/);
assert.match(elements.get("#metrics").innerHTML, /<strong>1<\/strong><span>Review queue<\/span>/);
assert.match(elements.get("#issues").innerHTML, /Reference has no provider evidence yet\./);
assert.match(elements.get("#journal").innerHTML, /<dt>Submission<\/dt><dd>DEMO-001<\/dd>/);
assert.match(elements.get("#report-note").textContent, /Synthetic sample data only/);
