import { describe, expect, it } from "vitest";
import { en } from "../strings/en";
import { shellWords } from "../strings/shellWords";
import { NAV, whereIs } from "./navigation";

describe("where a screen sits in the side menu", () => {
  it("names the group and the screen of each address", () => {
    expect(whereIs("/")?.item.label).toBe(en.nav.dashboard);
    expect(whereIs("/posts/01JA")?.item.label).toBe(en.nav.posts);
    expect(whereIs("/team/owner")?.group.label).toBe(shellWords.groups.settings);
    expect(whereIs("/media")?.group.label).toBe(shellWords.groups.write);
  });

  it("puts starting a post under Posts, and leaves out what the menu does not open", () => {
    expect(whereIs("/new")?.item.label).toBe(en.nav.posts);
    expect(whereIs("/sites")).toBeNull();
    expect(whereIs("/postsx")).toBeNull();
  });

  it("opens every screen once", () => {
    const paths = NAV.flatMap((group) => group.items.map((item) => item.to));
    expect(new Set(paths).size).toBe(paths.length);
    expect(NAV.map((group) => group.label)).toEqual([
      shellWords.groups.write,
      shellWords.groups.grow,
      shellWords.groups.site,
      shellWords.groups.settings,
    ]);
  });
});
