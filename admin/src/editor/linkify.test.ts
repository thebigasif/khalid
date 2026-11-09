import { Editor } from "@tiptap/core";
import { describe, expect, it } from "vitest";
import { find, tokenize } from "./linkify";
import { extensions } from "./richText";

describe("finding web addresses in text", () => {
  it("finds addresses with a scheme or www, and emails, with their places", () => {
    const text = "See https://example.com/a?b=1, www.example.org or ask me@example.net.";
    expect(find(text).map(({ value, href, start, end }) => [value, href, start, end])).toEqual([
      ["https://example.com/a?b=1", "https://example.com/a?b=1", 4, 29],
      ["www.example.org", "http://www.example.org", 31, 46],
      ["me@example.net", "mailto:me@example.net", 54, 68],
    ]);
  });

  it("leaves plain words and bare domains alone", () => {
    expect(find("Internal links help example.com rank.")).toEqual([]);
  });

  it("splits a word into the link and the text around it", () => {
    const tokens = tokenize("(https://example.com)").map((token) => token.toObject("https"));
    expect(tokens.map(({ value, isLink }) => [value, isLink])).toEqual([
      ["(", false],
      ["https://example.com", true],
      [")", false],
    ]);
  });

  it("links an address as it is typed in the rich-text view", () => {
    const editor = new Editor({ extensions, content: "", contentType: "markdown" });
    editor.commands.insertContent("Read https://example.com/a ");
    expect(editor.getMarkdown()).toContain("](https://example.com/a)");
    editor.destroy();
  });
});
