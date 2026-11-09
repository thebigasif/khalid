// The `[seo]` table of site.toml as the settings API sends it: one plain
// object per part, such as `titles` or `crawlers`, holding only what the
// file sets. These read it field by field, never trusting its shape, and
// build the parts a section saves.

/** The `[seo]` table: each part by name, as site.toml writes it. */
export type SeoTable = Record<string, unknown>;

/** One part of the table, or a table inside one. */
export type Part = Record<string, unknown>;

function isPart(value: unknown): value is Part {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

/** The part or table at `name` in `table`; empty when it is missing or not a table. */
export function partOf(table: Part | undefined, name: string): Part {
  const value = table?.[name];
  return isPart(value) ? value : {};
}

/** The text at `key`, or empty. */
export function textOf(part: Part, key: string): string {
  const value = part[key];
  return typeof value === "string" ? value : "";
}

/** The list of texts at `key`, or none. */
export function listOf(part: Part, key: string): string[] {
  const value = part[key];
  return Array.isArray(value) ? value.filter((each) => typeof each === "string") : [];
}

/** The whole number at `key`, or `fallback`. */
export function numberOf(part: Part, key: string, fallback: number): number {
  const value = part[key];
  return typeof value === "number" && Number.isInteger(value) ? value : fallback;
}

/** The switch at `key`, or `fallback`. */
export function flagOf(part: Part, key: string, fallback: boolean): boolean {
  const value = part[key];
  return typeof value === "boolean" ? value : fallback;
}

/** Lines of text as a list: each line trimmed, empty ones left out. */
export function lines(text: string): string[] {
  return text
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => line !== "");
}

// Text for `value` with every object's keys in order, so two parts holding
// the same settings compare equal whatever order their keys came in.
function sorted(value: unknown): string {
  if (Array.isArray(value)) {
    return `[${value.map(sorted).join(",")}]`;
  }
  if (isPart(value)) {
    const keys = Object.keys(value).sort();
    return `{${keys.map((key) => `${JSON.stringify(key)}:${sorted(value[key])}`).join(",")}}`;
  }
  return JSON.stringify(value) ?? "null";
}

/** Whether two parts hold the same settings. */
export function same(a: unknown, b: unknown): boolean {
  return sorted(a) === sorted(b);
}

/**
 * `part` ready to save: empty text, empty lists and empty tables left out,
 * since a setting left out keeps its default. `null` when nothing is left,
 * which removes the part from site.toml.
 */
export function cleaned(part: Part): Part | null {
  const kept: Part = {};
  for (const [key, value] of Object.entries(part)) {
    const text = typeof value === "string" ? value.trim() : value;
    if (isPart(text)) {
      const inner = cleaned(text);
      if (inner !== null) {
        kept[key] = inner;
      }
    } else if (text !== "" && text !== undefined && text !== null) {
      if (!Array.isArray(text) || text.length > 0) {
        kept[key] = text;
      }
    }
  }
  return Object.keys(kept).length === 0 ? null : kept;
}
