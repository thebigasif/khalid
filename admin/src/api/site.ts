// The settings routes of the Admin API, typed from the OpenAPI document.
// The answer wraps one object, `{ "settings": AdminSettings }`. The
// document types `[seo]`, `[privacy]` and `[code]` as open tables, so they
// keep the admin's own types for them, and the fields a server from before
// their screen leaves out stay optional.
import type { CodeEntry, PrivacyPart, PrivacyTable } from "../privacy/parts";
import type { Part, SeoTable } from "../seo/parts";
import { call, type Reply, sending } from "./call";
import type { components } from "./schema";
import type { TemplateSetting } from "./templates";

type Schemas = components["schemas"];

/** One section of the home page, as `[reader] home_sections` holds it. */
export type HomeSection = Schemas["HomeSectionSetting"];
/** A home section that shows one list, named by its slug. */
export type ListSectionKind = Extract<HomeSection, { slug: string }>["kind"];
/** A home section that names no list. */
export type PlainSectionKind = Exclude<HomeSection["kind"], ListSectionKind>;

/** One link in a menu, as `[[nav.primary]]` or `[[nav.footer]]` holds it. */
export type MenuLink = Schemas["MenuLinkSetting"];

/** The header and footer menus from `[nav]`. */
export type SiteNav = Schemas["NavSetting"];

/** Which menu: the header's main menu or the footer's. */
export type MenuName = keyof SiteNav;

type Settings = Schemas["AdminSettings"];

/** Fields a server from before their screen sends none of. */
type Later = "home_sections" | "nav" | "seo";

/** The site's core settings, as `site.toml` holds them. */
export type SiteSettings = Omit<Settings, Later | "template" | "mode" | "privacy" | "code"> &
  Partial<Pick<Settings, Later>> & {
    /** The template the blog uses and the options `site.toml` sets for it. */
    template: TemplateSetting;
    /** The `[seo]` table, one key per part such as `titles`. */
    seo?: SeoTable;
    /** How the site reaches readers. */
    mode?: "server" | "static" | "headless";
    /** The `[privacy]` table, one key per part such as `gpc`. */
    privacy?: PrivacyTable;
    /** The site's own code from `[code]`, one entry each, as written. */
    code?: CodeEntry[];
  };

/** A change: only the fields sent change. */
export type SiteChange = Omit<
  Schemas["SettingsChange"],
  "template" | "seo" | "privacy" | "code" | "nav"
> & {
  /** A new template; it replaces the name and every option. */
  template?: TemplateSetting | null;
  /** New parts of `[seo]`, each replacing its part whole; `null` removes one. */
  seo?: Record<string, Part | null>;
  /** New parts of `[privacy]`, each replacing its part whole; `null` removes one. */
  privacy?: Record<string, PrivacyPart | null>;
  /** Every entry of `[code]`, in place of the old ones. */
  code?: CodeEntry[];
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
  const body: Schemas["SettingsChanges"] = { settings: change };
  return call("/settings/", sending("PUT", body), settings);
}
