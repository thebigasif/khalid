// The Admin API's SEO routes the editor, the dashboard and the link
// screens use: link suggestions, copying a post, and the site's issues.
import { call, type Reply } from "./call";
import type { AdminPost } from "./posts";
import type { components } from "./schema";

/** One post worth linking to from the post being written. */
export type LinkSuggestion = components["schemas"]["LinkSuggestion"];
/** One lint rule that fires on the site, as a task. */
export type SiteIssue = components["schemas"]["SiteIssue"];

/** Up to ten posts that post `id` should link to and does not yet, best first. */
export function readLinkSuggestions(id: string): Promise<Reply<LinkSuggestion[]>> {
  const path = `/posts/${encodeURIComponent(id)}/link-suggestions/`;
  return call(path, {}, async (response) => {
    const body: { suggestions?: unknown } = await response.json();
    return Array.isArray(body.suggestions) ? body.suggestions : [];
  });
}

/** Copies post `id` as a new draft, and gives the copy. */
export function duplicatePost(id: string): Promise<Reply<AdminPost>> {
  const path = `/posts/${encodeURIComponent(id)}/duplicate/`;
  return call(path, { method: "POST" }, async (response) => {
    const body: { posts: AdminPost[] } = await response.json();
    const [copy] = body.posts;
    if (copy === undefined) {
      throw new Error("the copy was not sent back");
    }
    return copy;
  });
}

/** The live site's most pressing lint problems, those that stop posts first. */
export function readSiteIssues(): Promise<Reply<SiteIssue[]>> {
  return call("/lint/issues/", {}, async (response) => {
    const body: { issues?: unknown } = await response.json();
    return Array.isArray(body.issues) ? body.issues : [];
  });
}
