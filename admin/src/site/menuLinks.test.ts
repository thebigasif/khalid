import { describe, expect, it } from "vitest";
import { navWords as WORDS } from "../strings/navWords";
import { madeLink, swapped, takesHref } from "./menuLinks";

describe("the links a menu takes", () => {
  it("takes a path on the blog or a full address, as site.toml does", () => {
    for (const good of [
      "/blog/about/",
      "/",
      "https://www.example.com/shop/",
      "http://example.com",
    ]) {
      expect(takesHref(good), good).toBe(true);
    }
    for (const bad of ["blog/about/", "//evil.example/", "/a b/", "javascript:alert(1)", ""]) {
      expect(takesHref(bad), bad).toBe(false);
    }
  });

  it("asks for the words first, then a good address, and keeps a chosen language", () => {
    expect(madeLink(" ", "/blog/", "")).toEqual({ problem: WORDS.labelMissing });
    expect(madeLink("Guides", "guides", "")).toEqual({ problem: WORDS.hrefWrong });
    expect(madeLink(" Guides ", " /blog/guides/ ", "")).toEqual({
      link: { label: "Guides", href: "/blog/guides/" },
    });
    expect(madeLink("Impressum", "/blog/de/impressum/", "de")).toEqual({
      link: { label: "Impressum", href: "/blog/de/impressum/", language: "de" },
    });
  });

  it("moves one item and keeps the rest in order", () => {
    expect(swapped(["a", "b", "c"], 2, 1)).toEqual(["a", "c", "b"]);
    expect(swapped(["a", "b", "c"], 0, 1)).toEqual(["b", "a", "c"]);
  });
});
