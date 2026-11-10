import { describe, expect, it } from "vitest";
import { partNames } from "./parts";

describe("the parts of a post's text", () => {
  it("counts one part per run of lines between blank lines, named by its first words", () => {
    const text =
      "# Why internal links matter\n\nInternal links help readers and Google find the rest.\n\n\n- One\n- Two\n";
    expect(partNames(text)).toEqual([
      "Why internal links matter",
      "Internal links help readers and Google…",
      "One",
    ]);
  });

  it("keeps a code block whole, blank lines and all", () => {
    const text = "Before.\n\n```sh\nkhalid serve\n\nkhalid build\n```\n\nAfter.";
    expect(partNames(text)).toEqual(["Before.", "sh", "After."]);
  });

  it("keeps a block whole up to its own closing fence, inner blocks included", () => {
    const text =
      "::::faq\n:::question\nWhy?\n\nBecause.\n:::\n\n:::question\nHow?\n:::\n::::\n\nAfter.";
    expect(partNames(text)).toEqual(["faq", "After."]);
  });

  it("names a part with no words by its number alone", () => {
    expect(partNames("---\n\nText.")).toEqual(["", "Text."]);
  });

  it("finds no parts in an empty text", () => {
    expect(partNames("\n\n")).toEqual([]);
  });
});
