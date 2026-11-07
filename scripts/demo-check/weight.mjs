// The browser half of scripts/page-weight.sh: the bytes of pictures a
// reader downloads on each template's demo home page, held to a budget
// (docs/standards/performance.md, "Page weight").
//
// Each home page is opened in Chromium three times: at a 390 px phone with
// three device pixels per CSS pixel, as most phones sold now have, at the
// same phone with one, and at a 1440 px desktop window with one. Each time
// it is scrolled to
// the bottom a window at a time, so every lazy picture loads, as it would
// for a reader who reads to the end. Every picture the page fetches
// counts: <img>, <picture> sources, CSS backgrounds and icons. Fonts,
// stylesheets and the page itself do not.
//
// The first picture, the one the page fetches at once with high priority
// (fetchpriority="high"), is the hero. It is drawn as wide as the page and
// its size is a design choice, so the phone budgets leave it out. The
// desktop budget counts it.
//
// The budgets are the most any template may download, not a target:
//
//   - at 390 px and 3x, every picture but the hero together under
//     PHONE_3X_BUDGET, or the template's own line in PHONE_3X_OWN;
//   - at 390 px and 1x, every picture but the hero together under
//     PHONE_BUDGET;
//   - at 1440 px, every picture together under DESKTOP_BUDGET, or the
//     template's own line in DESKTOP_OWN, which only a template whose
//     design is large photographs has, with the reason beside it.
//
// A template over a budget almost always draws a picture smaller than the
// `sizes` its slot tells the browser, so the browser picks a file wider
// than it needs. The failure names the largest pictures to look at. A
// budget is never raised to make this check pass.
//
// Usage: node weight.mjs --sites <folder> [--packs keystone,wire]
//          [--report <file.json>]
// Each folder in --sites holds a site.toml and the site built into public/.
// A pack's demo publication, a folder named <pack>-publication, is the
// demo readers see, so it is measured in place of the showcase <pack>.
// Folders ending in -side-contents or -plain are left out.
import { existsSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { answerFrom, builtSites } from "./site-files.mjs";

const KB = 1024;
// The budgets, in bytes. The desktop budget was set on 2026-11-07 from the
// measurements in the pull request that added this check
// (perf/picture-sizes), with about 10% headroom over Atlas at 1,136 KB,
// fourteen photographs of which the two lead stories are drawn 808 px
// wide. The phone budgets were set the same day once the demos offered
// AVIF (perf/phone-pictures), with about 10% headroom: at 3x over
// Itinerary at 561 KB, at 1x over Aperture and Atlas at 125 KB. The 1x
// budget was 270 KB before.
const PHONE_3X_BUDGET = 620 * KB;
const PHONE_BUDGET = 140 * KB;
const DESKTOP_BUDGET = 1250 * KB;
// Templates whose design is large photographs, with their own budget.
// Aperture shows five stories, each one photograph across its 1,296 px
// frame, measured at 1,875 KB on a desktop and 734 KB on a phone at 3x.
const DESKTOP_OWN = { aperture: 2100 * KB };
const PHONE_3X_OWN = { aperture: 810 * KB };
// What is measured: a width in CSS px and the device pixels per CSS pixel.
// A phone at 3x draws a 326 px card with 978 pixels.
const PHONE = { width: 390, scale: 3 };
const PHONE_1X = { width: 390, scale: 1 };
const DESKTOP = { width: 1440, scale: 1 };
const PUBLICATION = "-publication";
const LEFT_OUT = ["-side-contents", "-plain"];

const options = {};
for (let i = 2; i < process.argv.length; i += 1) {
  options[process.argv[i].replace(/^--/, "")] = process.argv[i + 1] ?? "";
  i += 1;
}
const list = (value) => value.split(",").map((s) => s.trim()).filter(Boolean);
if (!options.sites || !existsSync(options.sites)) {
  console.log("Name the folder of built sites with --sites. scripts/page-weight.sh builds it and runs this.");
  process.exit(2);
}
const only = options.packs ? new Set(list(options.packs)) : null;

// One site per pack: its demo publication where it has one, otherwise the
// showcase in the pack.
const all = builtSites(options.sites).filter((site) => !LEFT_OUT.some((end) => site.name.endsWith(end)));
const names = new Set(all.map((site) => site.name));
const sites = all
  .map((site) => ({ ...site, pack: site.name.endsWith(PUBLICATION) ? site.name.slice(0, -PUBLICATION.length) : site.name }))
  .filter((site) => site.name.endsWith(PUBLICATION) || !names.has(`${site.name}${PUBLICATION}`))
  .filter((site) => !only || only.has(site.pack));
if (sites.length === 0) {
  console.log(`${options.sites} holds no built site to measure. Build the demos first; scripts/page-weight.sh does.`);
  process.exit(2);
}

// Scrolls the page to the bottom a window at a time, waiting at each stop
// for the pictures that came into view, then back to the top.
const readToTheEnd = async (tab) => {
  await tab.evaluate(async () => {
    const step = Math.max(200, Math.floor(window.innerHeight * 0.8));
    for (let y = 0; y < document.documentElement.scrollHeight; y += step) {
      window.scrollTo(0, y);
      await new Promise((done) => setTimeout(done, 60));
    }
    window.scrollTo(0, document.documentElement.scrollHeight);
  });
  await tab.waitForLoadState("networkidle");
};

// Every picture the home page of `site` downloads in `view`, as
// { url, bytes, hero }, largest first.
const measure = async (browser, site, view) => {
  const context = await browser.newContext({ viewport: { width: view.width, height: 900 }, deviceScaleFactor: view.scale });
  await context.route("**/*", answerFrom([site]));
  const tab = await context.newPage();
  const pictures = new Map();
  tab.on("response", async (response) => {
    if (response.request().resourceType() !== "image" || response.status() !== 200) return;
    const url = response.url();
    pictures.set(url, response.body().then((body) => body.length, () => 0));
  });
  const url = `${site.origin}${site.mount}`;
  const answer = await tab.goto(url, { waitUntil: "load" });
  if (!answer || answer.status() !== 200) throw new Error(`${url} answered ${answer?.status() ?? "nothing"}`);
  const hero = await tab.evaluate(() => document.querySelector('img[fetchpriority="high"]')?.currentSrc ?? "");
  await readToTheEnd(tab);
  const found = [];
  for (const [address, bytes] of pictures) found.push({ url: address, bytes: await bytes, hero: address === hero });
  await context.close();
  return found.sort((a, b) => b.bytes - a.bytes);
};

const kb = (bytes) => `${(bytes / KB).toFixed(0)} KB`;
const sum = (pictures) => pictures.reduce((total, picture) => total + picture.bytes, 0);
const short = (address) => new URL(address).pathname;

const playwright = createRequire(import.meta.url)("playwright");
const browser = await playwright.chromium.launch();
const rows = [];
const failures = [];
// One line of the report: the bytes of `pictures`, with and without the hero.
const weigh = (pictures) => {
  const total = sum(pictures);
  const hero = sum(pictures.filter((p) => p.hero));
  return { total, hero, rest: total - hero, pictures: pictures.length };
};
const heaviest = (pictures) =>
  pictures
    .filter((p) => !p.hero)
    .slice(0, 3)
    .map((p) => `${short(p.url)} ${kb(p.bytes)}`)
    .join(", ");
const named = (view) => (view === DESKTOP ? `${view.width} px` : `${view.width} px and ${view.scale}x`);
for (const site of sites) {
  const phone = await measure(browser, site, PHONE);
  const phone1x = await measure(browser, site, PHONE_1X);
  const desktop = await measure(browser, site, DESKTOP);
  const row = { pack: site.pack, site: site.name, phone: weigh(phone), phone1x: weigh(phone1x), desktop: weigh(desktop) };
  rows.push(row);
  console.log(
    `${site.pack.padEnd(12)} 390 px 3x: ${kb(row.phone.rest).padStart(7)} without the hero (${kb(row.phone.total)} with it, ${row.phone.pictures} pictures)` +
      `   1x: ${kb(row.phone1x.rest).padStart(7)} (${kb(row.phone1x.total)})   1440 px: ${kb(row.desktop.total).padStart(7)} (${row.desktop.pictures} pictures)`,
  );
  const checks = [
    [PHONE, phone, row.phone.rest, PHONE_3X_OWN[site.pack] ?? PHONE_3X_BUDGET, " besides the hero"],
    [PHONE_1X, phone1x, row.phone1x.rest, PHONE_BUDGET, " besides the hero"],
    [DESKTOP, desktop, row.desktop.total, DESKTOP_OWN[site.pack] ?? DESKTOP_BUDGET, ""],
  ];
  for (const [view, pictures, bytes, budget, besides] of checks) {
    if (bytes > budget) {
      failures.push(`${site.pack}: at ${named(view)} its home page downloads ${kb(bytes)} of pictures${besides}, over the ${kb(budget)} budget. The heaviest: ${heaviest(pictures)}.`);
    }
  }
}
await browser.close();
if (options.report) writeFileSync(options.report, `${JSON.stringify(rows, null, 2)}\n`);

if (failures.length > 0) {
  console.log(`\n${failures.length} page-weight ${failures.length === 1 ? "budget was" : "budgets were"} broken:\n`);
  for (const failure of failures) console.log(`- ${failure}`);
  console.log(
    "\nHow to fix it: give each picture slot the width the pack's CSS really draws it at, at a phone, a tablet and a desktop width." +
      " Put it in the pack's picture_sizes.html and pass it as the slot's `sizes`, such as" +
      ' picture(post.image, slot_sizes("card")) or cards(posts, 3, sizes=slot_sizes("home", template.options.cards)).' +
      " A `sizes` wider than the slot makes the browser fetch a bigger file than it draws; one narrower makes the picture blurry." +
      " When the same post shows in two slots of one page, give the smaller slot the larger one's `sizes`, so the picture is fetched once." +
      ' Never raise a budget to make this pass. See docs/standards/performance.md, "Page weight".',
  );
  process.exit(1);
}
console.log(
  `\nEvery home page is inside the page-weight budgets without the hero: under ${kb(PHONE_3X_BUDGET)} at ${named(PHONE)} (Aperture ${kb(PHONE_3X_OWN.aperture)})` +
    ` and under ${kb(PHONE_BUDGET)} at ${named(PHONE_1X)}; and with it under ${kb(DESKTOP_BUDGET)} at ${named(DESKTOP)} (Aperture ${kb(DESKTOP_OWN.aperture)}).`,
);
