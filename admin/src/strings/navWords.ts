import type { MenuName } from "../api/site";
import { en } from "./en";
import { languageName } from "./languages";

/** The words of each menu's card. */
const menus: Record<MenuName, { title: string; lede: string; none: string }> = {
  primary: {
    title: "Header menu",
    lede: "The main links at the top of every page, left to right. On a phone they fold into a Menu button.",
    none: "No links yet, so the header shows only your blog's name.",
  },
  footer: {
    title: "Footer menu",
    lede: "Links at the bottom of every page, such as About or your legal pages. Columns of footer links under a heading are set in site.toml, and saving here keeps them.",
    none: "No links yet, so the footer shows none.",
  },
};

// The words for the Navigation screen, beside en.ts, which is near its
// size limit.
export const navWords = {
  nav: "Navigation",
  title: "Navigation",
  lede: "The links in your blog's header and footer, on every page. Readers use them to find their way around.",
  loading: "Reading your menus",
  saved: en.site.saved,
  live: en.site.live,
  menus,
  label: { label: "Link text", hint: "The words readers see, such as Guides." },
  href: {
    label: "Address",
    hint: "A page on your blog, such as /blog/about/, or a full address, such as https://www.example.com/.",
  },
  language: {
    label: "Show on",
    every: "Pages in every language",
    only: (code: string) => `${languageName(code)} pages only`,
  },
  save: "Save menus",
  add: "Add link",
  addTo: (menu: MenuName) => `Add link to the ${menus[menu].title.toLowerCase()}`,
  up: "Move up",
  down: "Move down",
  remove: "Remove",
  upNamed: (label: string) => `Move ${label} up`,
  downNamed: (label: string) => `Move ${label} down`,
  removeNamed: (label: string) => `Remove ${label}`,
  labelMissing: "Type the link text first. Readers need words to click.",
  hrefWrong: "Start the address with / for a page on your blog, or with https:// for another site.",
};
