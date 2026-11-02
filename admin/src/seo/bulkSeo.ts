// The bulk SEO editor's rows: each post's SEO title, meta description and
// focus keyword as three fields, what an edit of them sends, and which
// rows a "missing" filter keeps.
import type { AdminPost, PostEdit } from "../api/posts";

/** The three fields of one row, as text: empty for none. */
export type SeoFields = { title: string; description: string; keyword: string };

/** What the screen can show only the posts that lack. */
export const MISSING = ["title", "description", "keyword"] as const;

/** One of the fields a post may lack. */
export type Missing = (typeof MISSING)[number];

/** The fields of `post` as they are saved. */
export function seoFields(post: AdminPost): SeoFields {
  const { settings } = post;
  return {
    title: settings.seo_title ?? "",
    description: settings.excerpt ?? "",
    keyword: settings.focus_keyword ?? "",
  };
}

/** Whether `fields` leave `missing` empty, so the filter keeps the row; any row without one. */
export function lacks(fields: SeoFields, missing: Missing | null): boolean {
  return missing === null || fields[missing].trim() === "";
}

function orNull(text: string): string | null {
  const trimmed = text.trim();
  return trimmed === "" ? null : trimmed;
}

/** The edit that saves `now` over `saved` on the copy of `post`: only the fields that differ. */
export function seoEdit(post: AdminPost, saved: SeoFields, now: SeoFields): PostEdit {
  const settings: NonNullable<PostEdit["settings"]> = {};
  if (now.title.trim() !== saved.title.trim()) {
    settings.seo_title = orNull(now.title);
  }
  if (now.description.trim() !== saved.description.trim()) {
    settings.excerpt = orNull(now.description);
  }
  if (now.keyword.trim() !== saved.keyword.trim()) {
    settings.focus_keyword = orNull(now.keyword);
  }
  return { updated_at: post.updated_at, settings };
}

/** Whether an edit made by `seoEdit` changes anything. */
export function changes(edit: PostEdit): boolean {
  return Object.keys(edit.settings ?? {}).length > 0;
}
