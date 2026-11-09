// A blog for the Privacy screen's tests: the demo's settings with one
// untagged script and no [privacy] table, as a fresh site has.
import type { SiteChange, SiteSettings } from "../src/api/site";
import { type Answers, adminBlog } from "./blog";
import { json } from "./render";

/** The demo site's settings, as the settings API sends them. */
export const PRIVACY_DEMO: SiteSettings = {
  name: "Field Notes on SEO",
  address: "https://www.example.com",
  mount: "/blog/",
  default_language: "en",
  languages: ["en", "de"],
  mode: "server",
  template: { name: "classic", options: {} },
  seo: {},
  privacy: {},
  code: [{ head: '<script src="https://stats.example.net/a.js"></script>' }],
  updated_at: "2026-11-08T09:00:00Z",
  restart_needed: false,
};

/** A blog with `site`'s settings that records each change, then needs a restart; `more` answers the rest. */
export function privacyBlog(site: SiteSettings = PRIVACY_DEMO, more: Answers = {}): SiteChange[] {
  const changes: SiteChange[] = [];
  let now = site;
  adminBlog({
    ...more,
    "GET /settings/": () => json(200, { settings: now }),
    "PUT /settings/": async (request) => {
      const body: { settings: SiteChange } = await request.json();
      changes.push(body.settings);
      const privacy: Record<string, unknown> = { ...now.privacy };
      for (const [name, part] of Object.entries(body.settings.privacy ?? {})) {
        if (part === null) {
          delete privacy[name];
        } else {
          privacy[name] = part;
        }
      }
      const code = body.settings.code ?? now.code;
      now = { ...now, privacy, code, updated_at: "2026-11-08T09:05:00Z", restart_needed: true };
      return json(200, { settings: now });
    },
  });
  return changes;
}
