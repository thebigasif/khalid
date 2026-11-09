// Proves the network capture of network.mjs on two small sites it writes
// itself, in Chromium. The site "held" keeps a player frame inside an
// inert template behind its card and an analytics script as text, as
// Khalid writes them for a reader who has not answered. The site "leaky"
// draws the same frame live, far down a page so it loads only when
// reached, and runs the same script. The capture must find the frame with
// scripts on and off, the script with scripts on, and nothing on "held".
//
// Usage: node network-selftest.mjs, from anywhere. Exits 1 when the
// capture misses a request or finds one that is not there.
import { spawnSync } from "node:child_process";
import { mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const work = join(here, "..", "..", "target", "network-selftest");
const PLAYER = "https://www.youtube-nocookie.com/embed/dQw4w9WgXcQ";
const SCRIPT = "https://stats.example.net/a.js";

const page = (body) => `<!doctype html><html lang="en"><head><meta charset="utf-8"><title>Fixture</title></head>
<body><main><h1>A post</h1>${body}</main></body></html>`;
const frame = `<iframe src="${PLAYER}" title="A video" width="560" height="315" loading="lazy"></iframe>`;
const card = (live) => `<div style="height:3000px">Words above the video.</div>
<figure class="embed embed-wide"><div class="embed-card"><p>This comes from YouTube.</p></div>${live ? frame : `<template>${frame}</template>`}</figure>`;
const script = (live) => `<script ${live ? "" : 'type="text/plain" data-khalid-consent="analytics" '}src="${SCRIPT}"></script>`;

const site = (name, live) => {
  const files = join(work, "sites", name, "public");
  mkdirSync(join(files, "video"), { recursive: true });
  writeFileSync(join(work, "sites", name, "site.toml"), `address = "http://${name}.localhost"\nmount = "/"\n`);
  writeFileSync(join(files, "index.html"), page(`<p>Hello.</p>${script(live)}`));
  writeFileSync(join(files, "video", "index.html"), page(card(live)));
};

rmSync(work, { recursive: true, force: true });
site("held", false);
site("leaky", true);
spawnSync(process.execPath, [join(here, "network.mjs"), "--sites", join(work, "sites")], { stdio: "inherit" });
const found = JSON.parse(readFileSync(join(work, "network.json"), "utf8")).map((f) => [f.site, f.page, f.setting, f.request].join(" "));
const wanted = [`leaky / scripts on ${SCRIPT}`, `leaky /video/ scripts on ${PLAYER}`, `leaky /video/ scripts off ${PLAYER}`];
let wrong = 0;
for (const f of wanted.filter((w) => !found.includes(w))) {
  console.log(`MISSED: ${f}`);
  wrong += 1;
}
for (const f of found.filter((w) => !wanted.includes(w))) {
  console.log(`NOT A LEAK: ${f}`);
  wrong += 1;
}
if (wrong > 0) {
  console.log(`The network capture is wrong ${wrong} times on its own fixtures. Fix network.mjs, then run this again.`);
  process.exit(1);
}
console.log("The network capture finds the live frame and the live script, and nothing on the site that holds them.");
