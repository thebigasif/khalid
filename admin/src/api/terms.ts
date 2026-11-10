// The tag and category routes of the Admin API, for the Tags and categories screen.
import { call, nothing, type Reply, sending } from "./call";
import type { components } from "./schema";

/** A tag in one language, with how many posts have it. */
export type AdminTag = components["schemas"]["AdminTag"];

async function tags(response: Response): Promise<AdminTag[]> {
  const body: components["schemas"]["AdminTags"] = await response.json();
  return body.tags;
}

/** Every tag the site's posts carry, drafts included, in every language. */
export function listTags(): Promise<Reply<AdminTag[]>> {
  return call("/tags/", {}, tags);
}

/** Renames `tag` to `name` in every post that has it; a name in use merges the two. */
export function renameTag(tag: AdminTag, name: string): Promise<Reply<AdminTag[]>> {
  const body = { tags: [{ name, updated_at: tag.updated_at }] };
  return call(`/tags/${encodeURIComponent(tag.id)}/`, sending("PUT", body), tags);
}

/** Takes `tag` off every post that has it, unless one changed since the list was read. */
export function removeTag(tag: AdminTag): Promise<Reply<null>> {
  const query = new URLSearchParams({ updated_at: tag.updated_at });
  return call(`/tags/${encodeURIComponent(tag.id)}/?${query}`, { method: "DELETE" }, nothing);
}

/** A category with its name in each language and how many posts are in it. */
export type AdminCategory = components["schemas"]["AdminCategory"];

/** A new category: the slug posts name it by, and its name by language code. */
export type NewCategory = components["schemas"]["NewCategory"];

async function categories(response: Response): Promise<AdminCategory[]> {
  const body: components["schemas"]["AdminCategories"] = await response.json();
  return body.categories;
}

/** Every category of the site, with its names and how many posts are in it. */
export function listCategories(): Promise<Reply<AdminCategory[]>> {
  return call("/categories/", {}, categories);
}

/** Adds a category to `categories.toml`. */
export function createCategory(category: NewCategory): Promise<Reply<AdminCategory[]>> {
  return call("/categories/", sending("POST", { categories: [category] }), categories);
}

/** Replaces every name of `category` with `names`; posts keep the category. */
export function renameCategory(
  category: AdminCategory,
  names: Record<string, string>,
): Promise<Reply<AdminCategory[]>> {
  const body = { categories: [{ names, updated_at: category.updated_at }] };
  return call(`/categories/${encodeURIComponent(category.id)}/`, sending("PUT", body), categories);
}

/** Deletes a category no post is in, unless the categories changed since the list was read. */
export function deleteCategory(category: AdminCategory): Promise<Reply<null>> {
  const query = new URLSearchParams({ updated_at: category.updated_at });
  const path = `/categories/${encodeURIComponent(category.id)}/?${query}`;
  return call(path, { method: "DELETE" }, nothing);
}
