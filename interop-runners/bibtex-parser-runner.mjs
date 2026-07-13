import fs from "node:fs/promises";
import { parse } from "@retorquere/bibtex-parser";

const [inputPath, outputPath] = process.argv.slice(2);
if (!inputPath || !outputPath) {
  console.error("usage: node bibtex-parser-runner.mjs INPUT OUTPUT");
  process.exit(2);
}

const input = await fs.readFile(inputPath, "utf8");
const parsed = parse(input);
await fs.writeFile(outputPath, `${JSON.stringify(parsed, null, 2)}\n`, "utf8");
