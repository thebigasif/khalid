// The browser half of scripts/demo-check.sh. Opens every page at six widths,
// first with scripts on and then with scripts off, and fails on any axe
// violation (BL-WEB-4), a page wider than its window (BL-WEB-3), or a page
// that is not whole: no main landmark or not exactly one h1 (BL-WEB-1,
// BL-WEB-2).
//
// Each page loads once per scripts setting and is then resized through the
// widths, so the layout and axe run at each width without a reload. A few
// pages are checked at once, one browser tab each, and a page that takes
// longer than STEP_LIMIT fails by name instead of stalling the whole run.
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

// `work` finished, or a failure naming `what` once STEP_LIMIT has passed.
const limited = (what, work) => {
  let timer;
  const late = new Promise((_, reject) => {
    timer = setTimeout(() => reject(new Error(`${what} took longer than ${STEP_LIMIT / 1000} seconds`)), STEP_LIMIT);
  });
  return Promise.race([work, late]).finally(() => clearTimeout(timer));
};

// Every check of `url` at every width, in `page`, which has `scripts` set.
const checkPage = async (page, url, scripts) => {
  const setting = `scripts ${scripts ? "on" : "off"}`;
  await page.setViewportSize({ width: WIDTHS[0], height: HEIGHT });
  await limited(`Loading ${url} with ${setting}`, page.goto(url, { waitUntil: "load" }));
  for (const width of WIDTHS) {
    const where = `${url} at ${width} px with ${setting}`;
    await page.setViewportSize({ width, height: HEIGHT });
    // Reading the widths lays the page out again at the new size first.
    const shape = await page.evaluate(() => ({
      wider: document.documentElement.scrollWidth - document.documentElement.clientWidth,
      h1: document.querySelectorAll("h1").length,
      main: document.querySelectorAll("main").length,
    }));
    if (shape.wider > 0) failures.push(`${where}: the page is ${shape.wider} px wider than the window`);
    if (shape.h1 !== 1) failures.push(`${where}: the page has ${shape.h1} h1 headings, not one`);
    if (shape.main !== 1) failures.push(`${where}: the page has ${shape.main} main landmarks, not one`);
    const { violations } = await limited(`axe on ${where}`, new AxeBuilder({ page }).analyze());
    for (const v of violations) {
      failures.push(`${where}: axe ${v.id}, ${v.help} (${v.nodes.length} places, ${v.helpUrl})`);
    }
  }
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
console.log(`All ${pages.length} pages pass axe and the layout checks at six widths, with scripts on and off.`);
