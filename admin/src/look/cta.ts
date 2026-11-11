// The call to action on the Look screen: `[template.cta]` as the three
// lines the form edits, and what a save sends for them.
import type { Cta } from "../api/site";
import type { Template } from "../api/templates";

/** The call to action as the form holds it, each line empty when not set. */
export type CtaLines = { label: string; href: string; note: string };

/** The lines the form starts with for `cta`. */
export function ctaLines(cta: Cta | null | undefined): CtaLines {
  return { label: cta?.label ?? "", href: cta?.href ?? "", note: cta?.note ?? "" };
}

/** `lines` as sent, trimmed: `null` when every line is empty, which removes it. */
export function ctaOf(lines: CtaLines): Cta | null {
  const label = lines.label.trim();
  const href = lines.href.trim();
  const note = lines.note.trim();
  if (label === "" && href === "" && note === "") {
    return null;
  }
  return note === "" ? { label, href } : { label, href, note };
}

/** Whether `lines` say something other than the `saved` call to action. */
export function ctaChanged(saved: Cta | null | undefined, lines: CtaLines): boolean {
  return JSON.stringify(ctaOf(ctaLines(saved))) !== JSON.stringify(ctaOf(lines));
}

/** Whether `template` shows the call to action as a band or a header button. */
export function showsCta(template: Template): boolean {
  const keys = ["cta_band", "cta_button"];
  return template.options.some((option) => keys.includes(option.key));
}
