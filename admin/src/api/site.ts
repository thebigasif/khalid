// The settings routes of the Admin API. They are typed by hand, field for
// field as `AdminSettings` and `SettingsChange` in the G24 stack, because
// this document gains them only when that stack lands
// (docs/decisions/2026-10-31-admin-screens.md). The answer wraps one
// object, `{ "settings": AdminSettings }`, as G24's handlers write it.
import type { CodeEntry, PrivacyPart, PrivacyTable } from "../privacy/parts";
import type { Part, SeoTable } from "../seo/parts";
import { call, type Reply, sending } from "./call";
import type { TemplateSetting } from "./templates";

/** The site's core settings, as `site.toml` holds them. */
export type SiteSettings = {
  name: string;
  /** The main site's address, such as `https://www.example.com`. */
  address: string;
  /** The path the blog lives under, such as `/blog/`. */
  mount: string;
  default_language: string;
  languages: string[];
  /** The template the blog uses and the options `site.toml` sets for it. */
  template: TemplateSetting;
  /**
   * The `[seo]` table as `site.toml` writes it, one key per part such as
   * `titles`. A server from before the SEO settings screen sends none.
   */
  seo?: SeoTable;
  /** How the site reaches readers. A server from before the Privacy screen sends none. */
  mode?: "server" | "static" | "headless";
  /** The `[privacy]` table as `site.toml` writes it, one key per part such as `gpc`. */
  privacy?: PrivacyTable;
  /** The site's own code from `[code]`, one entry each, as written. */
  code?: CodeEntry[];
  /** When `site.toml` last changed; a change sends it back. */
  updated_at: string;
  /** Whether the running server needs a restart to use what `site.toml` says. */
  restart_needed: boolean;
};

/** A change: only the fields sent change. */
export type SiteChange = {
  updated_at: string;
  name?: string | null;
  address?: string | null;
  mount?: string | null;
  default_language?: string | null;
  languages?: string[] | null;
  /** A new template; it replaces the name and every option. */
  template?: TemplateSetting | null;
  /** New parts of `[seo]`, each replacing its part whole; `null` removes one. */
  seo?: Record<string, Part | null>;
  /** New parts of `[privacy]`, each replacing its part whole; `null` removes one. */
  privacy?: Record<string, PrivacyPart | null>;
  /** Every entry of `[code]`, in place of the old ones. */
  code?: CodeEntry[];
};

async function settings(response: Response): Promise<SiteSettings> {
  const body: { settings: SiteSettings } = await response.json();
  return body.settings;
}

/** The site's core settings. */
export function readSite(): Promise<Reply<SiteSettings>> {
  return call("/settings/", {}, settings);
}

/** Saves a change made on the copy that `change.updated_at` names. */
export function changeSite(change: SiteChange): Promise<Reply<SiteSettings>> {
  return call("/settings/", sending("PUT", { settings: change }), settings);
}
