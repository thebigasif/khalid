// The editor's fields, reached from beside the form so the AI suggestions
// add to the editor without changing its components.

/** A field a writer types in. */
export type TextField = HTMLInputElement | HTMLTextAreaElement;

/** The field with `id` inside `root`, when it is one a writer types in. */
export function textField(root: ParentNode, id: string): TextField | null {
  const found = root.querySelector(`#${id}`);
  return found instanceof HTMLInputElement || found instanceof HTMLTextAreaElement ? found : null;
}

/**
 * Puts `value` in `field` as typing would, so the form's own state takes
 * it, then leaves the field, so the editor saves as it does after typing.
 */
export function fillField(field: TextField, value: string): void {
  const kind = field instanceof HTMLTextAreaElement ? HTMLTextAreaElement : HTMLInputElement;
  const setter = Object.getOwnPropertyDescriptor(kind.prototype, "value")?.set;
  setter?.call(field, value);
  field.dispatchEvent(new Event("input", { bubbles: true }));
  field.dispatchEvent(new FocusEvent("focusout", { bubbles: true }));
}

/** `markdown` with `block` added at its end, a blank line between. */
export function withBlock(markdown: string, block: string): string {
  const text = markdown.trimEnd();
  return text === "" ? `${block}\n` : `${text}\n\n${block}\n`;
}

// Checks about words a writer can rewrite, where a suggestion can help. A
// check about a missing author, a slug or a broken link needs a person.
const WORDS = [
  "title",
  "description",
  "heading",
  "alt text",
  "keyword",
  "keyphrase",
  "introduction",
  "first paragraph",
  "sentence",
  "paragraph",
  "link text",
  "passive",
  "transition",
];

/** Whether a suggested fix can help with the check that found `what`. */
export function helps(what: string): boolean {
  const lower = what.toLowerCase();
  return WORDS.some((word) => lower.includes(word));
}

/** The text of the `n`th paragraph in `section`, trimmed, or "". */
export function paragraph(section: Element, n: number): string {
  return section.querySelectorAll("p").item(n)?.textContent?.trim() ?? "";
}
