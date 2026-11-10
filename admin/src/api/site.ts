// The settings routes of the Admin API. They are typed by hand, field for
// field as `AdminSettings` and `SettingsChange` in the G24 stack, because
// this document gains them only when that stack lands
// (docs/decisions/2026-10-31-admin-screens.md). The answer wraps one
// object, `{ "settings": AdminSettings }`, as G24's handlers write it.
import type { CodeEntry, PrivacyPart, PrivacyTable } from "../privacy/parts";
import type { Part, SeoTable } from "../seo/parts";
import { call, type Reply, sending } from "./call";
import type { TemplateSetting } from "./templates";

/** A home section that names no list. */
export type PlainSectionKind = "latest" | "featured" | "newsletter";
/** A home section that shows one list, named by its slug. */
export type ListSectionKind = "category" | "collection" | "channel" | "series";

/** One section of the home page, as `[reader] home_sections` holds it. */
export type HomeSection = { kind: PlainSectionKind } | { kind: ListSectionKind; slug: string };

/** One link in a menu, as `[[nav.primary]]` or `[[nav.footer]]` holds it. */
export type MenuLink = {
  label: string;
  /** A path on the blog, such as `/blog/about/`, or a full `https` address. */
  href: string;
  /** The one language whose pages show it; every page when left out. */
  language?: string;
};

/** Which menu: the header's main menu or the footer's. */
export type MenuName = "primary" | "footer";

/** The header and footer menus from `[nav]`. */
export type SiteNav = Record<MenuName, MenuLink[]>;

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
  /** The home page's sections, top first. A server from before them sends none. */
  home_sections?: HomeSection[];
  /** The header and footer menus. A server from before the Navigation screen sends none. */
  nav?: SiteNav;
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
  /** Every home section, top first, in place of the old ones. */
  home_sections?: HomeSection[];
  /** New menus, each replacing its menu whole; a menu left out stays. */
  nav?: Partial<SiteNav>;
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
