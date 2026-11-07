// The fallback check: the demos as a browser that cannot read AVIF sees
// them, such as Safari before version 16 or an old in-app browser. The
// demos make no WebP (`avif = "only"`, scripts/demo-media.sh), so such
// a browser gets each picture's one JPEG or PNG. This fails when any
// picture on a page then shows nothing.
//
// Chromium reads AVIF and has no switch to stop, so each page is changed
// before the browser reads it: every <source type="image/avif"> is taken
// out, which is what a browser without AVIF does when it skips a source
// whose type it cannot read. Any request for an AVIF file that still
// happens, such as from a stylesheet, fails the check too.
//
// Each site is opened at a 390 px phone, scrolled to the bottom a window
// at a time, with every lazy picture asked for at once. A picture fails
// when it did not load, loaded nothing (no width), or is an AVIF file.
//
// Usage: node fallback.mjs --sites <folder> [--packs keystone,wire]
// Each folder in --sites holds a site.toml and the site built into public/.
// A pack's demo publication, <pack>-publication, is checked on the pages
// its check-pages.txt names, the showcase <pack> on its home page, a post
// and a category page. Folders ending in -side-contents or -plain are left
// out.
import { existsSync } from "node:fs";
import { createRequire } from "node:module";
import { answerFrom, builtSites } from "./site-files.mjs";

const SHOWCASE_PAGES = ["", "content-benchmark/", "category/content-strategy/"];
const AVIF_SOURCE = /<source\b[^>]*\btype="image\/avif"[^>]*>\s*/g;

const options = {};
for (let i = 2; i < process.argv.length; i += 1) {
  options[process.argv[i].replace(/^--/, "")] = process.argv[i + 1] ?? "";
  i += 1;
}
if (!options.sites || !existsSync(options.sites)) {
  console.log("Name the folder of built sites with --sites, such as target/page-weight/sites after scripts/page-weight.sh.");
  process.exit(2);
}
const only = (options.packs || "").split(",").map((s) => s.trim()).filter(Boolean);
const packOf = (name) => name.replace(/-publication$/, "");
const sites = builtSites(options.sites).filter(
  (site) => !/-(side-contents|plain)$/.test(site.name) && (only.length === 0 || only.includes(packOf(site.name))),
);
if (sites.length === 0) {
  console.log("No built site matches, so there is nothing to check.");
  process.exit(2);
}

// Asks for every lazy picture at once, since one beside the window, such
// as in a row that scrolls sideways, may never come near it, then scrolls
// down the page a window at a time and waits for every picture to finish,
// for at most 5 seconds.
const walkDown = async () => {
  for (const img of document.images) img.loading = "eager";
  const step = Math.floor(window.innerHeight * 0.8);
  for (let y = 0; y < document.documentElement.scrollHeight; y += step) {
    window.scrollTo(0, y);
    await new Promise((done) => setTimeout(done, 40));
  }
  const settle = (img) =>
    new Promise((done) => {
      img.addEventListener("load", done, { once: true });
      img.addEventListener("error", done, { once: true });
    });
  const settled = [...document.images].filter((img) => !img.complete).map(settle);
  await Promise.race([Promise.all(settled), new Promise((done) => setTimeout(done, 5000))]);
};

// Every picture the page draws that shows nothing, or shows an AVIF file.
// A picture inside something hidden, such as a closed menu, is not drawn,
// so a lazy one never loads, and it is left out.
const brokenPictures = () =>
  [...document.images]
    .filter((img) => img.getClientRects().length > 0)
    .filter((img) => !img.complete || img.naturalWidth === 0 || /\.avif(\?|#|$)/i.test(img.currentSrc))
    .map((img) => img.currentSrc || img.getAttribute("src") || "(no address)");

const playwright = createRequire(import.meta.url)("playwright");
const browser = await playwright.chromium.launch();
const faults = [];
let pages = 0;
let pictures = 0;
try {
  for (const site of sites) {
    const context = await browser.newContext({ viewport: { width: 390, height: 844 }, deviceScaleFactor: 1 });
    const answer = answerFrom([site], { page: (html) => html.replace(AVIF_SOURCE, "") });
    const avifAsked = [];
    await context.route("**/*", (route) => {
      if (/\.avif$/i.test(new URL(route.request().url()).pathname)) {
        avifAsked.push(route.request().url());
        return route.abort("blockedbyclient");
      }
      return answer(route);
    });
    const tab = await context.newPage();
    const paths = site.checkPages ? site.checkPages.map(([, path]) => path) : SHOWCASE_PAGES;
    for (const path of paths) {
      const address = `${site.origin}${site.mount}${path}`;
      const response = await tab.goto(address, { waitUntil: "load" });
      if (!response || response.status() !== 200) {
        faults.push(`${site.name} /${path}: the page answered ${response?.status() ?? "nothing"}.`);
        continue;
      }
      await tab.evaluate(walkDown);
      pages += 1;
      pictures += await tab.evaluate(() => [...document.images].filter((img) => img.getClientRects().length > 0).length);
      for (const src of await tab.evaluate(brokenPictures)) {
        faults.push(`${site.name} /${path}: a picture shows nothing without AVIF (${src}).`);
      }
    }
    for (const url of new Set(avifAsked)) {
      faults.push(`${site.name}: an AVIF file is fetched with no fallback (${url}).`);
    }
    await context.close();
  }
} finally {
  await browser.close();
}
if (faults.length > 0) {
  console.log(`Without AVIF, ${faults.length} pictures or pages fail:`);
  for (const fault of faults) console.log(`  ${fault}`);
  console.log("Each picture needs a JPEG or PNG in its <img> for browsers without AVIF. Check that khalid-media still makes the fallback file and that the template's picture keeps it.");
  process.exit(1);
}
console.log(`Without AVIF, all ${pictures} pictures on ${pages} pages of ${sites.length} sites show their JPEG or PNG.`);
