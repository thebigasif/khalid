import { describe, expect, it } from "vitest";
import type { DraftComment } from "../../api/draftComments";
import { draftCommentWords as words } from "../../strings/draftCommentWords";
import { aboutLine, threadsOf } from "./threads";

const said = (id: number, parent: number | null): DraftComment => ({
  id,
  parent,
  block: null,
  author: "maria",
  text: `Comment ${id}`,
  resolved: false,
  created_at: "2026-11-10T10:42:00Z",
});

describe("threads on a draft", () => {
  it("puts each answer under the comment it answers, threads in the order they began", () => {
    const threads = threadsOf([said(1, null), said(2, null), said(3, 1), said(4, 2), said(5, 1)]);
    expect(threads.map((t) => [t.first.id, t.answers.map((a) => a.id)])).toEqual([
      [1, [3, 5]],
      [2, [4]],
    ]);
  });

  it("names what a thread is about from the text's parts as they are now", () => {
    const parts = ["Why internal links matter", ""];
    expect(aboutLine(null, parts)).toBe(words.wholePost);
    expect(aboutLine(1, parts)).toBe('About part 1: "Why internal links matter"');
    expect(aboutLine(2, parts)).toBe("About part 2");
    expect(aboutLine(3, parts)).toBe(words.gonePart(3));
  });
});
