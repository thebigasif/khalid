// Fails the build when the admin, compressed, passes either limit in
// docs/standards/performance.md: the first download, which is what a
// visitor waits for, and every built file together, a ceiling against
// runaway growth. Each file is gzipped at level 9, as khalid-perf counts.
import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { gzipSync } from "node:zlib";

const FIRST_LIMIT_KB = 250;
const TOTAL_LIMIT_KB = 450;
const DIST = "dist";
const PAGE = join(DIST, "index.html");

function files(dir) {
  return readdirSync(dir).flatMap((name) => {
    const path = join(dir, name);
    return statSync(path).isDirectory() ? files(path) : [path];
  });
}

function gzipped(path) {
  return gzipSync(readFileSync(path), { level: 9 }).length;
}

// The file under dist an address names. The address starts with the
// folder the admin is mounted at, which dist does not have, so leading
// folders are dropped until a file is there.
function found(address) {
  let rest = address.replace(/^\/+/, "");
  while (rest) {
    const path = join(DIST, rest);
    if (!rest.split("/").includes("..") && existsSync(path)) {
      return path;
    }
    const slash = rest.indexOf("/");
    rest = slash < 0 ? "" : rest.slice(slash + 1);
  }
  throw new Error(`index.html links ${address}, which is not in ${DIST}.`);
}

// Each script and stylesheet index.html loads at once: <script src>,
// <link rel="modulepreload"> and <link rel="stylesheet">.
function linked(html) {
  const tags = html.match(/<(script|link)\b[^>]*>/g) ?? [];
  return tags.flatMap((tag) => {
    const address = tag.match(/\s(?:src|href)="([^"]+)"/)?.[1];
    const loads = tag.startsWith("<script") || /\srel="(?:modulepreload|stylesheet)"/.test(tag);
    return address && loads ? [found(address)] : [];
  });
}

// The chunks a script imports at the top, which load with it. An
// import() that waits for a route is not followed.
function imports(path) {
  const code = readFileSync(path, "utf8");
  const named = code.matchAll(/(?:\bfrom|\bimport)\s*["'](\.{1,2}\/[^"']+\.js)["']/g);
  return [...named].map((match) => join(dirname(path), match[1]));
}

const first = new Set([PAGE]);
const waiting = linked(readFileSync(PAGE, "utf8"));
while (waiting.length > 0) {
  const path = waiting.pop();
  if (!first.has(path)) {
    first.add(path);
    if (path.endsWith(".js")) {
      waiting.push(...imports(path));
    }
  }
}

const counted = files(DIST).filter((path) => /\.(js|css|html)$/.test(path));
let firstBytes = 0;
let totalBytes = 0;
for (const path of counted) {
  const size = gzipped(path);
  totalBytes += size;
  const now = first.has(path);
  if (now) {
    firstBytes += size;
  }
  const when = now ? "first" : "later";
  console.log(`${(size / 1024).toFixed(1).padStart(8)} KB  ${when}  ${relative(DIST, path)}`);
}

const firstKb = firstBytes / 1024;
const totalKb = totalBytes / 1024;
const over = [];
if (firstKb > FIRST_LIMIT_KB) {
  over.push(
    `The admin's first download is ${firstKb.toFixed(1)} KB compressed, over its ${FIRST_LIMIT_KB} KB limit.`,
    "Load a screen with a lazy import, or drop a dependency, then build again.",
  );
}
if (totalKb > TOTAL_LIMIT_KB) {
  over.push(
    `All the admin's files are ${totalKb.toFixed(1)} KB compressed, over their ${TOTAL_LIMIT_KB} KB ceiling.`,
    "Drop a dependency or code no screen uses, then build again.",
  );
}
if (over.length > 0) {
  for (const line of over) {
    console.error(line);
  }
  process.exit(1);
}
console.log(
  `The admin's first download is ${firstKb.toFixed(1)} KB compressed, inside its ${FIRST_LIMIT_KB} KB limit.`,
);
console.log(
  `All its files are ${totalKb.toFixed(1)} KB compressed, inside their ${TOTAL_LIMIT_KB} KB ceiling.`,
);
