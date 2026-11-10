import { describe, expect, it } from "vitest";
import type { AdminPost } from "../api/posts";
import { basicsOf, editFrom, tagsOf } from "./quickEdit";

const POST: AdminPost = {
  id: "en.anchor-words",
  slug: "anchor-words",
  title: "Anchor words",
  author: "asif",
  status: "published",
  markdown: "Pick link words with care.\n",
  updated_at: "2026-11-02T09:00:00Z",
  settings: {
    co_authors: [],
    featured: false,
    review_state: "ready",
    access: "public",
    password_protected: false,
    tags: ["internal links", "SEO"],
    category: "on-page-seo",
    share_cards: {},
    aliases: [],
    excerpt: "How to pick the words of a link.",
  },
};

describe("quick edit", () => {
  it("shows the basics as the post has them", () => {
    expect(basicsOf(POST)).toEqual({
      title: "Anchor words",
      description: "How to pick the words of a link.",
      tags: "internal links, SEO",
      category: "on-page-seo",
      author: "asif",
    });
  });

  it("reads tags between commas, each once whatever its capitals", () => {
    expect(tagsOf(" seo, Internal links,, SEO ,internal LINKS ")).toEqual([
      "seo",
      "Internal links",
    ]);
    expect(tagsOf(" , ")).toEqual([]);
  });

  it("saves nothing when nothing changed, spaces aside", () => {
    const same = { ...basicsOf(POST), title: " Anchor words ", tags: "internal links,SEO" };
    expect(editFrom(POST, same)).toBeNull();
  });

  it("sends only what changed, clearing what was emptied", () => {
    const basics = { ...basicsOf(POST), description: " ", tags: "SEO", category: "", author: "" };
    expect(editFrom(POST, basics)).toEqual({
      updated_at: POST.updated_at,
      author: null,
      settings: { excerpt: null, tags: ["SEO"], category: null },
    });
    expect(editFrom(POST, { ...basicsOf(POST), title: "Anchor text" })).toEqual({
      updated_at: POST.updated_at,
      title: "Anchor text",
    });
  });
});
