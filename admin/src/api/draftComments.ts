import { call, nothing, type Reply, sending } from "./call";
import type { components } from "./schema";

/** A comment staff left on a draft for each other; readers never see it. */
export type DraftComment = components["schemas"]["DraftComment"];
/** A new comment: about the whole post, one block from 1, or answering a thread. */
export type NewDraftComment = components["schemas"]["DraftCommentBody"];

const path = (id: string) => `/posts/${encodeURIComponent(id)}/draft-comments/`;

// An answer with no list counts as a failure, so the panel shows a problem instead of breaking.
async function draftComments(response: Response): Promise<DraftComment[]> {
  const body: Partial<components["schemas"]["DraftCommentList"]> = await response.json();
  if (!Array.isArray(body.draft_comments)) {
    throw new TypeError("The answer has no list of comments.");
  }
  return body.draft_comments;
}

/** Every comment on post `id`, oldest first, each answer after the comment it answers. */
export function listDraftComments(id: string): Promise<Reply<DraftComment[]>> {
  return call(path(id), {}, draftComments);
}

/** Adds one comment to post `id`; the people in its thread and the editors get an email. */
export function addDraftComment(
  id: string,
  comment: NewDraftComment,
): Promise<Reply<DraftComment[]>> {
  const body: components["schemas"]["NewDraftComments"] = { draft_comments: [comment] };
  return call(path(id), sending("POST", body), draftComments);
}

/** Marks the thread that `first` begins as done, or open again. */
export function resolveThread(id: string, first: number, resolved: boolean): Promise<Reply<null>> {
  const body: components["schemas"]["ResolveThreads"] = { draft_comments: [{ resolved }] };
  return call(`${path(id)}${first}/`, sending("PUT", body), nothing);
}
