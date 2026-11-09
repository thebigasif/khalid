// The SEO workouts' steps: which link each one asks for, and adding it to
// the post that should hold it.
import { problemMessage, readPost, savePost } from "../api/posts";
import type { LinkSuggestion } from "../api/seo";
import { withLink } from "../editor/linkInsert";
import { en } from "../strings/en";

/** The post a link leads to: its title and its address on the site. */
export type Target = { title: string; path: string };

/** The pillar of a cluster member's topic among its suggestions, when it does not link there yet. */
export function pillarOf(suggestions: LinkSuggestion[]): LinkSuggestion | null {
  return suggestions.find((each) => each.shares.pillar) ?? null;
}

/** The posts an orphan is closest to, best first, which could each link to it. */
export function relatedTo(suggestions: LinkSuggestion[], most = 3): LinkSuggestion[] {
  return suggestions.slice(0, most);
}

/**
 * Adds a link to `to` in post `from`, on the words `anchor` when its text holds them outside
 * a link, or else as a Related line at the end; `null` when saved, or else why not.
 */
export async function addLink(from: string, to: Target, anchor: string | null) {
  const read = await readPost(from);
  if (!("post" in read)) {
    return problemMessage(read.problem, en.problems.unreachable).what;
  }
  const { post } = read;
  const markdown = withLink(post.markdown, anchor, to.title, to.path);
  const saved = await savePost(from, { updated_at: post.updated_at, markdown });
  return "post" in saved ? null : problemMessage(saved.problem, en.problems.unreachable).what;
}
