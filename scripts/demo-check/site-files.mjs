// Serves built sites to a browser from their folders, for layout.mjs and
// pictures.mjs. Nothing leaves the computer.
import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { extname, join } from "node:path";

const TYPES = { ".html": "text/html; charset=utf-8", ".css": "text/css", ".js": "text/javascript", ".mjs": "text/javascript", ".svg": "image/svg+xml", ".png": "image/png", ".jpg": "image/jpeg", ".jpeg": "image/jpeg", ".webp": "image/webp", ".avif": "image/avif", ".gif": "image/gif", ".woff2": "font/woff2", ".json": "application/json", ".xml": "application/xml", ".txt": "text/plain", ".ico": "image/x-icon", ".mp4": "video/mp4", ".webm": "video/webm", ".vtt": "text/vtt" };

// Every built site in `folder`: a folder holding a site.toml and the site
// built into public/, with its name, address, mount and, for a demo
// publication, the pages its check-pages.txt names.
export function builtSites(folder) {
  return readdirSync(folder)
    .filter((name) => existsSync(join(folder, name, "public")))
    .sort()
    .map((name) => {
      const toml = readFileSync(join(folder, name, "site.toml"), "utf8");
      const field = (key) => toml.match(new RegExp(`^${key} = "(.*)"`, "m"))?.[1] ?? "";
      return { name, origin: new URL(field("address")).origin, mount: field("mount") || "/", files: join(folder, name, "public"), checkPages: checkPages(join(folder, name)) };
    });
}

// The pages a demo publication's check-pages.txt names, which
// scripts/demo-publication.sh copies beside the site's site.toml, as
// [kind, path] pairs with the path after the mount, the home page first.
// Each line is a kind, such as post, table, category, tag or author, then
// a path such as /my-post/; # starts a note. Null for a site without the
// file, which keeps the showcase's own pages.
export function checkPages(site) {
  const file = join(site, "check-pages.txt");
  if (!existsSync(file)) return null;
  const pages = readFileSync(file, "utf8")
    .split("\n")
    .map((line) => line.replace(/#.*/, "").trim())
    .filter(Boolean)
    .map((line) => {
      const [kind, path = "/"] = line.split(/\s+/);
      return [kind, path.replace(/^\/+/, "")];
    });
  return pages.some(([kind]) => kind === "home") ? pages : [["home", ""], ...pages];
}

// A Playwright route handler that answers a request for one of `sites`'
// own files from its folder. A request to any other address is refused.
export function answerFrom(sites) {
  const byOrigin = new Map(sites.map((site) => [site.origin, site]));
  return async (route) => {
    const url = new URL(route.request().url());
    const site = byOrigin.get(url.origin);
    if (!site) return route.abort("blockedbyclient");
    const path = decodeURIComponent(url.pathname);
    let file = path.startsWith(site.mount) ? join(site.files, path.slice(site.mount.length)) : "";
    if (file && existsSync(file) && statSync(file).isDirectory()) file = join(file, "index.html");
    if (!file || !existsSync(file)) return route.fulfill({ status: 404, contentType: TYPES[".html"], body: "Not found" });
    return route.fulfill({ status: 200, contentType: TYPES[extname(file)] ?? "application/octet-stream", body: readFileSync(file) });
  };
}
