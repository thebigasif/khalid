import { describe, expect, it } from "vitest";
import { BLOCK_SPECS } from "../blocks/specs";
import { MENU_LENGTH, matchingItems, SLASH_ITEMS } from "./slashItems";

const ids = (query: string) => matchingItems(query).map((item) => item.id);

describe("the slash menu's blocks", () => {
  it("offers every directive block", () => {
    for (const spec of BLOCK_SPECS) {
      expect(SLASH_ITEMS.some((item) => item.spec === spec)).toBe(true);
    }
  });

  it("names and explains every block", () => {
    for (const item of SLASH_ITEMS) {
      expect(item.name).not.toBe("");
      expect(item.hint).not.toBe("");
    }
  });

  it("shows the first few blocks before anything is typed", () => {
    expect(ids("")).toEqual(SLASH_ITEMS.slice(0, MENU_LENGTH).map((item) => item.id));
  });

  it("finds a block by its name or its directive", () => {
    expect(ids("call")[0]).toBe("callout");
    expect(ids("Toc")[0]).toBe("toc");
    expect(ids("table of")[0]).toBe("toc");
  });

  it("finds a block by a word of what it does", () => {
    expect(ids("steps")).toEqual(expect.arrayContaining(["howto", "numbers"]));
  });

  it("puts a name that starts with the words before a hint that holds them", () => {
    expect(ids("video")[0]).toBe("video");
  });

  it("gives every item its own id", () => {
    const all = SLASH_ITEMS.map((item) => item.id);
    expect(new Set(all).size).toBe(all.length);
  });

  it("offers the grid table first and the folding table after it", () => {
    expect(ids("table")[0]).toBe("table");
    expect(ids("table")).toContain("folding-table");
    expect(ids("fold")[0]).toBe("folding-table");
  });

  it("finds the blocks Google reads by their names", () => {
    for (const block of ["book", "chart", "course", "dataset", "event", "takeaways"]) {
      expect(ids(block)[0]).toBe(block);
    }
    expect(ids("privacy")[0]).toBe("privacy-facts");
  });

  it("finds nothing for words no block has", () => {
    expect(ids("zzz")).toEqual([]);
  });

  it("offers photo and GIF search only when the owner turned them on", () => {
    expect(ids("unsplash")).toEqual([]);
    const gifs = matchingItems("gif", { unsplash: false, gifs: true }).map((item) => item.id);
    expect(gifs[0]).toBe("gifs");
    expect(gifs).not.toContain("unsplash");
    expect(matchingItems("photo", { unsplash: true, gifs: true })[0]?.id).toBe("unsplash");
  });

  it("always offers a picture from the media library", () => {
    expect(ids("picture")[0]).toBe("library");
    expect(ids("lib")).toEqual(["library"]);
  });
});
