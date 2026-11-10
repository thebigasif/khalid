import type { MenuLink } from "../api/site";
import { navWords as words } from "../strings/navWords";

/** `list` with the item at `from` moved to `to`. */
export function swapped<T>(list: readonly T[], from: number, to: number): T[] {
  const next = [...list];
  const [moved] = next.splice(from, 1);
  if (moved !== undefined) {
    next.splice(to, 0, moved);
  }
  return next;
}

/**
 * Whether `href` is an address `site.toml` takes: a path on the blog that
 * starts with one `/` and holds no spaces, or a full `http` or `https` one.
 */
export function takesHref(href: string): boolean {
  if (href.startsWith("/")) {
    return !href.startsWith("//") && !/[\s\\]/.test(href);
  }
  return /^https?:\/\/[^\s/]+/.test(href);
}

/** The link the "Add link" fields describe, or what to fix first. */
export function madeLink(
  label: string,
  href: string,
  language: string,
): { link: MenuLink } | { problem: string } {
  const text = label.trim();
  const address = href.trim();
  if (text === "") {
    return { problem: words.labelMissing };
  }
  if (!takesHref(address)) {
    return { problem: words.hrefWrong };
  }
  const link: MenuLink = { label: text, href: address };
  return { link: language === "" ? link : { ...link, language } };
}
