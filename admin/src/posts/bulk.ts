import { type Problem, problemMessage, savePost } from "../api/posts";
import { trashPost } from "../api/records";
import { addRedirect, isData } from "../api/redirects";
import type { PostRow } from "../api/rows";
import { en } from "../strings/en";

/**
 * What a live post's old address does once the post is in the trash: say it
 * is gone for good (410), or send readers on to the post at `to` (301).
 */
export type Farewell = { kind: "gone" } | { kind: "moved"; to: string };

/** What one change to many posts does to each of them. */
export type BulkAction =
  | { kind: "author"; author: string }
  | { kind: "trash"; farewell: Farewell };

/** What a change to many posts came to: how many changed, and what to say about the others. */
export type BulkResult = { changed: number; left: string[] };

/** What one post's change came to: whether it changed, and anything to tell about it. */
type Outcome = { changed: boolean; note: string | null };

/** The address of `row` on the site when it is live, which a farewell is needed for. */
export function liveAddress(row: PostRow): string | null {
  return row.status === "published" ? (row.path ?? null) : null;
}

/**
 * Changes each of `rows` on its own, each from the copy its row was read from, so a post
 * someone else changed meanwhile is left alone and one refusal never stops the rest.
 */
export async function changeAll(rows: PostRow[], action: BulkAction): Promise<BulkResult> {
  const outcomes = await Promise.all(rows.map((row) => changeOne(row, action)));
  const changed = outcomes.filter((outcome) => outcome.changed).length;
  const left = outcomes.flatMap((outcome) => (outcome.note === null ? [] : [outcome.note]));
  return { changed, left };
}

/** Changes one post, and says why not when it could not. */
async function changeOne(row: PostRow, action: BulkAction): Promise<Outcome> {
  const refused = (problem: Problem | null): Outcome => {
    const why = problemMessage(problem, en.problems.unreachable).what;
    return { changed: false, note: en.bulk.left(row.title, why) };
  };
  if (action.kind === "trash") {
    const reply = await trashPost(row.id, row.updated_at);
    return isData(reply) ? farewell(row, action.farewell) : refused(reply.problem);
  }
  const edit = { updated_at: row.updated_at, author: action.author };
  const answer = await savePost(row.id, edit);
  return "post" in answer ? { changed: true, note: null } : refused(answer.problem);
}

/** Stores what a trashed live post's old address does now. */
async function farewell(row: PostRow, said: Farewell): Promise<Outcome> {
  const from = liveAddress(row);
  if (from === null) {
    return { changed: true, note: null };
  }
  const reply = await addRedirect(from, said.kind === "moved" ? said.to : null);
  if (isData(reply)) {
    return { changed: true, note: null };
  }
  const why = problemMessage(reply.problem, en.problems.unreachable).what;
  return { changed: true, note: en.bulk.noFarewell(row.title, from, why) };
}
