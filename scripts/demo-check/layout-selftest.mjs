// Proves the frame, reading, table and stylesheet checks of layout.mjs on two
// small sites it writes itself, in Chromium. The site "good" meets every
// check. The site "bad" breaks each check once: its home page draws a
// frame of 800 px where its brief asks for 1100, its post's text is 17 px,
// its folded-table post holds 85 characters a line, and its stylesheet is
// over 20 KB with Brotli, and a table column breaks a word onto a
// second line, where "good" has the same word just fitting its column.
// Both list their posts as narrow cards in small
// text, which the reading check leaves alone. The text is monospaced, so
// a line holds exactly as many characters as its column is wide in ch.
//
// Usage: node layout-selftest.mjs, from anywhere. Exits 1 when a check
// misses a fault or finds one that is not there.
import { spawnSync } from "node:child_process";
import { mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const work = join(here, "..", "..", "target", "layout-selftest");
const POSTS = ["ai-search-guide/", "content-benchmark-data/"];
const LISTS = ["", "category/seo-writing/", "tag/ai-search/", "authors/maren-okafor/", "search/", "glossary/answer-capsule/"];

const words = "frame column reader measure layout template brief steady wide narrow line text".split(" ");
const prose = (n) => Array.from({ length: n }, (_, i) => words[(i * 7) % words.length]).join(" ");
// Text Brotli cannot shrink, so the stylesheet stays over 20 KB.
const noise = (n) => {
  let seed = 7;
  return Array.from({ length: n }, () => {
    // Park and Miller's generator, exact in a JavaScript number.
    seed = (seed * 48271) % 2147483647;
    return "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"[seed % 62];
  }).join("");
};
const CSS = `*{box-sizing:border-box}body{margin:0;font:16px/1.5 monospace}
.site-header,main{max-width:1200px;margin:0 auto;padding:16px}.site-header{display:flex;justify-content:space-between}
.narrow{max-width:800px}.cards{display:grid;grid-template-columns:repeat(auto-fill,minmax(160px,1fr));gap:16px}
.card{border:1px solid #888;padding:8px;font-size:13px}.post-body p{font-size:19px;max-width:68ch}
.small p{font-size:17px}.long p{max-width:85ch}
.fit td{width:12ch;padding:0;overflow-wrap:normal}.crush td{width:6ch;padding:0;overflow-wrap:anywhere}`;

const page = (body, mainClass) => `<!doctype html><html lang="en"><head><meta charset="utf-8"><title>Fixture</title>
<meta name="viewport" content="width=device-width, initial-scale=1"><link rel="stylesheet" href="/style.css"></head><body>
<header class="site-header"><a href="/">Fixture blog</a><nav><a href="/search/">Search</a></nav></header>
<main id="main"${mainClass ? ` class="${mainClass}"` : ""}>${body}</main></body></html>`;
const cards = `<h1>Posts</h1><div class="cards">${Array.from({ length: 8 }, (_, i) => `<article class="card"><h2>Post ${i + 1}</h2><p>${prose(20)}</p></article>`).join("")}</div>`;
// A table whose first column is wider than its longest word, or,
// with tableClass "crush", narrower, so the word breaks onto a second line.
const table = (tableClass) => `<table class="${tableClass}"><tr><td>measurable</td><td>${prose(6)}</td></tr></table>`;
const post = (bodyClass, tableClass) => `<article><h1>A post</h1><div class="post-body${bodyClass ? ` ${bodyClass}` : ""}">${[1, 2, 3].map(() => `<p>${prose(90)}</p>`).join("")}${tableClass ? table(tableClass) : ""}</div></article>`;

const site = (name, faults) => {
  const folder = join(work, "sites", name);
  const files = join(folder, "public");
  for (const path of [...LISTS, ...POSTS]) mkdirSync(join(files, path), { recursive: true });
  writeFileSync(join(folder, "site.toml"), `address = "http://${name}.localhost"\nmount = "/"\n`);
  writeFileSync(join(files, "style.css"), faults ? `${CSS}\n/* ${noise(30000)} */\n` : CSS);
  for (const path of LISTS) writeFileSync(join(files, path, "index.html"), page(cards, faults && path === "" ? "narrow" : ""));
  writeFileSync(join(files, POSTS[0], "index.html"), page(post(faults ? "small" : "")));
  writeFileSync(join(files, POSTS[1], "index.html"), page(post(faults ? "long" : "", faults ? "crush" : "fit")));
};

rmSync(work, { recursive: true, force: true });
site("good", false);
site("bad", true);
writeFileSync(join(work, "frames.json"), JSON.stringify({ frames: { good: 1100, bad: 1100 } }));

const run = (widths) => {
  const args = [join(here, "layout.mjs"), "--sites", join(work, "sites"), "--browsers", "chromium", "--widths", widths, "--frames", join(work, "frames.json")];
  spawnSync(process.execPath, args, { stdio: "inherit" });
  return JSON.parse(readFileSync(join(work, "report.json"), "utf8")).map((f) => [f.pack, f.page, f.check, f.width].join(" "));
};
const expect = (width) => [`bad home frame-fill ${width}`, `bad post reading-size ${width}`, `bad folded table reading-column ${width}`, `bad folded table crushed-cell ${width}`];
const cases = [
  ["390,1440", [...expect(1440), "bad folded table crushed-cell 390", "bad home stylesheet-size 0"]],
  ["2000", [...expect(2000), "bad home stylesheet-size 0"]],
];
let wrong = 0;
for (const [widths, wanted] of cases) {
  const found = run(widths);
  for (const f of wanted.filter((w) => !found.includes(w))) {
    console.log(`MISSED at --widths ${widths}: ${f}`);
    wrong += 1;
  }
  for (const f of found.filter((w) => !wanted.includes(w))) {
    console.log(`NOT A FAULT at --widths ${widths}: ${f}`);
    wrong += 1;
  }
}
if (wrong > 0) {
  console.log(`The layout check is wrong ${wrong} times on its own fixtures. Fix layout.mjs or layout-measure.mjs, then run this again.`);
  process.exit(1);
}
console.log("The frame, reading, table and stylesheet checks find every fault in the fixtures and nothing else, at 390, 1440 and 2000 px.");
