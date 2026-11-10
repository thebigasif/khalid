import type { HomeSection, ListSectionKind, PlainSectionKind } from "../api/site";

type Kind = HomeSection["kind"];

/** What each kind is called in the list and in the "Add a section" menu. */
const names: Record<Kind, string> = {
  latest: "Newest posts",
  featured: "Featured posts",
  newsletter: "Newsletter sign-up",
  category: "Category",
  collection: "Collection",
  channel: "Channel",
  series: "Series",
};

/** What each kind shows, under its name. */
const shows: Record<Kind, string> = {
  latest: "Your newest posts, with a link to the archive.",
  featured: "Posts marked featured, newest first.",
  newsletter: "The sign-up form, here instead of where the template puts it.",
  category: "The newest posts in this category.",
  collection: "The newest posts in this collection.",
  channel: "The newest posts in this channel.",
  series: "The parts of this series, in reading order.",
};

/** Where to find a list's slug, for the field that asks for it. */
const slugHints: Record<ListSectionKind, string> = {
  category:
    "The end of the category's address, such as on-page-seo for /blog/category/on-page-seo/.",
  collection: "The collection's folder name, such as guides.",
  channel: "The channel's name in routes.toml, such as featured.",
  series:
    "The end of the series page's address, such as link-building-101 for /blog/series/link-building-101/.",
};

/** The kinds the menu offers, in its order. */
export const SECTION_KINDS: readonly Kind[] = [
  "latest",
  "featured",
  "newsletter",
  "category",
  "collection",
  "channel",
  "series",
];

/** Whether `kind` shows one list, so it needs a slug. */
export function namesAList(kind: Kind): kind is ListSectionKind {
  return !(["latest", "featured", "newsletter"] as PlainSectionKind[]).includes(
    kind as PlainSectionKind,
  );
}

/** A section's name in the list, such as "Category: technical-seo". */
function label(section: HomeSection): string {
  return "slug" in section ? `${names[section.kind]}: ${section.slug}` : names[section.kind];
}

// The words for the home page's sections in Settings, beside en.ts, which
// is near its size limit.
export const homeSections = {
  title: "Home page",
  lede: "Choose what your home page shows, top first. Each section shows a few posts and a link to all of them. With no sections, the home page lists your newest posts.",
  none: "No sections yet, so the home page lists your newest posts.",
  names,
  shows,
  label,
  kindLabel: "Add a section",
  menuName: (kind: Kind) => (namesAList(kind) ? `A ${names[kind].toLowerCase()}` : names[kind]),
  slugLabel: (kind: ListSectionKind) => `${names[kind]} slug`,
  slugHint: (kind: ListSectionKind) => slugHints[kind],
  slugMissing: "Type the slug first. The section needs it to know which posts to show.",
  already: "That section is on the list already. Each section can show once.",
  add: "Add section",
  up: "Move up",
  down: "Move down",
  remove: "Remove",
  upNamed: (section: HomeSection) => `Move ${label(section)} up`,
  downNamed: (section: HomeSection) => `Move ${label(section)} down`,
  removeNamed: (section: HomeSection) => `Remove ${label(section)}`,
  save: "Save home page",
};
