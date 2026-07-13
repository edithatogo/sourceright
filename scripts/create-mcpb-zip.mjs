import { deflateRawSync } from "node:zlib";
import { readdir, readFile, stat, writeFile } from "node:fs/promises";
import { join, relative, resolve } from "node:path";

const [stagingRootArg, outputArg, binaryName, platform] = process.argv.slice(2);
if (!stagingRootArg || !outputArg || !binaryName || !platform) {
  throw new Error("usage: node create-mcpb-zip.mjs <staging-root> <output> <binary-name> <platform>");
}

const stagingRoot = resolve(stagingRootArg);
const output = resolve(outputArg);

async function filesUnder(directory) {
  const entries = await readdir(directory, { withFileTypes: true });
  const files = [];
  for (const entry of entries) {
    const absolute = join(directory, entry.name);
    if (entry.isDirectory()) files.push(...await filesUnder(absolute));
    else if (entry.isFile()) files.push(absolute);
  }
  return files;
}

// CRC-32 is intentionally implemented here to keep the MCPB builder
// dependency-free and reproducible on Windows and CI runners.
const crcTable = new Uint32Array(256);
for (let n = 0; n < 256; n += 1) {
  let c = n;
  for (let bit = 0; bit < 8; bit += 1) c = (c & 1) ? (0xedb88320 ^ (c >>> 1)) : (c >>> 1);
  crcTable[n] = c >>> 0;
}
function crc32(buffer) {
  let c = 0xffffffff;
  for (const byte of buffer) c = crcTable[(c ^ byte) & 0xff] ^ (c >>> 8);
  return (c ^ 0xffffffff) >>> 0;
}

const files = await filesUnder(stagingRoot);
const entries = [];
let offset = 0;
const creatorPlatform = platform === "win32" ? 0 : 3; // DOS or Unix

for (const absolute of files) {
  const data = await readFile(absolute);
  const name = relative(stagingRoot, absolute).split("\\").join("/");
  const nameBytes = Buffer.from(name, "utf8");
  const compressed = deflateRawSync(data, { level: 9 });
  const executable = name === `bin/${binaryName}`;
  const mode = executable ? 0o755 : 0o644;
  const externalAttributes = platform === "win32" ? 0x20 : mode << 16;

  const local = Buffer.alloc(30 + nameBytes.length);
  local.writeUInt32LE(0x04034b50, 0);
  local.writeUInt16LE(20, 4);
  local.writeUInt16LE(0, 6);
  local.writeUInt16LE(8, 8);
  local.writeUInt16LE(0, 10);
  local.writeUInt16LE(0, 12);
  local.writeUInt32LE(crc32(data), 14);
  local.writeUInt32LE(compressed.length, 18);
  local.writeUInt32LE(data.length, 22);
  local.writeUInt16LE(nameBytes.length, 26);
  local.writeUInt16LE(0, 28);
  nameBytes.copy(local, 30);
  entries.push({ nameBytes, compressed, data, local, offset, externalAttributes });
  offset += local.length + compressed.length;
}

const central = [];
for (const entry of entries) {
  const record = Buffer.alloc(46 + entry.nameBytes.length);
  record.writeUInt32LE(0x02014b50, 0);
  record.writeUInt16LE((creatorPlatform << 8) | 20, 4);
  record.writeUInt16LE(20, 6);
  record.writeUInt16LE(0, 8);
  record.writeUInt16LE(8, 10);
  record.writeUInt16LE(0, 12);
  record.writeUInt16LE(0, 14);
  record.writeUInt32LE(crc32(entry.data), 16);
  record.writeUInt32LE(entry.compressed.length, 20);
  record.writeUInt32LE(entry.data.length, 24);
  record.writeUInt16LE(entry.nameBytes.length, 28);
  record.writeUInt16LE(0, 30);
  record.writeUInt16LE(0, 32);
  record.writeUInt16LE(0, 34);
  record.writeUInt16LE(0, 36);
  record.writeUInt32LE(entry.externalAttributes, 38);
  record.writeUInt32LE(entry.offset, 42);
  entry.nameBytes.copy(record, 46);
  central.push(record);
}

const centralDirectory = Buffer.concat(central);
const end = Buffer.alloc(22);
end.writeUInt32LE(0x06054b50, 0);
end.writeUInt16LE(0, 4);
end.writeUInt16LE(0, 6);
end.writeUInt16LE(entries.length, 8);
end.writeUInt16LE(entries.length, 10);
end.writeUInt32LE(centralDirectory.length, 12);
end.writeUInt32LE(offset, 16);
end.writeUInt16LE(0, 20);

const chunks = [];
for (const entry of entries) chunks.push(entry.local, entry.compressed);
chunks.push(centralDirectory, end);
await writeFile(output, Buffer.concat(chunks));
const outputStat = await stat(output);
console.log(JSON.stringify({ output, files: entries.length, bytes: outputStat.size, creatorPlatform }));
