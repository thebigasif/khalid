import type { AdminPage, PageEdit } from "../api/pages";

/** A page's fields as the form holds them: plain text. */
export type PageFields = { title: string; description: string; markdown: string };

/** The form's starting text for `page`. */
export function fieldsOf(page: AdminPage): PageFields {
  return { title: page.title, description: page.description ?? "", markdown: page.markdown };
}

/**
 * The edit that turns `page` into `fields`, with only the fields that
 * changed; `null` when nothing did. An empty description removes it, as
 * the API reads it. The text is sent as typed, since spaces can matter in
 * markdown; only a change at its end alone does not count.
 */
export function changes(page: AdminPage, fields: PageFields): PageEdit | null {
  const before = fieldsOf(page);
  const edit: PageEdit = { updated_at: page.updated_at };
  let changed = false;
  for (const key of ["title", "description"] as const) {
    if (fields[key].trim() !== before[key].trim()) {
      edit[key] = fields[key].trim();
      changed = true;
    }
  }
  if (fields.markdown.trimEnd() !== before.markdown.trimEnd()) {
    edit.markdown = fields.markdown;
    changed = true;
  }
  return changed ? edit : null;
}
