// The tag routes of the Admin API, for the Tags and categories screen.
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
