import { screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { adminBlog } from "../../test/blog";
import { axeViolations, json, renderAt, signedIn } from "../../test/render";
import { row, rowsPage } from "../../test/rows";
import type { AdminPost } from "../api/posts";
import { seoToolWords } from "../strings/seoToolWords";

const words = seoToolWords.bulk;

/** A post with a description and a focus keyword but no SEO title, unless `settings` say so. */
function post(id: string, title: string, settings: Partial<AdminPost["settings"]> = {}): AdminPost {
  return {
    id,
    slug: id.toLowerCase(),
    title,
    status: "published",
    markdown: "Text.\n",
    updated_at: "2026-11-02T09:00:00Z",
    settings: {
      co_authors: [],
      featured: false,
      review_state: "ready",
      access: "public",
      password_protected: false,
      tags: [],
      share_cards: {},
      aliases: [],
      excerpt: "How to pick the words of a link.",
      focus_keyword: "anchor text",
      ...settings,
    },
  };
}

// The bare post has no focus keyword either, so the keyword filter keeps it.
const BARE = post("01JA", "Why internal links matter", { focus_keyword: null });
const DONE = post("01JB", "Planning a topic cluster", { seo_title: "Topic clusters, planned" });

/** A blog with both posts that records every request. */
function blog() {
  return adminBlog({
    "GET /posts/": () =>
      rowsPage([row({ id: "01JA", title: BARE.title }), row({ id: "01JB", title: DONE.title })]),
    "GET /posts/01JA/": () => json(200, { posts: [BARE] }),
    "GET /posts/01JB/": () => json(200, { posts: [DONE] }),
    "PUT /posts/01JA/": () =>
      json(200, { posts: [{ ...BARE, settings: { ...BARE.settings, seo_title: "Links" } }] }),
  });
}

describe("the bulk SEO editor", () => {
  it("edits a post's SEO title in its row and saves only what changed", async () => {
    signedIn();
    const sent = blog();
    renderAt("/bulk-seo");
    expect(await screen.findByRole("heading", { level: 1, name: words.title })).toBeVisible();
    const field = await screen.findByRole("textbox", {
      name: words.labelFor(words.seoTitle, BARE.title),
    });
    const save = screen.getByRole("button", { name: words.saveFor(BARE.title) });
    expect(save).toBeDisabled();
    expect(await axeViolations()).toEqual([]);
    await userEvent.type(field, "Links");
    await userEvent.click(save);
    const row = save.closest("tr");
    expect(await within(row ?? document.body).findByText(words.saved)).toBeVisible();
    const put = sent.find((request) => request.method === "PUT");
    expect(await put?.json()).toEqual({
      posts: [{ updated_at: BARE.updated_at, settings: { seo_title: "Links" } }],
    });
  });

  it("counts characters under the title and description but not the focus keyword", async () => {
    signedIn();
    blog();
    renderAt("/bulk-seo");
    const cell = async (field: string) =>
      (await screen.findByRole("textbox", { name: words.labelFor(field, DONE.title) })).closest(
        "td",
      ) ?? document.body;
    const title = DONE.settings.seo_title ?? "";
    expect(
      within(await cell(words.seoTitle)).getByText(words.length(title.length, 60)),
    ).toBeVisible();
    expect((await cell(words.keyword)).textContent).toBe("anchor text");
  });

  it("shows only the posts with no SEO title when asked", async () => {
    signedIn();
    blog();
    renderAt("/bulk-seo");
    await screen.findByRole("link", { name: DONE.title });
    await userEvent.selectOptions(screen.getByLabelText(words.show), "title");
    expect(await screen.findByRole("link", { name: BARE.title })).toBeVisible();
    expect(screen.queryByRole("link", { name: DONE.title })).toBeNull();
    expect(screen.getByText(words.hidden)).toBeVisible();
  });

  it("asks the list for posts with no focus keyword", async () => {
    signedIn();
    const sent = blog();
    renderAt("/bulk-seo");
    await screen.findByRole("link", { name: DONE.title });
    await userEvent.selectOptions(screen.getByLabelText(words.show), "keyword");
    await screen.findByRole("link", { name: BARE.title });
    const asked = sent.map((request) => new URL(request.url).searchParams.get("filter"));
    expect(asked).toContain("has_focus_keyword:false");
  });
});
