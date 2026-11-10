// The tag routes of the Admin API, for the Tags and categories screen.
import { call, type Reply } from "./call";
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
