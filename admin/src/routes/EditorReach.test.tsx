import { fireEvent, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { axeViolations, json, renderAt, serve, signedIn } from "../../test/render";
import type { AdminPost } from "../api/posts";
import { en } from "../strings/en";
import { reachWords } from "../strings/reachWords";

const POST: AdminPost = {
  id: "01JB",
  slug: "planning-a-topic-cluster",
  title: "Planning a topic cluster",
  status: "draft",
  markdown: "A topic cluster is a group of posts.\n",
  updated_at: "2026-10-27T10:42:00Z",
  settings: {
    co_authors: [],
    featured: false,
    review_state: "draft",
    access: "public",
    password_protected: true,
    tags: ["seo"],
    share_cards: {},
    connections: { main_post: false, related: [] },
    listing: { keyword_synonyms: [], reviewed_by: [], sitemap: true },
    aliases: [],
  },
};

/** A blog with one post and two categories, which records every edit it is sent. */
function blog() {
  const edits: { settings?: object }[] = [];
  serve(async (request) => {
    if (request.url.includes("/categories/")) {
      return json(200, {
        categories: [
          { slug: "on-page-seo", names: { en: "On-page SEO" } },
          { slug: "technical-seo", names: { en: "Technical SEO" } },
        ],
      });
    }
    if (request.method !== "PUT") {
      return json(200, { posts: [POST] });
    }
    const body: { posts: { settings?: object }[] } = await request.json();
    edits.push(...body.posts);
    return json(200, { posts: [{ ...POST, updated_at: "2026-10-27T10:44:00Z" }] });
  });
  return edits;
}

/** Every setting the edits sent: each blur saves what changed since the last save. */
function sentSettings(edits: { settings?: object }[]): object {
  return Object.assign({}, ...edits.map((edit) => edit.settings ?? {}));
}

describe("filing a post and choosing who reads it in the editor", () => {
  it("saves the tags, the category and the featured switch", async () => {
    signedIn();
    const edits = blog();
    renderAt("/posts/01JB");
    const tags = await screen.findByLabelText(reachWords.tags);
    expect(tags).toHaveValue("seo");
    await screen.findByRole("option", { name: "On-page SEO" });
    expect(await axeViolations()).toEqual([]);
    fireEvent.change(tags, { target: { value: "seo, topic clusters" } });
    await userEvent.selectOptions(screen.getByLabelText(reachWords.category), "on-page-seo");
    await userEvent.click(screen.getByLabelText(reachWords.featured));
    fireEvent.blur(screen.getByLabelText(reachWords.featured));
    await waitFor(() => expect(screen.getByRole("status")).toHaveTextContent(en.editor.saved));
    await waitFor(() =>
      expect(sentSettings(edits)).toEqual({
        tags: ["seo", "topic clusters"],
        category: "on-page-seo",
        featured: true,
      }),
    );
  });

  it("opens the post to everyone by turning its password off, and limits it to members", async () => {
    signedIn();
    const edits = blog();
    renderAt("/posts/01JB");
    await userEvent.click(await screen.findByText(reachWords.title));
    expect(screen.getByLabelText(reachWords.password)).toHaveValue("");
    await userEvent.click(screen.getByLabelText(reachWords.passwordOn));
    expect(screen.queryByLabelText(reachWords.password)).toBeNull();
    const access = screen.getByLabelText(reachWords.access);
    await userEvent.selectOptions(access, "members");
    fireEvent.blur(access);
    await waitFor(() => expect(screen.getByRole("status")).toHaveTextContent(en.editor.saved));
    await waitFor(() => expect(sentSettings(edits)).toEqual({ password: null, access: "members" }));
  });
});
