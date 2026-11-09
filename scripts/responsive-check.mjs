// Checks that every template, and the admin, works on every screen from a
// small phone to a 27 inch iMac. For each page and width it fails when:
//
//   - the page scrolls sideways, and names the elements that stick out;
//   - the site footer's left or right edge is more than 2 px from the
//     header's;
//   - on a phone, reading text is under 15 px or a navigation tap target is
//     under 40 by 40 px;
//   - on 1920 and 2560, the reading column is wider than about 80
//     characters, the content runs edge to edge or is off centre, or a
//     picture is drawn larger than its own pixels.
//
// It also saves a screenshot of the first screens of each page, for the
// things a script cannot see. See docs/templates.md, "Checking every
// screen size".
//
// Usage, from a folder where `require("playwright")` works:
//
//   node scripts/responsive-check.mjs --dir <pages folder> [--mount /khalid/]
//        [--templates classic,harbor] [--pages /,/seo-writing-guide/]
//        [--widths 320,390,1440,2560@2] [--out <folder>] [--screens 3]
//   node scripts/responsive-check.mjs --admin http://localhost:8080/blog/admin/
//        --sign-in-file <file holding the link khalid staff owner printed>
//
//   node scripts/responsive-check.mjs --dir docs/design/admin-mockups --mount /
//        --pages sign-in.html,dashboard.html --app
//
// `--dir` serves a folder scripts/pages.sh built, or with `--pages` any
// folder of pages; `--app` checks them as admin screens. By default it checks each
// template under templates/ with its home, /seo-writing-guide/, a category
// page and the German home. A width written as 2560@2 is checked at device
// scale 2, the way a 5120 pixel iMac screen shows it.
import { createReadStream, existsSync, mkdirSync, readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { createServer } from "node:http";
import { createRequire } from "node:module";
import { extname, join, resolve } from "node:path";

const PHONE_MAX = 414;
const LARGE_MIN = 1920;
const HEIGHTS = { 320: 568, 360: 740, 375: 667, 390: 844, 414: 896, 768: 1024, 1024: 768, 1280: 800, 1440: 900, 1920: 1080, 2560: 1440 };
const DEFAULT_WIDTHS = "320,360,375,390,414,768,1024,1280,1440,1920,2560";
const TEMPLATE_PAGES = [
  ["home", ""],
  ["guide", "seo-writing-guide/"],
  ["category", "category/seo-writing/"],
  ["de", "de/"],
];
const TYPES = { ".html": "text/html; charset=utf-8", ".css": "text/css", ".js": "text/javascript", ".svg": "image/svg+xml", ".png": "image/png", ".jpg": "image/jpeg", ".jpeg": "image/jpeg", ".webp": "image/webp", ".avif": "image/avif", ".woff2": "font/woff2", ".json": "application/json", ".xml": "application/xml", ".ico": "image/x-icon" };

function options(argv) {
  const out = {};
  for (let i = 0; i < argv.length; i += 1) {
    const key = argv[i].replace(/^--/, "");
    out[key] = argv[i + 1] && !argv[i + 1].startsWith("--") ? argv[(i += 1)] : "yes";
  }
  return out;
}

function playwright() {
  for (const from of [join(process.cwd(), "x.js"), import.meta.url]) {
    try {
      return createRequire(from)("playwright");
    } catch {
      // Try the next place.
    }
  }
  console.error("Playwright was not found. Run this script from a folder where Playwright is installed, such as one where you ran npm install playwright.");
  process.exit(2);
}

// Serves `dir` at `mount` on a free local port, the way GitHub Pages does.
function serve(dir, mount) {
  const server = createServer((req, res) => {
    const path = decodeURIComponent(new URL(req.url, "http://x").pathname);
    let file = path.startsWith(mount) ? join(dir, path.slice(mount.length)) : "";
    if (file && existsSync(file) && statSync(file).isDirectory()) file = join(file, "index.html");
    if (!file || !existsSync(file)) {
      res.writeHead(404, { "content-type": TYPES[".html"] });
      res.end("Not found");
      return;
    }
    res.writeHead(200, { "content-type": TYPES[extname(file)] ?? "application/octet-stream" });
    createReadStream(file).pipe(res);
  });
  return new Promise((done) => server.listen(0, "127.0.0.1", () => done(server)));
}

function widths(list) {
  return list.split(",").map((spec) => {
    const [w, scale] = spec.split("@").map(Number);
    return { width: w, scale: scale || 1, height: HEIGHTS[w] ?? Math.round(w * 0.5625), label: scale > 1 ? `${w * scale}` : `${w}` };
  });
}

// Runs inside the page. Returns a list of problems, each one line.
function measure({ phone, large, admin }) {
  const problems = [];
  const vw = document.documentElement.clientWidth;
  const name = (el) => {
    const cls = typeof el.className === "string" ? el.className.trim().split(/\s+/).filter(Boolean).slice(0, 2) : [];
    return `${el.tagName.toLowerCase()}${el.id ? `#${el.id}` : ""}${cls.map((c) => `.${c}`).join("")}`;
  };
  const shown = (el) => {
    const style = getComputedStyle(el);
    const box = el.getBoundingClientRect();
    return style.visibility !== "hidden" && style.display !== "none" && box.width > 1 && box.height > 1;
  };
  const clipped = (el) => {
    for (let up = el.parentElement; up && up !== document.body; up = up.parentElement) {
      const style = getComputedStyle(up);
      if (style.position === "fixed") return true;
      if (style.overflowX !== "visible") return true;
    }
    return getComputedStyle(el).position === "fixed";
  };

  // Sideways scrolling, and the outermost elements that cause it.
  const scroll = document.documentElement.scrollWidth;
  if (scroll > vw + 1) {
    const out = [...document.body.querySelectorAll("*")].filter((el) => {
      const box = el.getBoundingClientRect();
      return box.width > 0 && box.right > vw + 1 && !clipped(el);
    });
    const roots = out.filter((el) => !out.includes(el.parentElement)).slice(0, 6);
    const named = roots.map((el) => `${name(el)} to ${Math.round(el.getBoundingClientRect().right)} px`);
    problems.push(`scrolls sideways: ${scroll} px wide in ${vw}. Sticks out: ${named.join(", ") || "nothing named"}`);
  }

  if (phone) {
    // Notes, captions and bylines may be smaller than the reading text.
    const skip = "nav, footer, figcaption, aside, small, time, label, .lock,[class*='meta'], [class*='caption'], [class*='byline'], [class*='kicker']";
    const reading = [...document.querySelectorAll("main p, main li")].filter(
      (el) => shown(el) && el.textContent.trim().length >= 40 && !el.closest(skip) && !(el.tagName === "LI" && el.querySelector("p")),
    );
    const small = reading.filter((el) => Number.parseFloat(getComputedStyle(el).fontSize) < 15);
    if (small.length) {
      const el = small[0];
      problems.push(`reading text under 15 px: ${small.length} blocks, such as ${name(el)} at ${getComputedStyle(el).fontSize}`);
    }
    // Links in a nav, and in the site's own header (not a post's header).
    const targets = [...document.querySelectorAll(":is(nav, header) :is(a, button, summary)")].filter((el) => {
      const box = el.getBoundingClientRect();
      const header = el.closest("header");
      const inNav = el.closest("nav") || (header && !header.closest("main, article"));
      return inNav && shown(el) && box.bottom + window.scrollY > 0 && box.right > 0;
    });
    // A target may reach past its box with an absolute ::before or ::after.
    const reach = (el) => {
      const box = el.getBoundingClientRect();
      let { width, height } = box;
      for (const part of ["::before", "::after"]) {
        const style = getComputedStyle(el, part);
        if (style.content === "none" || style.position !== "absolute") continue;
        const out = (side) => Math.max(0, -(Number.parseFloat(style[side]) || 0));
        width = Math.max(width, box.width + out("left") + out("right"));
        height = Math.max(height, box.height + out("top") + out("bottom"));
      }
      return { width, height };
    };
    const tiny = targets.filter((el) => {
      const box = reach(el);
      return box.width < 39.5 || box.height < 39.5;
    });
    if (tiny.length) {
      const list = tiny.slice(0, 4).map((el) => {
        const box = reach(el);
        return `${name(el)} "${el.textContent.trim().slice(0, 20)}" ${Math.round(box.width)}x${Math.round(box.height)}`;
      });
      problems.push(`navigation tap targets under 40x40: ${tiny.length}, such as ${list.join("; ")}`);
    }
  }

  if (large) {
    // Characters per line of the widest prose paragraph.
    const canvas = document.createElement("canvas").getContext("2d");
    let widest = null;
    for (const p of document.querySelectorAll("main p")) {
      const text = p.textContent.trim();
      if (text.length < 160 || !shown(p) || p.closest("nav, footer")) continue;
      const style = getComputedStyle(p);
      canvas.font = `${style.fontStyle} ${style.fontWeight} ${style.fontSize} ${style.fontFamily}`;
      const perChar = canvas.measureText(text).width / text.length;
      const chars = Math.round(p.clientWidth / perChar);
      if (!widest || chars > widest.chars) widest = { chars, el: p };
    }
    if (widest && widest.chars > 85) {
      const words = widest.el.textContent.trim().slice(0, 30);
      problems.push(`reading column about ${widest.chars} characters wide at ${name(widest.el)} "${words}", more than about 80`);
    }

    // Where the content sits across the screen. A row that scrolls sideways
    // inside the page may hold items past the edge; those are not counted.
    const items = [...document.querySelectorAll("main :is(p, h1, h2, h3, li, img, figure, table, pre)")].filter((el) => {
      const box = el.getBoundingClientRect();
      return shown(el) && !el.closest("nav, [aria-hidden='true']") && box.left >= 0 && box.right <= vw + 1;
    });
    if (items.length) {
      const left = Math.min(...items.map((el) => el.getBoundingClientRect().left));
      const right = Math.max(...items.map((el) => el.getBoundingClientRect().right));
      const span = right - left;
      if (admin) {
        if (span > 1680) problems.push(`admin content ${Math.round(span)} px wide; keep it in a max width`);
      } else {
        if (span > vw * 0.9) problems.push(`content runs edge to edge: ${Math.round(left)} to ${Math.round(right)} px of ${vw}`);
        const gap = Math.abs(left - (vw - right));
        if (gap > vw * 0.12) problems.push(`content off centre: ${Math.round(left)} px on the left, ${Math.round(vw - right)} px on the right`);
      }
    }
  }

  // The header and the footer share one column, so their edges line up at
  // every width.
  const header = document.querySelector(".site-header");
  const footer = document.querySelector(".site-footer");
  if (!admin && header && footer && shown(header) && shown(footer)) {
    const top = header.getBoundingClientRect();
    const bottom = footer.getBoundingClientRect();
    if (Math.abs(top.left - bottom.left) > 2 || Math.abs(top.right - bottom.right) > 2) {
      problems.push(`footer ${Math.round(bottom.left)} to ${Math.round(bottom.right)} px does not line up with the header ${Math.round(top.left)} to ${Math.round(top.right)} px`);
    }
  }
  return problems;
}

// Pictures drawn larger than their own pixels, read from each file itself
// so a srcset's density does not hide it.
async function blownUp(page) {
  return page.evaluate(async () => {
    const found = [];
    for (const img of document.querySelectorAll("main img, header img")) {
      const src = img.currentSrc || img.src;
      const box = img.getBoundingClientRect();
      if (!src || /\.svg(\?|$)/.test(src) || src.startsWith("data:image/svg") || box.width < 2) continue;
      const pixels = await new Promise((done) => {
        const probe = new Image();
        probe.onload = () => done(probe.naturalWidth);
        probe.onerror = () => done(0);
        probe.src = src;
      });
      if (pixels > 0 && box.width > pixels * 1.1) {
        found.push(`picture drawn ${Math.round(box.width)} px wide from ${pixels} px: ${src.split("/").pop()}`);
      }
    }
    return found;
  });
}

async function settle(page) {
  await page.evaluate(() => {
    for (const img of document.querySelectorAll("img[loading='lazy']")) img.loading = "eager";
  });
  await page.waitForLoadState("networkidle", { timeout: 15000 }).catch(() => {});
  await page.evaluate(() => document.fonts.ready);
}

async function check(context, job, size, out, screens) {
  const page = await context.newPage();
  try {
    const response = await page.goto(job.url, { waitUntil: "load", timeout: 60000 });
    if (job.find) {
      await page.waitForLoadState("networkidle", { timeout: 15000 }).catch(() => {});
      const href = await page.evaluate((sel) => document.querySelector(sel)?.href ?? "", job.find);
      if (!href) return [`no link matches ${job.find}`];
      await page.goto(href, { waitUntil: "load", timeout: 60000 });
    }
    if (response && response.status() >= 400) return [`answered ${response.status()}`];
    await settle(page);
    const phone = size.width <= PHONE_MAX;
    const large = size.width >= LARGE_MIN;
    const problems = await page.evaluate(measure, { phone, large, admin: Boolean(job.admin) });
    if (large) problems.push(...(await blownUp(page)));
    const tall = await page.evaluate(() => document.documentElement.scrollHeight);
    const height = Math.min(tall, size.height * screens);
    await page.screenshot({ path: join(out, `${job.label}-${size.label}.png`), clip: { x: 0, y: 0, width: size.width, height }, fullPage: true });
    return problems;
  } catch (error) {
    return [`could not be checked: ${error.message.split("\n")[0]}`];
  } finally {
    await page.close();
  }
}

function templateJobs(dir, base, opts) {
  const root = join(dir, "templates");
  // A plain folder of pages, such as docs/design/admin-mockups, with --pages.
  if (!existsSync(root) && opts.pages) {
    return opts.pages.split(",").map((p) => ({ label: p.replace(/\.html$/, "").replace(/\W+/g, "-"), url: `${base}${p}`, admin: Boolean(opts.app) }));
  }
  const names = opts.templates
    ? opts.templates.split(",")
    : readdirSync(root).filter((n) => statSync(join(root, n)).isDirectory()).sort();
  const pages = opts.pages ? opts.pages.split(",").map((p) => [p.replace(/\W+/g, "-").replace(/^-|-$/g, "") || "home", p.replace(/^\//, "")]) : TEMPLATE_PAGES;
  return names.flatMap((n) => pages.map(([label, path]) => ({ label: `${n}-${label}`, url: `${base}templates/${n}/${path}` })));
}

function adminJobs(admin) {
  return [
    { label: "admin-dashboard", url: admin, admin: true },
    { label: "admin-posts", url: `${admin}posts`, admin: true },
    { label: "admin-editor", url: `${admin}posts`, find: "main a[href*='/posts/']", admin: true },
    { label: "admin-media", url: `${admin}media`, admin: true },
    { label: "admin-look", url: `${admin}look`, admin: true },
  ];
}

async function main() {
  const opts = options(process.argv.slice(2));
  if (!opts.dir && !opts.admin) {
    console.error("Name what to check: --dir <folder scripts/pages.sh built> or --admin <admin address>.");
    process.exit(2);
  }
  const out = resolve(opts.out ?? "responsive-shots");
  mkdirSync(out, { recursive: true });
  const sizes = widths(opts.widths ?? DEFAULT_WIDTHS);
  const screens = Number(opts.screens ?? 3);
  const { chromium } = playwright();
  const browser = await chromium.launch();
  let server = null;
  let jobs = [];
  let signedIn;
  if (opts.dir) {
    const mount = opts.mount ?? "/khalid/";
    server = await serve(resolve(opts.dir), mount);
    jobs = templateJobs(resolve(opts.dir), `http://127.0.0.1:${server.address().port}${mount}`, opts);
  } else {
    const admin = opts.admin.endsWith("/") ? opts.admin : `${opts.admin}/`;
    jobs.push({ label: "admin-sign-in", url: `${admin}sign-in`, admin: true, fresh: true });
    if (opts["sign-in-file"]) {
      const context = await browser.newContext();
      const page = await context.newPage();
      await page.goto(readFileSync(opts["sign-in-file"], "utf8").trim(), { waitUntil: "networkidle" });
      signedIn = await context.storageState();
      await context.close();
      jobs.push(...adminJobs(admin));
    } else {
      console.error("No --sign-in-file, so only the sign-in page is checked.");
    }
  }

  const failures = [];
  const report = [];
  const workers = Number(opts.workers ?? 3);
  for (const size of sizes) {
    const contexts = {
      fresh: await browser.newContext({ viewport: { width: size.width, height: size.height }, deviceScaleFactor: size.scale }),
      signed: await browser.newContext({ viewport: { width: size.width, height: size.height }, deviceScaleFactor: size.scale, storageState: signedIn }),
    };
    const queue = [...jobs];
    await Promise.all(
      Array.from({ length: workers }, async () => {
        for (let job = queue.shift(); job; job = queue.shift()) {
          const problems = await check(job.fresh ? contexts.fresh : contexts.signed, job, size, out, screens);
          report.push({ page: job.label, width: size.label, problems });
          const line = `${problems.length ? "FAIL" : "ok  "} ${size.label.padStart(4)} ${job.label}`;
          console.log(problems.length ? `${line}\n       ${problems.join("\n       ")}` : line);
          if (problems.length) failures.push(line);
        }
      }),
    );
    await contexts.fresh.close();
    await contexts.signed.close();
  }
  await browser.close();
  server?.close();
  writeFileSync(join(out, "report.json"), `${JSON.stringify(report, null, 2)}\n`);
  console.log(`\nChecked ${jobs.length} pages at ${sizes.length} widths. Screenshots and report.json are in ${out}.`);
  if (failures.length) {
    console.log(`${failures.length} page and width pairs failed. Fix them and run this again.`);
    process.exit(1);
  }
  console.log("Every page passed at every width.");
}

await main();
