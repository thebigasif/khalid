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
//     breaks the word mid-way;
//   - squashed-avatar: a person's photo not drawn square, within 1 px;
//   - frame-fill: at 1440 px and wider, the site header, or <main> on the
//     home page or a list of posts, is narrower than the minimum frame the
//     template's design brief sets, read from --frames;
//   - reading-size, reading-column: at 1440 px and wider, a post's body
//     text is under 18 px, or a full line of it holds fewer than 60 or
//     more than 75 characters;
//   - stylesheet-size: a stylesheet the home page links is 20 KB or more
//     with Brotli, checked once per pack.
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
//          [--widths 390,768,1440,2000] [--packs keystone,wire] [--shots <folder>]
//          [--tabs 4] [--frames frames.json]
// Each folder in --sites holds a site.toml and the site built into public/.
// A folder whose name ends in -side-contents is a pack built with its
// contents in a side column, and only its posts are checked.
// --frames names a JSON file whose "frames" maps each pack to its minimum
// frame in px; by default frames.json beside this script.
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { brotliCompressSync, constants } from "node:zlib";
import { frame, measure, reading, walkDownScript } from "./layout-measure.mjs";
import { answerFrom, builtSites } from "./site-files.mjs";

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
// The home page and lists of posts, dense by design, whose <main> fills the
// frame. Posts are held to the reading check instead, and the author,
// search and glossary pages are a single column of text.
const LISTS = new Set(["home", "category", "tag"]);
const SIDE = "-side-contents";
const walkDown = walkDownScript();
const STEP_LIMIT = 90_000;
// The frame and reading checks start at this width, and the bounds they
// hold a post's reading text to (FRONTEND.md, R4 of the design plan).
const WIDE = 1440;
const READING = { size: 18, fewest: 60, most: 75 };
const CSS_BUDGET = 20 * 1024;

const options = {};
for (let i = 2; i < process.argv.length; i += 1) {
  const key = process.argv[i].replace(/^--/, "");
  options[key] = process.argv[i + 1] ?? "";
  i += 1;
}
const list = (value, fallback) => (value || fallback).split(",").map((s) => s.trim()).filter(Boolean);
const browsers = list(options.browsers, "chromium,firefox,webkit");
const widths = list(options.widths, "390,768,1440,2000").map(Number);
const packs = options.packs ? new Set(list(options.packs)) : null;
const tabs = Number(options.tabs) || 4;
const shots = options.shots;
if (!options.sites || !existsSync(options.sites)) {
  console.log("Name the folder of built sites with --sites. scripts/layout-check.sh builds it and runs this.");
  process.exit(2);
}
// Playwright comes from scripts/demo-check/node_modules, or from NODE_PATH.
const playwright = createRequire(import.meta.url)("playwright");

const framesFile = options.frames || join(dirname(fileURLToPath(import.meta.url)), "frames.json");
const framesData = JSON.parse(readFileSync(framesFile, "utf8"));
const frames = framesData.frames ?? {};
// Templates whose reading check waits for their redesign (frames.json).
const readingPending = new Set(framesData.reading_pending ?? []);

// Every site to check: its pack, address, mount, files and pages.
const sites = builtSites(options.sites)
  .map((site) => {
    const side = site.name.endsWith(SIDE);
    const pages = PAGES.filter(([what]) => !side || POSTS.has(what));
    return { ...site, pack: side ? site.name.slice(0, -SIDE.length) : site.name, side, pages };
  })
  .filter((site) => !packs || packs.has(site.pack));
const answer = answerFrom(sites);

// The checks of a wide window: the frame the page fills against the
// minimum in --frames, and on a post its reading text.
const wideChecks = async (tab, site, what) => {
  const problems = [];
  const least = frames[site.pack];
  const sizes = await tab.evaluate(frame);
  const parts = LISTS.has(what) ? ["header", "main"] : ["header"];
  if (typeof least !== "number") {
    problems.push({ check: "frame-fill", a: site.pack, b: "", detail: `${framesFile} sets no minimum frame for this pack; add the one its design brief sets` });
  }
  for (const part of typeof least === "number" ? parts : []) {
    if (sizes[part] < least) {
      problems.push({ check: "frame-fill", a: part === "header" ? "<header.site-header>" : "<main>", b: "", detail: `what it draws is ${sizes[part]} px wide, and the template's brief sets a frame of at least ${least} px` });
    }
  }
  if (!POSTS.has(what) || readingPending.has(site.pack)) return problems;
  const text = await tab.evaluate(reading);
  if (!text) {
    problems.push({ check: "reading-size", a: "<main>", b: "", detail: "the post has no .post-body with paragraphs to measure" });
  } else {
    if (text.size < READING.size) problems.push({ check: "reading-size", a: "<div.post-body>", b: "", detail: `its paragraphs are ${text.size} px, and post text needs at least ${READING.size} px on a wide screen` });
    if (text.chars < READING.fewest || text.chars > READING.most) {
      problems.push({ check: "reading-column", a: "<div.post-body>", b: "", detail: `a full line holds about ${text.chars} characters in a ${text.column} px column, and a comfortable line holds ${READING.fewest} to ${READING.most}` });
    }
  }
  return problems;
};

// Each stylesheet a pack's home page links, compressed with Brotli at its
// best quality, as Khalid serves it, against the 20 KB budget.
const stylesheetChecks = (site) => {
  const home = readFileSync(join(site.files, "index.html"), "utf8");
  const found = [];
  for (const [, href] of home.matchAll(/<link rel="stylesheet" href="([^"]+)"/g)) {
    const path = new URL(href, `${site.origin}${site.mount}`).pathname;
    const file = join(site.files, decodeURIComponent(path.slice(site.mount.length)));
    if (!path.startsWith(site.mount) || !existsSync(file)) {
      found.push({ check: "stylesheet-size", a: href, b: "", detail: "the home page links it, and the build has no such file" });
      continue;
    }
    const size = brotliCompressSync(readFileSync(file), { params: { [constants.BROTLI_PARAM_QUALITY]: 11 } }).length;
    if (size >= CSS_BUDGET) found.push({ check: "stylesheet-size", a: href, b: "", detail: `${(size / 1024).toFixed(1)} KB with Brotli, and a stylesheet must stay under 20 KB` });
  }
  return found.map((p) => ({ pack: site.name, page: "home", browser: "", width: 0, ...p }));
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
  if (width >= WIDE) found.push(...(await wideChecks(tab, site, what)));
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
for (const site of sites.filter((s) => !s.side)) failures.push(...stylesheetChecks(site));
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
  const where = [f.pack, f.page, f.browser, f.width && `${f.width} px`].filter(Boolean).join(", ");
  console.log(`${where}: ${f.check}: ${f.a}${f.b ? ` and ${f.b}` : ""} (${f.detail})`);
}
if (failures.length > 0) {
  const checks = [...new Set(failures.map((f) => f.check))].map((c) => `${failures.filter((f) => f.check === c).length} ${c}`);
  console.log(`${failures.length} layout problems in ${seconds()} seconds: ${checks.join(", ")}. A reader on those pages sees them. Fix each one in the pack or block style that draws it, then run just layout-check again.`);
  process.exit(1);
}
console.log(`Every page of every pack lays out cleanly in ${browsers.join(", ")} at ${widths.join(", ")} px, checked in ${seconds()} seconds.`);
