import { call, nothing, type Reply, sending } from "./call";
import type { components } from "./schema";

/** A comment as the Admin API sends it, in any status. */
export type AdminComment = components["schemas"]["AdminComment"];
/** Where a comment stands in moderation: pending, approved, spam or trash. */
export type CommentStatus = components["schemas"]["CommentStatus"];

// An answer with no list counts as a failure, so a screen hides the count instead of breaking.
async function comments(response: Response): Promise<AdminComment[]> {
  const body: Partial<components["schemas"]["AdminComments"]> = await response.json();
  if (!Array.isArray(body.comments)) {
    throw new TypeError("The answer has no list of comments.");
  }
  return body.comments;
}

/** The newest 100 comments with one status. */
export function listComments(status: CommentStatus): Promise<Reply<AdminComment[]>> {
  return call(`/comments/?status=${status}&limit=100`, {}, comments);
}

/** Shows a comment under its post. */
export function approveComment(id: number): Promise<Reply<AdminComment[]>> {
  return call(`/comments/${id}/approve/`, { method: "POST" }, comments);
}

/** Marks a comment as spam, which hides it from its post. */
export function markSpam(id: number): Promise<Reply<AdminComment[]>> {
  return call(`/comments/${id}/spam/`, { method: "POST" }, comments);
}

/** Puts a comment marked as spam back among the waiting ones. */
export function notSpam(id: number): Promise<Reply<AdminComment[]>> {
  return call(`/comments/${id}/spam/`, { method: "DELETE" }, comments);
}

/** Answers a comment as staff; the answer shows at once and approves what it answers. */
export function replyToComment(
  id: number,
  name: string,
  text: string,
): Promise<Reply<AdminComment[]>> {
  const body: components["schemas"]["CommentReplies"] = { comments: [{ author_name: name, text }] };
  return call(`/comments/${id}/replies/`, sending("POST", body), comments);
}

/** Removes a comment and every reply to it. */
export function deleteComment(id: number): Promise<Reply<null>> {
  return call(`/comments/${id}/`, { method: "DELETE" }, nothing);
}
