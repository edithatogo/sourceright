import fs from "node:fs/promises";

const [inputPath, outputPath] = process.argv.slice(2);
if (!inputPath || !outputPath) {
  console.error("usage: node citation-js-runner.mjs INPUT OUTPUT");
  process.exit(2);
}

const { default: Cite } = await import("citation-js");
const input = await fs.readFile(inputPath, "utf8");
const cite = await Cite.async(input);
const output = cite.get({ type: "json" }).map(({ ["citation-key"]: _citationKey, ...item }) => item);
await fs.writeFile(outputPath, `${JSON.stringify(output, null, 2)}\n`, "utf8");
