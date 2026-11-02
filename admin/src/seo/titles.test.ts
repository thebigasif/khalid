import { describe, expect, it } from "vitest";
import { cleaned, listOf, partOf, textOf } from "./parts";
import { defaultTemplate, filled, type Sample } from "./titles";

const SAMPLE: Sample = {
  title: "Choosing anchor text",
  site: "Field Notes on SEO",
  sep: "",
  tagline: "",
  excerpt: "Pick words that say where the link goes.",
  category: "Technical SEO",
  author: "Asif Khan",
};

describe("title templates", () => {
  it("fills each variable and the separator", () => {
    expect(filled("{title} {sep} {site}", SAMPLE, "·")).toBe(
      "Choosing anchor text · Field Notes on SEO",
    );
    expect(filled("{category}: {title}", SAMPLE, "|")).toBe("Technical SEO: Choosing anchor text");
  });

  it("drops a separator left at an end by an empty variable", () => {
    expect(filled("{site} {sep} {tagline}", SAMPLE, "|")).toBe("Field Notes on SEO");
  });

  it("keeps a variable it does not know as written", () => {
    expect(filled("{title} {year}", SAMPLE, "|")).toBe("Choosing anchor text {year}");
  });

  it("gives the home page its own defaults", () => {
    expect(defaultTemplate("home", "title")).toBe("{site} {sep} {tagline}");
    expect(defaultTemplate("post", "description")).toBe("{excerpt}");
  });
});

describe("parts of the seo table", () => {
  it("reads only the shapes it expects", () => {
    const seo = { titles: { separator: "·", post: ["wrong"] }, crawlers: { block: ["GPTBot", 3] } };
    expect(textOf(partOf(seo, "titles"), "separator")).toBe("·");
    expect(partOf(partOf(seo, "titles"), "post")).toEqual({});
    expect(listOf(partOf(seo, "crawlers"), "block")).toEqual(["GPTBot"]);
  });

  it("leaves out what is empty, and a part with nothing left is null", () => {
    expect(cleaned({ separator: " · ", tagline: "", post: { title: "" }, block: [] })).toEqual({
      separator: "·",
    });
    expect(cleaned({ tagline: " ", post: {} })).toBeNull();
  });
});
