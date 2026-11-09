// The browser half of scripts/demo-pictures.sh: full-page pictures of each
// template's demo for the founder to approve, laid out as one sheet per
// template beside the stored pictures of reference blogs of the same kind.
//
// For each pack, the home page, a long post and a category page, at each
// width, once in the pack's sample brand (the site <pack>) and once with
// the pack's own defaults and no theme (the site <pack>-plain). The sheet
// has one row per page and width: the branded demo, the pack default and
// the reference picture. Its tiles show the top of each picture and link
// to the whole one. It is written as <out>/<pack>/sheet.html and a PNG of
// it, sheet.png, beside the pictures.
//
// A reference picture is any image in <refs>/<pack>/, named such as
// <source>-<page>-<width>.png. The width in its name, such as 2000, picks
// the row's width, and "post", or "category" or "list", picks the page; one
// with neither is a home page. A row holds every reference that fits it,
// labelled by the start of its name. Pictures whose name holds no width go
// in a last row of their own.
//
// Usage: node pictures.mjs --sites <folder> --out <folder> [--refs <folder>]
//          [--packs keystone,wire] [--widths 2000,1440,390] [--browser chromium]
import { existsSync, mkdirSync, readdirSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join, relative, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { answerFrom, builtSites } from "./site-files.mjs";

const PAGES = [
  ["home", ""],
  ["post", "content-benchmark/"],
  ["category", "category/content-strategy/"],
];
const LOOKS = [
  ["brand", "Branded demo"],
  ["plain", "Pack default"],
];
const PLAIN = "-plain";
// The sheet's width, a tile's width and the height of a tile's window on
// its picture, in px.
const SHEET = 3200;
const TILE = 600;
const TALL = 1200;

const options = {};
for (let i = 2; i < process.argv.length; i += 1) {
  options[process.argv[i].replace(/^--/, "")] = process.argv[i + 1] ?? "";
  i += 1;
}
const list = (value, fallback) => (value || fallback).split(",").map((s) => s.trim()).filter(Boolean);
if (!options.sites || !existsSync(options.sites) || !options.out) {
  console.log("Name the folder of built sites with --sites and where the pictures go with --out. scripts/demo-pictures.sh builds the sites and runs this.");
  process.exit(2);
}
const widths = list(options.widths, "2000,1440,390").map(Number);
const out = resolve(options.out);
const sites = builtSites(options.sites);
const packs = sites.map((s) => s.name).filter((n) => !n.endsWith(PLAIN) && (!options.packs || list(options.packs).includes(n)));
const playwright = createRequire(import.meta.url)("playwright");
const browser = await playwright[options.browser || "chromium"].launch();
const started = Date.now();

// Scrolls down the page a window at a time, so lazy pictures load, and
// back to the top.
const walkDown = async () => {
  const step = Math.floor(window.innerHeight * 0.8);
  for (let y = 0; y < document.documentElement.scrollHeight; y += step) {
    window.scrollTo(0, y);
    await new Promise((done) => setTimeout(done, 40));
  }
  window.scrollTo(0, 0);
  await Promise.race([Promise.all([...document.images].map((img) => img.decode().catch(() => {}))), new Promise((done) => setTimeout(done, 3000))]);
};

// The pages pictured on `site`: PAGES, or for a demo publication the first
// page of each of those kinds its check-pages.txt names.
const pagesOf = (site) => PAGES.map(([page, path]) => [page, site.checkPages?.find(([kind]) => kind === page)?.[1] ?? path]);

// One pack's pictures at every width, by `${page}-${width}-${look}`.
const shoot = async (pack) => {
  const taken = new Map();
  for (const width of widths) {
    const context = await browser.newContext({ viewport: { width, height: 900 }, deviceScaleFactor: 1 });
    await context.route("**/*", answerFrom(sites));
    const tab = await context.newPage();
    for (const [look] of LOOKS) {
      const site = sites.find((s) => s.name === (look === "brand" ? pack : `${pack}${PLAIN}`));
      if (!site) continue;
      for (const [page, path] of pagesOf(site)) {
        const response = await tab.goto(`${site.origin}${site.mount}${path}`, { waitUntil: "load" });
        if (!response || response.status() !== 200) {
          console.log(`  ${pack}: ${path || "the home page"} answered ${response?.status() ?? "nothing"}, so it has no picture.`);
          continue;
        }
        await tab.evaluate(() => document.fonts.ready.then(() => undefined));
        await tab.evaluate(walkDown);
        const file = join(out, pack, `${page}-${width}-${look}.png`);
        await tab.screenshot({ path: file, fullPage: true });
        taken.set(`${page}-${width}-${look}`, file);
      }
    }
    await context.close();
  }
  return taken;
};

// The stored reference pictures of a pack, placed by their file names.
const references = (pack) => {
  const folder = options.refs ? join(options.refs, pack) : "";
  if (!folder || !existsSync(folder)) return { placed: new Map(), other: [] };
  const placed = new Map();
  const other = [];
  for (const name of readdirSync(folder).filter((n) => /\.(png|jpe?g|webp|avif)$/i.test(n)).sort()) {
    const width = widths.find((w) => new RegExp(`(^|\\D)${w}(\\D|$)`).test(name));
    const page = /post/i.test(name) ? "post" : /category|list/i.test(name) ? "category" : "home";
    if (!width) {
      other.push(join(folder, name));
      continue;
    }
    const key = `${page}-${width}`;
    placed.set(key, [...(placed.get(key) ?? []), join(folder, name)]);
  }
  return { placed, other };
};

const escape = (text) => text.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/"/g, "&quot;");
const tile = (pack, label, file, width) => {
  const wide = Math.min(TILE, width || TILE);
  if (!file) return `<figure style="width:${wide}px"><figcaption>${label}</figcaption><div class="none">No picture</div></figure>`;
  const src = escape(relative(join(out, pack), file));
  return `<figure style="width:${wide}px"><figcaption>${label}</figcaption><a href="${src}"><img src="${src}" alt="${escape(label)}" width="${wide}"></a></figure>`;
};

// The sheet of one pack, as HTML, then as a PNG of it.
const sheet = async (pack, taken) => {
  const { placed, other } = references(pack);
  const branded = sites.find((s) => s.name === pack);
  const rows = (branded ? pagesOf(branded) : PAGES).flatMap(([page, path]) =>
    widths.map((width) => {
      const tiles = LOOKS.map(([look, label]) => tile(pack, label, taken.get(`${page}-${width}-${look}`), width));
      const refs = placed.get(`${page}-${width}`) ?? [""];
      tiles.push(...refs.map((file) => tile(pack, file ? `Reference: ${file.split("/").pop().split(/[-_.]/)[0]}` : "Reference", file, width)));
      return `<section><h2>${page[0].toUpperCase()}${page.slice(1)}, /${path}, at ${width} px</h2><div class="row">${tiles.join("")}</div></section>`;
    }),
  );
  if (other.length > 0) rows.push(`<section><h2>Other reference pictures</h2><div class="row">${other.map((f) => tile(pack, "Reference", f, 0)).join("")}</div></section>`);
  const html = `<!doctype html><html lang="en"><head><meta charset="utf-8"><title>${pack}: pictures for review</title><style>
body{margin:0;padding:32px;width:${SHEET}px;box-sizing:border-box;font:16px/1.4 system-ui,sans-serif;color:#1a1a1a;background:#fff}
h1{font-size:32px;margin:0 0 8px}h2{font-size:20px;margin:32px 0 8px}
.row{display:flex;flex-wrap:wrap;gap:24px;align-items:flex-start}figure{margin:0}figcaption{font-weight:600;margin-bottom:6px}
figure a{display:block;max-height:${TALL}px;overflow:hidden;border:1px solid #c8c8c8}img{display:block;height:auto}
.none{height:120px;display:grid;place-items:center;border:1px dashed #b0b0b0;color:#555}
</style></head><body><h1>${pack}</h1><p>Each tile shows the top ${TALL} px of its picture; open a tile for the whole page.</p>${rows.join("")}</body></html>`;
  const page = join(out, pack, "sheet.html");
  writeFileSync(page, html);
  const tab = await browser.newPage({ viewport: { width: SHEET, height: 900 } });
  await tab.goto(pathToFileURL(page).href, { waitUntil: "load" });
  await tab.screenshot({ path: join(out, pack, "sheet.png"), fullPage: true });
  await tab.close();
};

try {
  for (const pack of packs) {
    mkdirSync(join(out, pack), { recursive: true });
    await sheet(pack, await shoot(pack));
    console.log(`  ${pack}: ${join(out, pack, "sheet.png")} (${Math.round((Date.now() - started) / 1000)} s)`);
  }
} finally {
  await browser.close();
}
console.log(`Pictures of ${packs.length} templates at ${widths.join(", ")} px are in ${out}. Open each template's sheet.html to review it.`);
