// The network capture of scripts/layout-check.sh (privacy plan U10, R1):
// a reader who has not answered the consent banner must load nothing from
// any address but the site's own. Opens pages of every built site in
// Chromium with no consent cookie, once with scripts on and once with
// scripts off, and fails on each request to another address: a player
// frame, an analytics script, a font or a picture from someone else.
//
// The pages of each site are its home page, the pages its check-pages.txt
// names (site-files.mjs), or the showcase's post with every block where it
// has none, and every page that holds an embed, found in its files, since
// only an embed writes a frame. Each page is drawn as tall as itself, so
// every lazy frame and picture on it loads as it would for a reader who
// scrolls to the end.
//
// Usage: node network.mjs --sites <folder> [--packs keystone,wire] [--tabs 4]
// Each folder in --sites holds a site.toml and the site built into public/.
// Copies built with the contents in a side column hold the same pages and
// are left out. Every request found goes to network.json beside the sites
// folder. Exits 1 when a page asked another address for anything.
import { existsSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join, relative, sep } from "node:path";
import { answerFrom, builtSites } from "./site-files.mjs";

const SIDE = "-side-contents";
// The showcase post with every block, for a site with no check-pages.txt.
const SHOWCASE_POST = "ai-search-guide/";
// The tallest a page is drawn, in px, so a very long page stays cheap.
const TALLEST = 40_000;
const STEP_LIMIT = 90_000;

const options = {};
for (let i = 2; i < process.argv.length; i += 1) {
  options[process.argv[i].replace(/^--/, "")] = process.argv[i + 1] ?? "";
  i += 1;
}
if (!options.sites || !existsSync(options.sites)) {
  console.log("Name the folder of built sites with --sites. scripts/layout-check.sh builds it and runs this.");
  process.exit(2);
}
const packs = options.packs ? new Set(options.packs.split(",").map((s) => s.trim())) : null;
const tabs = Number(options.tabs) || 4;
// Playwright comes from scripts/demo-check/node_modules, or from NODE_PATH.
const { chromium } = createRequire(import.meta.url)("playwright");

// Every index.html under `folder`, as paths after the site's mount.
const htmlPages = (folder, under = folder) =>
  readdirSync(under).flatMap((name) => {
    const path = join(under, name);
    if (statSync(path).isDirectory()) return htmlPages(folder, path);
    return name === "index.html" ? [relative(folder, under).split(sep).join("/").replace(/(.)$/, "$1/")] : [];
  });

// The pages of `site` to open, the home page first, each once.
const pagesOf = (site) => {
  const named = site.checkPages?.map(([, path]) => path) ?? ["", SHOWCASE_POST];
  const embeds = htmlPages(site.files)
    .filter((path) => readFileSync(join(site.files, path, "index.html"), "utf8").includes('<figure class="embed '))
    .sort();
  return [...new Set(["", ...named, ...embeds])].filter((path) => existsSync(join(site.files, path, "index.html")));
};

const sites = builtSites(options.sites)
  .filter((site) => !site.name.endsWith(SIDE))
  .filter((site) => !packs || packs.has(site.name.replace(/-publication$/, "")))
  .map((site) => ({ ...site, pages: pagesOf(site) }));
const answer = answerFrom(sites);
const own = new Set(sites.map((site) => site.origin));

// `work` finished, or a failure naming `what` once STEP_LIMIT has passed.
const limited = (what, work) => {
  let timer;
  const late = new Promise((_, reject) => {
    timer = setTimeout(() => reject(new Error(`${what} took longer than ${STEP_LIMIT / 1000} seconds`)), STEP_LIMIT);
  });
  return Promise.race([work, late]).finally(() => clearTimeout(timer));
};

const found = [];
const started = Date.now();

// Opens `url` in `tab` and draws it as tall as itself, so its lazy parts
// load. The requests are seen by the context's listener.
const open = async (tab, url) => {
  await tab.setViewportSize({ width: 1280, height: 900 });
  const response = await tab.goto(url, { waitUntil: "load" });
  if (!response || response.status() !== 200) throw new Error(`the page answered ${response?.status() ?? "nothing"}`);
  const height = await tab.evaluate(() => document.documentElement.scrollHeight);
  await tab.setViewportSize({ width: 1280, height: Math.min(Math.max(height, 900), TALLEST) });
  await tab.waitForLoadState("networkidle", { timeout: 10_000 }).catch(() => {});
};

// Every page of every site with `scripts` on or off, a few tabs at once.
const run = async (browser, scripts) => {
  const setting = `scripts ${scripts ? "on" : "off"}`;
  const context = await browser.newContext({ javaScriptEnabled: scripts, bypassCSP: true });
  await context.route("**/*", answer);
  const queue = sites.flatMap((site) => site.pages.map((path) => ({ site, path })));
  const one = async () => {
    let tab = await context.newPage();
    let current = null;
    tab.on("request", (request) => watch(request, () => current));
    for (let job = queue.shift(); job; job = queue.shift()) {
      current = { ...job, setting };
      const url = `${job.site.origin}${job.site.mount}${job.path}`;
      try {
        await limited(`${url} with ${setting}`, open(tab, url));
      } catch (error) {
        found.push({ site: job.site.name, page: `/${job.path}`, setting, request: "", detail: error.message.split("\n")[0] });
        await tab.close().catch(() => {});
        tab = await context.newPage();
        tab.on("request", (request) => watch(request, () => current));
      }
    }
    current = null;
    await tab.close().catch(() => {});
  };
  await Promise.all(Array.from({ length: tabs }, one));
  await context.close();
  console.log(`  ${setting} done (${Math.round((Date.now() - started) / 1000)} s)`);
};

// Notes `request` when it goes to an address no site here has, against
// the page `at()` names. Pictures written into the page itself are fine.
const watch = (request, at) => {
  const where = at();
  const url = request.url();
  if (!where || !/^https?:/.test(url) || own.has(new URL(url).origin)) return;
  found.push({ site: where.site.name, page: `/${where.path}`, setting: where.setting, request: url, detail: request.resourceType() });
};

const total = sites.reduce((n, site) => n + site.pages.length, 0);
console.log(`Network capture: ${sites.length} sites, ${total} pages, each with scripts on and off, no consent given.`);
const browser = await chromium.launch();
try {
  for (const scripts of [true, false]) await run(browser, scripts);
} finally {
  await browser.close();
}

writeFileSync(join(options.sites, "..", "network.json"), `${JSON.stringify(found, null, 1)}\n`);
for (const f of found) {
  console.log(`${f.site}, ${f.page}, ${f.setting}: ${f.request ? `asked ${f.request} for a ${f.detail}` : f.detail}`);
}
if (found.length > 0) {
  console.log(`${found.length} requests left the site before the reader agreed to anything. Each one shares the reader's address with someone else, which privacy law forbids without consent. Hold it behind the consent banner or an embed card, then run just layout-check again.`);
  process.exit(1);
}
console.log(`No page of ${sites.length} sites asked another address for anything, with scripts on or off, in ${Math.round((Date.now() - started) / 1000)} seconds.`);
