import { en } from "../strings/en";
import { searchConsoleWords } from "../strings/searchConsoleWords";
import { shellWords } from "../strings/shellWords";
import type { IconName } from "./Icon";

/** The address of every screen the side menu opens. */
export type NavPath =
  | "/"
  | "/posts"
  | "/media"
  | "/comments"
  | "/trash"
  | "/analytics"
  | "/search-console"
  | "/serpdad"
  | "/look"
  | "/authors"
  | "/redirects"
  | "/schema-templates"
  | "/settings"
  | "/team"
  | "/keys"
  | "/webhooks"
  | "/passkeys"
  | "/audit"
  | "/account";

/** One screen in the side menu. */
export type NavItem = { to: NavPath; label: string; icon: IconName };

/** A group of screens under one quiet heading. */
export type NavGroup = { label: string; items: NavItem[] };

/** The side menu: writing first, then growing the blog, the site, and the team's settings. */
export const NAV: NavGroup[] = [
  {
    label: shellWords.groups.write,
    items: [
      { to: "/", label: en.nav.dashboard, icon: "home" },
      { to: "/posts", label: en.nav.posts, icon: "posts" },
      { to: "/media", label: en.nav.media, icon: "media" },
      { to: "/comments", label: en.nav.comments, icon: "comments" },
      { to: "/trash", label: en.nav.trash, icon: "trash" },
    ],
  },
  {
    label: shellWords.groups.grow,
    items: [
      { to: "/analytics", label: en.nav.analytics, icon: "analytics" },
      { to: "/search-console", label: searchConsoleWords.title, icon: "search" },
      { to: "/serpdad", label: en.nav.serpdad, icon: "link" },
    ],
  },
  {
    label: shellWords.groups.site,
    items: [
      { to: "/look", label: en.nav.look, icon: "look" },
      { to: "/authors", label: en.nav.authors, icon: "person" },
      { to: "/redirects", label: en.nav.redirects, icon: "redirect" },
      { to: "/schema-templates", label: en.nav.templates, icon: "braces" },
      { to: "/settings", label: en.nav.settings, icon: "sliders" },
    ],
  },
  {
    label: shellWords.groups.settings,
    items: [
      { to: "/team", label: en.nav.team, icon: "team" },
      { to: "/keys", label: en.nav.keys, icon: "key" },
      { to: "/webhooks", label: en.nav.webhooks, icon: "bolt" },
      { to: "/passkeys", label: en.nav.passkeys, icon: "shield" },
      { to: "/audit", label: en.nav.audit, icon: "list" },
      { to: "/account", label: en.account.nav, icon: "account" },
    ],
  },
];

/** Whether the screen at `path` belongs to `item`: its own address or one under it. */
function opens(item: NavItem, path: string): boolean {
  if (item.to === "/") {
    return path === "/";
  }
  return path === item.to || path.startsWith(`${item.to}/`);
}

/**
 * The group and screen the top bar names for `path`, the address inside
 * the admin; `null` for an address the menu does not open. Starting a
 * post belongs to Posts.
 */
export function whereIs(path: string): { group: NavGroup; item: NavItem } | null {
  const inside = path === "/new" ? "/posts" : path;
  for (const group of NAV) {
    const item = group.items.find((each) => opens(each, inside));
    if (item !== undefined) {
      return { group, item };
    }
  }
  return null;
}
