import type { AdminCategory } from "../api/terms";

/**
 * The languages a category is named in: the site's, when its settings
 * could be read; else every language a category already has; else English.
 */
export function nameLanguages(site: string[] | undefined, categories: AdminCategory[]): string[] {
  if (site !== undefined && site.length > 0) {
    return site;
  }
  const used = [...new Set(categories.flatMap((category) => Object.keys(category.names)))];
  return used.length > 0 ? used.sort() : ["en"];
}

/** The name to show first: the one in `language`, else the first it has, else its slug. */
export function categoryName(category: AdminCategory, language: string): string {
  return category.names[language] ?? Object.values(category.names)[0] ?? category.slug;
}

/** The names the person filled in, trimmed, leaving out the empty ones. */
export function filledNames(names: Record<string, string>): Record<string, string> {
  const filled = Object.entries(names)
    .map(([code, name]) => [code, name.trim()] as const)
    .filter(([, name]) => name !== "");
  return Object.fromEntries(filled);
}
