import { describe, expect, it } from "vitest";
import type { AdminPost } from "../api/posts";
import { changes, lacks, seoEdit, seoFields } from "./bulkSeo";

const POST: AdminPost = {
  id: "en.anchor-words",
  slug: "anchor-words",
  title: "Anchor words",
  status: "published",
  markdown: "Pick link words with care.\n",
  updated_at: "2026-11-02T09:00:00Z",
  settings: {
    co_authors: [],
    featured: false,
    review_state: "ready",
    access: "public",
    password_protected: false,
    share_cards: {},
    aliases: [],
    excerpt: "How to pick the words of a link.",
    focus_keyword: "anchor text",
  },
};

describe("the bulk SEO editor's rows", () => {
  it("reads the three fields, empty for none", () => {
    expect(seoFields(POST)).toEqual({
      title: "",
      description: "How to pick the words of a link.",
      keyword: "anchor text",
    });
  });

  it("keeps the rows that lack the field the filter names", () => {
    const fields = seoFields(POST);
    expect(lacks(fields, null)).toBe(true);
    expect(lacks(fields, "title")).toBe(true);
    expect(lacks(fields, "description")).toBe(false);
    expect(lacks({ ...fields, keyword: "  " }, "keyword")).toBe(true);
  });

  it("sends only the fields that changed, clearing an emptied one", () => {
    const saved = seoFields(POST);
    const now = { title: "Anchor text that helps", description: saved.description, keyword: " " };
    const edit = seoEdit(POST, saved, now);
    expect(edit).toEqual({
      updated_at: POST.updated_at,
      settings: { seo_title: "Anchor text that helps", focus_keyword: null },
    });
    expect(changes(edit)).toBe(true);
    expect(changes(seoEdit(POST, saved, { ...saved, keyword: "anchor text " }))).toBe(false);
  });
});
