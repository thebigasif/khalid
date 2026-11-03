// The browser half of scripts/layout-check.sh. Opens the same pages of the
// showcase in every template pack, in Chromium, Firefox and WebKit, at each
// width, and fails on what a reader would see as broken (BL-WEB-3):
//
//   - text-over-text, text-over-picture: the glyphs of one piece of text
//     cover another piece of text or a picture by more than 4 px both ways;
//   - marker-over-text: a list marker, the browser's ::marker or a ::before
//     a template draws, covers the first letter of its item, or other text;
//   - sideways-scroll: the page is wider than the window;
//   - broken-picture: a picture that did not load;
//   - crushed-cell: a table column narrower than its longest word, which
//     breaks the word mid-way.
//
// The page is scrolled down a window at a time first, so lazy pictures
// load. At each stop the parts that move with the window, fixed like the
// floating contents button or sticky like the side contents column, are
// checked against what is under them; then the whole page is checked from
// the top. Each failure names the pack, page, browser, width and the two
// elements, with where on the page they meet. A screenshot of each failing
// page, one window around its first problem, goes to --shots, and every
// failure to report.json beside the sites folder.
//
// Usage: node layout.mjs --sites <folder> [--browsers chromium,firefox,webkit]
//          [--widths 390,768,1440] [--packs keystone,wire] [--shots <folder>]
//          [--tabs 4]
// Each folder in --sites holds a site.toml and the site built into public/.
// A folder whose name ends in -side-contents is a pack built with its
// contents in a side column, and only its posts are checked.
import { existsSync, mkdirSync, readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { extname, join } from "node:path";
import { measure, walkDownScript } from "./layout-measure.mjs";

// The pages each pack is checked on, by what they show.
const PAGES = [
  ["home", ""],
  // A lead picture, a contents list, pros and cons and wide tables.
  ["post", "ai-search-guide/"],
  // A long table folded after its first rows, with long words in it.
  ["folded table", "content-benchmark-data/"],
  ["category", "category/seo-writing/"],
  ["tag", "tag/ai-search/"],
  ["author", "authors/maren-okafor/"],
  ["search", "search/"],
  ["glossary", "glossary/answer-capsule/"],
];
const POSTS = new Set(["post", "folded table"]);
const SIDE = "-side-contents";
const walkDown = walkDownScript();
const STEP_LIMIT = 90_000;
const TYPES = { ".html": "text/html; charset=utf-8", ".css": "text/css", ".js": "text/javascript", ".mjs": "text/javascript", ".svg": "image/svg+xml", ".png": "image/png", ".jpg": "image/jpeg", ".jpeg": "image/jpeg", ".webp": "image/webp", ".avif": "image/avif", ".gif": "image/gif", ".woff2": "font/woff2", ".json": "application/json", ".xml": "application/xml", ".txt": "text/plain", ".ico": "image/x-icon", ".mp4": "video/mp4", ".webm": "video/webm", ".vtt": "text/vtt" };

const options = {};
for (let i = 2; i < process.argv.length; i += 1) {
  const key = process.argv[i].replace(/^--/, "");
  options[key] = process.argv[i + 1] ?? "";
  i += 1;
}
const list = (value, fallback) => (value || fallback).split(",").map((s) => s.trim()).filter(Boolean);
const browsers = list(options.browsers, "chromium,firefox,webkit");
const widths = list(options.widths, "390,768,1440").map(Number);
const packs = options.packs ? new Set(list(options.packs)) : null;
const tabs = Number(options.tabs) || 4;
const shots = options.shots;
if (!options.sites || !existsSync(options.sites)) {
  console.log("Name the folder of built sites with --sites. scripts/layout-check.sh builds it and runs this.");
  process.exit(2);
}
// Playwright comes from scripts/demo-check/node_modules, or from NODE_PATH.
const playwright = createRequire(import.meta.url)("playwright");

// Every site to check: its pack, address, mount, files and pages.
const sites = readdirSync(options.sites)
  .filter((name) => existsSync(join(options.sites, name, "public")))
  .sort()
  .map((name) => {
    const toml = readFileSync(join(options.sites, name, "site.toml"), "utf8");
    const field = (key) => toml.match(new RegExp(`^${key} = "(.*)"`, "m"))?.[1] ?? "";
    const pack = name.endsWith(SIDE) ? name.slice(0, -SIDE.length) : name;
    const pages = PAGES.filter(([what]) => !name.endsWith(SIDE) || POSTS.has(what));
    return { name, pack, origin: new URL(field("address")).origin, mount: field("mount") || "/", files: join(options.sites, name, "public"), pages };
  })
  .filter((site) => !packs || packs.has(site.pack));
const byOrigin = new Map(sites.map((site) => [site.origin, site]));

// Answers a request for a site's own file from its folder. Nothing leaves
// the computer: a request to any other address is refused.
const answer = async (route) => {
  const url = new URL(route.request().url());
  const site = byOrigin.get(url.origin);
  if (!site) return route.abort("blockedbyclient");
  const path = decodeURIComponent(url.pathname);
  let file = path.startsWith(site.mount) ? join(site.files, path.slice(site.mount.length)) : "";
  if (file && existsSync(file) && statSync(file).isDirectory()) file = join(file, "index.html");
  if (!file || !existsSync(file)) return route.fulfill({ status: 404, contentType: TYPES[".html"], body: "Not found" });
  return route.fulfill({ status: 200, contentType: TYPES[extname(file)] ?? "application/octet-stream", body: readFileSync(file) });
};

// `work` finished, or a failure naming `what` once STEP_LIMIT has passed.
const limited = (what, work) => {
  let timer;
  const late = new Promise((_, reject) => {
    timer = setTimeout(() => reject(new Error(`${what} took longer than ${STEP_LIMIT / 1000} seconds`)), STEP_LIMIT);
  });
  return Promise.race([work, late]).finally(() => clearTimeout(timer));
};

const failures = [];
const started = Date.now();
const seconds = () => Math.round((Date.now() - started) / 1000);

// Every check of one page at one width, in `tab`.
const checkPage = async (tab, job) => {
  const { site, what, path, browser, width } = job;
  const url = `${site.origin}${site.mount}${path}`;
  const response = await tab.goto(url, { waitUntil: "load" });
  if (!response || response.status() !== 200) {
    return [{ check: "missing-page", a: url, b: "", detail: `the page answered ${response?.status() ?? "nothing"}, so the showcase no longer has it; pick another page in PAGES` }];
  }
  await tab.evaluate(() => document.fonts.ready.then(() => undefined));
  // Down the page a window at a time: lazy pictures load, and each moving
  // part is checked against what is under it at each stop.
  const found = await tab.evaluate(walkDown);
  await tab.evaluate(() => Promise.race([Promise.all([...document.images].map((img) => img.decode().catch(() => {}))), new Promise((done) => setTimeout(done, 3000))]));
  found.push(...(await tab.evaluate(measure, "page")));
  const problems = [];
  const seen = new Map();
  for (const p of found) {
    // A moving part covers many things on its way down: one line each.
    const key = p.moving ? `${p.check}|${p.a}` : `${p.check}|${p.a}|${p.b}`;
    const old = seen.get(key);
    if (!old) {
      const kept = { ...p, under: [p.b] };
      seen.set(key, kept);
      problems.push(kept);
    } else if (p.moving && !old.under.includes(p.b)) {
      old.under.push(p.b);
    }
  }
  for (const p of problems.filter((q) => q.moving && q.under.length > 1)) {
    const more = p.under.length - 3;
    p.b = `${p.under.slice(0, 3).join(", ")}${more > 0 ? ` and ${more} more` : ""}`;
    p.detail = `while scrolling, first at ${p.detail}`;
  }
  for (const p of problems) {
    delete p.under;
    delete p.moving;
  }
  if (problems.length > 0 && shots) {
    mkdirSync(shots, { recursive: true });
    const file = `${site.name}-${what.replace(/\s+/g, "-")}-${browser}-${width}.png`;
    // One window of the page around the first problem: a full page of a
    // long post is tens of thousands of pixels tall and slow to take.
    const at = problems.map((p) => p.detail.match(/at (-?\d+),(-?\d+)/)).find(Boolean);
    const top = at ? Math.max(0, Number(at[2]) - 300) : 0;
    const clip = { x: 0, y: top, width, height: 900 };
    await tab.screenshot({ path: join(shots, file), fullPage: true, clip }).catch(() => {});
  }
  return problems;
};

// One browser at one width: a few tabs take the next page until none is
// left. A tab whose page failed or timed out is replaced.
const run = async (browser, type, width) => {
  const context = await browser.newContext({ viewport: { width, height: 900 }, deviceScaleFactor: 1, bypassCSP: true });
  await context.route("**/*", answer);
  const queue = sites.flatMap((site) => site.pages.map(([what, path]) => ({ site, what, path, browser: type, width })));
  const one = async () => {
    let tab = await context.newPage();
    for (let job = queue.shift(); job; job = queue.shift()) {
      const where = { pack: job.site.name, page: job.what, browser: type, width };
      try {
        const problems = await limited(`${job.site.name} ${job.what} in ${type} at ${width} px`, checkPage(tab, job));
        for (const p of problems) failures.push({ ...where, ...p });
      } catch (error) {
        failures.push({ ...where, check: "did-not-finish", a: "", b: "", detail: error.message.split("\n")[0] });
        await tab.close().catch(() => {});
        tab = await context.newPage();
      }
    }
    await tab.close().catch(() => {});
  };
  await Promise.all(Array.from({ length: tabs }, one));
  await context.close();
  console.log(`  ${type} at ${width} px done (${seconds()} s)`);
};

const total = sites.reduce((n, site) => n + site.pages.length, 0) * widths.length * browsers.length;
console.log(`Layout check: ${sites.length} sites, ${total} page views in ${browsers.join(", ")} at ${widths.join(", ")} px.`);
await Promise.all(
  browsers.map(async (type) => {
    const browser = await playwright[type].launch();
    try {
      for (const width of widths) await run(browser, type, width);
    } finally {
      await browser.close();
    }
  }),
);

const order = (f) => [f.check, f.pack, PAGES.findIndex(([w]) => w === f.page), browsers.indexOf(f.browser), f.width];
failures.sort((x, y) => {
  const a = order(x);
  const b = order(y);
  for (let i = 0; i < a.length; i += 1) if (a[i] !== b[i]) return a[i] < b[i] ? -1 : 1;
  return 0;
});
writeFileSync(join(options.sites, "..", "report.json"), `${JSON.stringify(failures, null, 1)}\n`);
for (const f of failures) {
  console.log(`${f.pack}, ${f.page}, ${f.browser}, ${f.width} px: ${f.check}: ${f.a}${f.b ? ` and ${f.b}` : ""} (${f.detail})`);
}
if (failures.length > 0) {
  const checks = [...new Set(failures.map((f) => f.check))].map((c) => `${failures.filter((f) => f.check === c).length} ${c}`);
  console.log(`${failures.length} layout problems in ${seconds()} seconds: ${checks.join(", ")}. A reader on those pages sees them. Fix each one in the pack or block style that draws it, then run just layout-check again.`);
  process.exit(1);
}
console.log(`Every page of every pack lays out cleanly in ${browsers.join(", ")} at ${widths.join(", ")} px, checked in ${seconds()} seconds.`);
