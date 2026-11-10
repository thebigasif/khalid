import type { DraftComment } from "../../api/draftComments";
import { draftCommentWords as words } from "../../strings/draftCommentWords";

/** A comment and the answers to it, oldest first. */
export type Thread = { first: DraftComment; answers: DraftComment[] };

/** The threads in `all`, in the order their first comments came. */
export function threadsOf(all: DraftComment[]): Thread[] {
  const firsts = all.filter((comment) => comment.parent == null);
  return firsts.map((first) => ({
    first,
    answers: all.filter((comment) => comment.parent === first.id),
  }));
}

/** What a thread is about, named from `parts`, the text's parts as the editor has them now. */
export function aboutLine(block: number | null | undefined, parts: string[]): string {
  if (block == null) {
    return words.wholePost;
  }
  const name = parts[block - 1];
  return name === undefined ? words.gonePart(block) : words.aboutPart(block, name);
}
