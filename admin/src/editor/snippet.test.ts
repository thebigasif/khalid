import { describe, expect, it } from "vitest";
import { boldParts, shownAddress } from "./snippet";

describe("the address in a search result", () => {
  it("shows the site, then each folder", () => {
    const url = "https://www.example.com/blog/why-internal-links-matter/";
    expect(shownAddress(url)).toBe("https://www.example.com › blog › why-internal-links-matter");
  });

  it("shows anything that is not an address as it is", () => {
    expect(shownAddress("")).toBe("");
  });
});

describe("the searched words in bold", () => {
  it("marks every place the keyword is, in any case", () => {
    expect(boldParts("Anchor text: pick anchor text with care", "anchor text")).toEqual([
      { text: "Anchor text", bold: true, at: 0 },
      { text: ": pick ", bold: false, at: 11 },
      { text: "anchor text", bold: true, at: 18 },
      { text: " with care", bold: false, at: 29 },
    ]);
  });

  it("leaves the text whole without a keyword", () => {
    expect(boldParts("Pick words", " ")).toEqual([{ text: "Pick words", bold: false, at: 0 }]);
  });
});
