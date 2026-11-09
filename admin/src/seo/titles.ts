// Title and description templates, from `[seo.titles]`: the page types
// they apply to, the variables they may hold, and the search result one
// makes, for the live preview on the SEO settings screen.

/** Each kind of page with its own templates, in the order the screen lists them. */
export const PAGE_TYPES = [
  "post",
  "page",
  "home",
  "category",
  "tag",
  "author",
  "collection",
  "date",
  "search",
  "glossary",
  "glossary_term",
  "not_found",
] as const;

/** A kind of page. */
export type PageType = (typeof PAGE_TYPES)[number];

/** The variables a template may hold, in the order the picker shows them. */
export const VARIABLES = [
  "title",
  "site",
  "sep",
  "tagline",
  "excerpt",
  "category",
  "author",
] as const;

/** A template variable. */
export type Variable = (typeof VARIABLES)[number];

/** The separators the screen offers; any other text may be typed too. */
export const SEPARATORS = ["|", "-", "·", "•", "»", "›", "/", "~"] as const;

/** What `{sep}` stands for when the site sets none. */
export const DEFAULT_SEPARATOR = "|";

/** The template a page type uses when the site sets none. */
export function defaultTemplate(type: PageType, field: "title" | "description"): string {
  if (type === "home") {
    return field === "title" ? "{site} {sep} {tagline}" : "{tagline}";
  }
  return field === "title" ? "{title} {sep} {site}" : "{excerpt}";
}

/** The values a template is filled with for the preview. */
export type Sample = Record<Variable, string>;

/**
 * `template` filled with `sample`, with spaces squeezed and separators
 * trimmed from the ends, as the blog writes a title whose variable is empty.
 */
export function filled(template: string, sample: Sample, separator: string): string {
  const text = template.replace(/\{([a-z_]+)\}/g, (whole, name: string) => {
    if (name === "sep") {
      return separator;
    }
    const known = VARIABLES.find((variable) => variable === name);
    return known === undefined ? whole : sample[known];
  });
  const squeezed = text.replace(/\s+/g, " ").trim();
  const ends = new RegExp(`^(${escaped(separator)}\\s*)+|(\\s*${escaped(separator)})+$`, "g");
  return separator === "" ? squeezed : squeezed.replace(ends, "").trim();
}

function escaped(text: string): string {
  return text.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}
