import { describe, expect, it } from "vitest";
import { withLink } from "./linkInsert";

const PATH = "/blog/choosing-anchor-text/";

describe("putting a suggested link into the text", () => {
  it("links the first words that name the post, outside any link", () => {
    const text = "See [anchor text](/blog/a/) first. Good anchor text says where.\n";
    expect(withLink(text, "anchor text", "Choosing anchor text", PATH)).toBe(
      `See [anchor text](/blog/a/) first. Good [anchor text](${PATH}) says where.\n`,
    );
  });

  it("adds a related line at the end when the text never names it", () => {
    expect(withLink("Links help readers.\n\n", null, "Choosing anchor text", PATH)).toBe(
      `Links help readers.\n\nRelated: [Choosing anchor text](${PATH}).\n`,
    );
  });
});
