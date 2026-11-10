import { fireEvent, screen, waitFor } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { item, mediaBlog } from "../../../test/media";
import { json, renderAt, serve, signedIn } from "../../../test/render";
import type { AdminPost } from "../../api/posts";
import { en } from "../../strings/en";

const TITLE = "Choosing anchor text that helps readers";

const POST: AdminPost = {
  id: "01JB",
  slug: "choosing-anchor-text",
  title: "Choosing anchor text",
  status: "draft",
  markdown: "Anchor text is the part of a link a reader clicks.\n",
  updated_at: "2026-11-02T10:42:00Z",
  settings: {
    co_authors: [],
    featured: false,
    review_state: "draft",
    access: "public",
    password_protected: false,
    tags: [],
    share_cards: {},
    aliases: [],
  },
};

const ON = { on: true, provider: "anthropic", label: "Anthropic", ready: true };

/** Options as the blog sends them, the second too long for a title. */
function offered(task: string, texts: string[]) {
  const suggestions = texts.map((text, at) => ({
    text,
    characters: text.length,
    fits: at === 0,
    note: at === 0 ? null : "Too long: 66 characters, and 60 is the most that shows in full.",
  }));
  return { task, label: "AI suggestion from Anthropic", suggestions, markdown: null };
}

/** A blog with one post and suggestions `ai`, recording each edit and each request. */
function blog(ai: unknown) {
  const edits: AdminPost[] = [];
  const asked: unknown[] = [];
  const paths: string[] = [];
  serve(async (request) => {
    const path = new URL(request.url).pathname;
    paths.push(path);
    if (path === "/api/admin/v1/ai/") {
      return json(200, ai);
    }
    if (path === "/api/admin/v1/ai/suggestions/") {
      asked.push(await request.json());
      return json(200, offered("seo_title", [TITLE, `${TITLE}, with every kind of link`]));
    }
    if (request.method === "PUT") {
      const body: { posts: AdminPost[] } = await request.json();
      edits.push(...body.posts);
    }
    return json(200, { posts: [POST] });
  });
  return { edits, asked, paths };
}

const SUGGEST_TITLE = "Suggest an SEO title with AI";

describe("AI suggestions in the editor", () => {
  it("shows no AI button while the owner has not turned suggestions on", async () => {
    signedIn();
    const { paths } = blog({ on: false, provider: "off", label: "Off", ready: false });
    renderAt("/posts/01JB");
    expect(await screen.findByLabelText(en.editor.title)).toHaveValue("Choosing anchor text");
    await waitFor(() => expect(paths).toContain("/api/admin/v1/ai/"));
    expect(screen.queryByRole("button", { name: SUGGEST_TITLE })).toBeNull();
  });

  it("fills the SEO title with the option the writer picks, and saves it", async () => {
    signedIn();
    const { edits, asked } = blog(ON);
    renderAt("/posts/01JB");
    fireEvent.click(await screen.findByRole("button", { name: SUGGEST_TITLE }));
    expect(await screen.findByText(TITLE)).toBeInTheDocument();
    expect(screen.getByText(/Too long: 66 characters/)).toBeInTheDocument();
    expect(asked[0]).toMatchObject({ task: "seo_title", post: "01JB" });
    fireEvent.click(screen.getAllByRole("button", { name: "Use this" })[0] as HTMLElement);
    expect(screen.getByLabelText(en.search.seoTitle)).toHaveValue(TITLE);
    await waitFor(() => expect(edits.at(-1)?.settings.seo_title).toBe(TITLE));
  });

  it("puts a Summarize button in the editor's bar", async () => {
    signedIn();
    blog(ON);
    renderAt("/posts/01JB");
    const name = "Summarize the post as key takeaways with AI";
    expect(await screen.findByRole("button", { name })).toBeInTheDocument();
  });
});

describe("AI suggestions in a file's details", () => {
  it("fills the alt text with the option the writer picks", async () => {
    signedIn();
    const stored = item();
    const sent = mediaBlog({
      "GET /ai/": () => json(200, ON),
      [`GET /media/${stored.id}/`]: () => json(200, { media: [stored] }),
      "POST /ai/suggestions/": () => json(200, offered("alt_text", ["A chart of page weight"])),
    });
    renderAt(`/media/${stored.id}`);
    const name = "Suggest alt text for this picture with AI";
    fireEvent.click(await screen.findByRole("button", { name }));
    fireEvent.click(await screen.findByRole("button", { name: "Use this" }));
    expect(screen.getByLabelText(en.media.details.alt)).toHaveValue("A chart of page weight");
    const request = sent.find((one) => one.url.endsWith("/ai/suggestions/"));
    expect(await request?.json()).toMatchObject({
      task: "alt_text",
      picture: { name: stored.name },
    });
  });
});
