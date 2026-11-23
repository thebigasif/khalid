import { describe, expect, it } from "vitest";
import type { AdminPost } from "../api/posts";
import { reachDraft, reachEdit, tagList } from "./reach";

const SETTINGS: AdminPost["settings"] = {
  co_authors: [],
  featured: false,
  review_state: "draft",
  access: "public",
  password_protected: false,
  tags: ["internal links", "seo"],
  category: "on-page-seo",
  share_cards: {},
  connections: { main_post: false, related: [] },
  listing: { keyword_synonyms: [], reviewed_by: [], sitemap: true },
  aliases: [],
  template: "wide",
  head_code: null,
};

describe("a post's filing and access in the editor", () => {
  it("holds each setting as its field shows it, with no password", () => {
    expect(reachDraft(SETTINGS)).toEqual({
      tags: "internal links, seo",
      category: "on-page-seo",
      featured: false,
      access: "public",
      passwordOn: false,
      newPassword: "",
      template: "wide",
      headCode: "",
    });
  });

  it("sends only what changed, and empty text clears a setting", () => {
    const saved = reachDraft(SETTINGS);
    expect(reachEdit(saved, saved)).toEqual({});
    const now = {
      ...saved,
      tags: "seo, Anchor text, ",
      category: "",
      featured: true,
      template: " ",
    };
    expect(reachEdit(saved, now)).toEqual({
      tags: ["seo", "Anchor text"],
      category: null,
      featured: true,
      template: null,
    });
    const members = {
      ...saved,
      access: "members" as const,
      headCode: '<meta name="a" content="b">',
    };
    expect(reachEdit(saved, members)).toEqual({
      access: "members",
      head_code: '<meta name="a" content="b">',
    });
  });

  it("sends a new password as typed, keeps a set one and removes it when turned off", () => {
    const open = reachDraft(SETTINGS);
    const typed = { ...open, passwordOn: true, newPassword: " open sesame" };
    expect(reachEdit(open, typed)).toEqual({ password: " open sesame" });
    expect(reachEdit(open, { ...open, passwordOn: true })).toEqual({});
    const locked = reachDraft({ ...SETTINGS, password_protected: true });
    expect(locked.passwordOn).toBe(true);
    expect(reachEdit(locked, locked)).toEqual({});
    expect(reachEdit(locked, { ...locked, passwordOn: false })).toEqual({ password: null });
  });

  it("drops empty tags and a tag listed twice, whatever its capitals", () => {
    expect(tagList("SEO, seo , , links")).toEqual(["SEO", "links"]);
    expect(tagList("")).toEqual([]);
  });
});
