import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { adminBlog } from "../../test/blog";
import { axeViolations, json, renderAt, signedIn } from "../../test/render";
import { row, rowsPage } from "../../test/rows";
import type { AdminPost } from "../api/posts";
import type { LinkSuggestion } from "../api/seo";
import { seoToolWords } from "../strings/seoToolWords";

const words = seoToolWords.workouts;

const ORPHAN = row({
  id: "en.anchor-words",
  title: "Anchor words",
  path: "/blog/anchor-words/",
  links: { inbound: 0, outbound: 1 },
});
const MEMBER = row({ id: "en.sitemaps", title: "What a sitemap does", cluster: "member" });

/** A suggestion of the post `id`, the cluster's pillar when `pillar`. */
function suggestion(id: string, title: string, pillar = false): LinkSuggestion {
  const shares = { tags: [], pillar, cluster: "internal-links", category: "seo" };
  return { id, title, path: `/blog/${id.slice(3)}/`, score: 7, shares, anchor: null };
}

const NEAR = suggestion("en.choosing-anchor-text", "Choosing anchor text");
const PILLAR = suggestion("en.internal-links", "Internal links, the guide", true);

/** The post `id` as the editor reads it, with one line of text. */
function post(id: string, title: string): AdminPost {
  return {
    id,
    slug: id.slice(3),
    title,
    status: "published",
    markdown: "One line.\n",
    updated_at: "2026-11-02T09:00:00Z",
    settings: {
      co_authors: [],
      featured: false,
      review_state: "ready",
      access: "public",
      password_protected: false,
      share_cards: {},
      aliases: [],
    },
  };
}

/** A blog with one orphan and one cluster member, which records every request. */
function blog() {
  return adminBlog({
    "GET /posts/": (request) => {
      const filter = new URL(request.url).searchParams.get("filter");
      return rowsPage(filter === "cluster:member" ? [MEMBER] : [ORPHAN]);
    },
    "GET /posts/en.anchor-words/link-suggestions/": () => json(200, { suggestions: [NEAR] }),
    "GET /posts/en.sitemaps/link-suggestions/": () => json(200, { suggestions: [PILLAR] }),
    "GET /posts/en.choosing-anchor-text/": () => json(200, { posts: [post(NEAR.id, NEAR.title)] }),
    "PUT /posts/en.choosing-anchor-text/": () => json(200, { posts: [post(NEAR.id, NEAR.title)] }),
    "GET /posts/en.sitemaps/": () => json(200, { posts: [post(MEMBER.id, MEMBER.title)] }),
    "PUT /posts/en.sitemaps/": () => json(200, { posts: [post(MEMBER.id, MEMBER.title)] }),
  });
}

describe("the SEO workouts", () => {
  it("links an orphan from the post closest to it", async () => {
    signedIn();
    const sent = blog();
    renderAt("/workouts");
    expect(await screen.findByRole("heading", { level: 1, name: words.title })).toBeVisible();
    const add = await screen.findByRole("button", {
      name: words.linkFromFor(NEAR.title, ORPHAN.title),
    });
    expect(screen.getByText(words.progress(0, 1))).toBeVisible();
    expect(await axeViolations()).toEqual([]);
    await userEvent.click(add);
    expect(await screen.findByText(words.linked(NEAR.title))).toBeVisible();
    expect(screen.getByText(words.progress(1, 1))).toBeVisible();
    const put = sent.find((request) => request.method === "PUT");
    expect(await put?.json()).toEqual({
      posts: [
        {
          updated_at: "2026-11-02T09:00:00Z",
          markdown: "One line.\n\nRelated: [Anchor words](/blog/anchor-words/).\n",
        },
      ],
    });
  });

  it("links each cluster member to its main post", async () => {
    signedIn();
    const sent = blog();
    renderAt("/workouts");
    await userEvent.click(await screen.findByRole("button", { name: words.tabs.cornerstone }));
    const add = await screen.findByRole("button", {
      name: words.linkToFor(MEMBER.title, PILLAR.title),
    });
    expect(await axeViolations()).toEqual([]);
    await userEvent.click(add);
    expect(await screen.findByText(words.linkedTo(PILLAR.title))).toBeVisible();
    const put = sent.find((request) => request.method === "PUT");
    expect(put?.url).toContain("/posts/en.sitemaps/");
    const asked = sent.map((request) => new URL(request.url).searchParams.get("filter"));
    expect(asked).toContain("cluster:member");
  });
});
