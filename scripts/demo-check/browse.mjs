// The browser half of scripts/demo-check.sh. Opens every page at six widths,
// first with scripts on and then with scripts off.
//
// With scripts on, it fails on any axe violation (BL-WEB-4), a page wider
// than its window (BL-WEB-3), or a page that is not whole: no main landmark
// or not exactly one h1 (BL-WEB-1). It also notes the text of the main
// landmark and every link a reader can see.
//
// With scripts off, it runs the same layout checks and then proves the page
// reads without JavaScript (BL-WEB-2): the main text and the visible links
// must be the same as with scripts on, at every width. axe does not run
// there. axe is itself a script the page runs, and a page with scripts off
// cannot run it.
//
// Each page loads once per scripts setting and is then resized through the
// widths, so every check runs at each width without a reload. A few pages
// are checked at once, one browser tab each, and a page that takes longer
// than STEP_LIMIT fails by name instead of stalling the whole run.
import { availableParallelism } from "node:os";
import { readFileSync } from "node:fs";
import AxeBuilder from "@axe-core/playwright";
import { chromium } from "playwright";

const WIDTHS = [320, 375, 768, 1024, 1280, 1920];
const HEIGHT = 900;
const TABS = Math.max(1, Math.min(4, availableParallelism()));
const STEP_LIMIT = 60_000;

const pages = readFileSync(process.argv[2], "utf8").split("\n").filter(Boolean);
const failures = [];
const started = Date.now();
// What each page showed with scripts on, by url and then width.
const withScripts = new Map();

// `work` finished, or a failure naming `what` once STEP_LIMIT has passed.
const limited = (what, work) => {
  let timer;
  const late = new Promise((_, reject) => {
    timer = setTimeout(() => reject(new Error(`${what} took longer than ${STEP_LIMIT / 1000} seconds`)), STEP_LIMIT);
  });
  return Promise.race([work, late]).finally(() => clearTimeout(timer));
};

// The page's shape and what a reader sees in it, read in one synchronous
// pass, which works with the page's own scripts off.
const read = () => {
  const root = document.documentElement;
  const shown = (element) => element.getClientRects().length > 0 && getComputedStyle(element).visibility !== "hidden";
  const links = [...document.querySelectorAll("a[href]")].filter(shown).map((a) => a.getAttribute("href"));
  return {
    wider: root.scrollWidth - root.clientWidth,
    h1: document.querySelectorAll("h1").length,
    main: document.querySelectorAll("main").length,
    text: (document.querySelector("main")?.innerText ?? "").replace(/\s+/g, " ").trim(),
    links: [...new Set(links)].sort(),
  };
};

// The text around the first place where `off` stops matching `on`.
const firstDifference = (on, off) => {
  let at = 0;
  while (at < on.length && on[at] === off[at]) at += 1;
  return on.slice(Math.max(0, at - 20), at + 40);
};

// What a reader without scripts misses at `where`, compared with `on`.
const compare = (where, on, off) => {
  if (off.text !== on.text) {
    failures.push(`${where}: the main text differs from the page with scripts on, near "${firstDifference(on.text, off.text)}"`);
  }
  const offLinks = new Set(off.links);
  const missing = on.links.filter((href) => !offLinks.has(href));
  if (missing.length > 0) {
    failures.push(`${where}: ${missing.length} links a reader sees with scripts on are missing or hidden, such as ${missing.slice(0, 3).join(", ")}`);
  }
};

// Every check of `url` at every width, in `page`, which has `scripts` set.
const checkPage = async (page, url, scripts) => {
  const setting = `scripts ${scripts ? "on" : "off"}`;
  const readings = new Map();
  await page.setViewportSize({ width: WIDTHS[0], height: HEIGHT });
  await limited(`Loading ${url} with ${setting}`, page.goto(url, { waitUntil: "load" }));
  for (const width of WIDTHS) {
    const where = `${url} at ${width} px with ${setting}`;
    await page.setViewportSize({ width, height: HEIGHT });
    // Reading the widths lays the page out again at the new size first.
    const seen = await limited(`Reading ${where}`, page.evaluate(read));
    if (seen.wider > 0) failures.push(`${where}: the page is ${seen.wider} px wider than the window`);
    if (seen.h1 !== 1) failures.push(`${where}: the page has ${seen.h1} h1 headings, not one`);
    if (seen.main !== 1) failures.push(`${where}: the page has ${seen.main} main landmarks, not one`);
    if (scripts) {
      readings.set(width, seen);
      const { violations } = await limited(`axe on ${where}`, new AxeBuilder({ page }).analyze());
      for (const v of violations) {
        failures.push(`${where}: axe ${v.id}, ${v.help} (${v.nodes.length} places, ${v.helpUrl})`);
      }
    } else {
      const on = withScripts.get(url)?.get(width);
      if (on) compare(where, on, seen);
      else failures.push(`${where}: there is nothing to compare with, because the check with scripts on did not finish`);
    }
  }
  if (scripts) withScripts.set(url, readings);
};

// One tab's share of the pages: it takes the next unchecked page until
// none is left. A tab whose page timed out is replaced, so a stuck page
// cannot hold up the ones after it.
const tab = async (context, queue, scripts) => {
  let page = await context.newPage();
  for (let url = queue.shift(); url; url = queue.shift()) {
    try {
      await checkPage(page, url, scripts);
    } catch (error) {
      failures.push(`${url} with scripts ${scripts ? "on" : "off"}: ${error.message}`);
      await page.close().catch(() => {});
      page = await context.newPage();
    }
    const seconds = Math.round((Date.now() - started) / 1000);
    console.log(`  checked ${url} with scripts ${scripts ? "on" : "off"} (${seconds} s)`);
  }
  await page.close().catch(() => {});
};

const browser = await chromium.launch();
try {
  // Scripts on goes first, because the run with scripts off is compared
  // with what it saw.
  for (const scripts of [true, false]) {
    const context = await browser.newContext({ javaScriptEnabled: scripts });
    const queue = [...pages];
    await Promise.all(Array.from({ length: TABS }, () => tab(context, queue, scripts)));
    await context.close();
  }
} finally {
  await browser.close();
}

for (const failure of failures) console.log(failure);
if (failures.length > 0) {
  console.log(`${failures.length} browser problems found on ${pages.length} pages. Readers on those widths meet them. Fix each one in the crate that renders it, then run scripts/demo-check.sh again.`);
  process.exit(1);
}
console.log(`All ${pages.length} pages pass axe and the layout checks at six widths with scripts on, and read the same with scripts off.`);
