// A blog for the SEO settings screen's tests: the demo's settings, with a
// separator and one post template set in [seo.titles].
import type { SiteChange, SiteSettings } from "../src/api/site";
import { adminBlog } from "./blog";
import { json } from "./render";

/** The demo site's settings, as the settings API sends them. */
export const SEO_DEMO: SiteSettings = {
  name: "Field Notes on SEO",
  address: "https://www.example.com",
  mount: "/blog/",
  default_language: "en",
  languages: ["en", "de"],
  template: { name: "classic", options: {} },
  seo: { titles: { separator: "·", post: { title: "{title} {sep} {site}" } } },
  updated_at: "2026-11-02T09:00:00Z",
  restart_needed: false,
};

/** A blog with `site`'s settings that records each change, in use with no restart. */
export function seoBlog(site: SiteSettings = SEO_DEMO): SiteChange[] {
  const changes: SiteChange[] = [];
  let now = site;
  adminBlog({
    "GET /settings/": () => json(200, { settings: now }),
    "PUT /settings/": async (request) => {
      const body: { settings: SiteChange } = await request.json();
      changes.push(body.settings);
      const seo = { ...now.seo, ...body.settings.seo };
      now = { ...now, seo, updated_at: "2026-11-02T09:05:00Z" };
      return json(200, { settings: now });
    },
  });
  return changes;
}
