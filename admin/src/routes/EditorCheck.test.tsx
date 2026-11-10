import { fireEvent, screen, waitFor } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { axeViolations, json, renderAt, serve, signedIn } from "../../test/render";
import type { PostCheck } from "../api/check";
import type { AdminPost } from "../api/posts";
import type { LinkSuggestion } from "../api/seo";
import { en } from "../strings/en";
import { seoEditorWords } from "../strings/seoEditorWords";

const POST: AdminPost = {
  id: "en.why-internal-links-matter",
  slug: "why-internal-links-matter",
  title: "Why internal links matter",
  status: "published",
  markdown: "Internal links help readers and search engines find the rest of your site.\n",
  updated_at: "2026-10-20T08:30:00Z",
  settings: {
    co_authors: [],
    featured: false,
    review_state: "ready",
    access: "public",
    password_protected: false,
    tags: [],
    share_cards: {},
    aliases: [],
  },
};

const NO_SOURCE = {
  rule: "no-external-link",
  severity: "warn",
  locale: "en",
  slug: "why-internal-links-matter",
  subject: null,
  what: "This post links to no other website.",
  why: "Links to good sources show readers and Google where your facts come from.",
  fix: "Add a link to a trusted source that backs up a claim in the post.",
};

const CHECK: PostCheck = {
  id: POST.id,
  lint: {
    blocks: false,
    findings: [NO_SOURCE],
    score: { value: 95, lost: [{ rule: "no-external-link", points: 5 }] },
  },
  scores: { seo: 95, readability: 60 },
  words: 640,
  snippet: {
    title: "Why internal links matter | Field Notes on SEO",
    url: "https://www.example.com/blog/why-internal-links-matter/",
    description: "Internal links help readers and search engines find the rest of your site.",
  },
  schema: { "@context": "https://schema.org", "@graph": [{ "@type": "BlogPosting" }] },
};

const EVENT = '{"@type": "Event", "name": "Internal links clinic"}';

/** A blog holding the demo post: its check is `check`, its link suggestions `suggestions`. */
function blog(check: () => Response, suggestions: unknown[] = []) {
  const edits: unknown[] = [];
  serve(async (request) => {
    if (request.url.endsWith("/check/")) {
      return check();
    }
    if (request.url.endsWith("/link-suggestions/")) {
      return json(200, { suggestions });
    }
    if (request.method === "PUT") {
      const body: { posts: unknown[] } = await request.json();
      edits.push(body.posts[0]);
    }
    return json(200, { posts: [POST] });
  });
  return edits;
}

describe("the editor's score and search result", () => {
  it("shows the score beside the checks, and the post as a Google result", async () => {
    signedIn();
    blog(() => json(200, { checks: [CHECK] }));
    renderAt(`/posts/${POST.id}`);
    expect(await screen.findByText(en.editor.score(95))).toBeVisible();
    expect(screen.getByText(NO_SOURCE.what)).toBeVisible();
    const result = screen.getByRole("region", { name: en.search.result });
    expect(result).toHaveTextContent("Why internal links matter | Field Notes on SEO");
    expect(result).toHaveTextContent("https://www.example.com › blog › why-internal-links-matter");
    expect(await axeViolations()).toEqual([]);
  });

  it("shows only the checks a publish brings back when the blog cannot check", async () => {
    signedIn();
    blog(() => json(404, { title: "This server does not check posts." }));
    renderAt(`/posts/${POST.id}`);
    expect(await screen.findByText(en.editor.noChecks)).toBeVisible();
    expect(screen.getByLabelText(en.search.focus)).toBeVisible();
    expect(screen.queryByRole("region", { name: en.search.result })).toBeNull();
  });
});

describe("the editor's two lights and the result on each device", () => {
  it("names each score's light in words, and counts the words", async () => {
    signedIn();
    blog(() => json(200, { checks: [CHECK] }));
    renderAt(`/posts/${POST.id}`);
    const lights = await screen.findByRole("list", { name: seoEditorWords.lights });
    expect(lights).toHaveTextContent(`${seoEditorWords.seo} ${seoEditorWords.outOf(95)}`);
    expect(lights).toHaveTextContent(seoEditorWords.state.good);
    expect(lights).toHaveTextContent(seoEditorWords.state.ok);
    expect(lights).toHaveTextContent(seoEditorWords.words(640));
  });

  it("bolds the focus keyword, and gives a long title two lines on a phone", async () => {
    signedIn();
    const title = "Why internal links matter, and how to add the right ones | Field Notes";
    const snippet = { ...CHECK.snippet, title };
    blog(() => json(200, { checks: [{ ...CHECK, snippet }] }));
    renderAt(`/posts/${POST.id}`);
    const result = await screen.findByRole("region", { name: en.search.result });
    fireEvent.change(screen.getByLabelText(en.search.focus), {
      target: { value: "internal links" },
    });
    expect(result.querySelector(".desc b")).toHaveTextContent("Internal links");
    // About 611 pixels wide at Google's desktop size, past its 600.
    expect(screen.getByText(seoEditorWords.titleCut)).toBeVisible();
    expect(result).toHaveTextContent("right ones | Field …");
    fireEvent.click(screen.getByRole("button", { name: seoEditorWords.mobile }));
    expect(screen.queryByText(seoEditorWords.titleCut)).toBeNull();
    expect(result).toHaveClass("mobile");
  });
});

describe("the editor's search fields and schema", () => {
  it("lists the schema the page carries and saves the writer's own", async () => {
    signedIn();
    const edits = blog(() => json(200, { checks: [CHECK] }));
    renderAt(`/posts/${POST.id}`);
    // The schema panel starts folded, so its parts are found by their words.
    expect(await screen.findByText("BlogPosting")).toBeInTheDocument();
    const test = screen.getByText(en.schema.test).closest("a");
    expect(test).toHaveAttribute("href", expect.stringContaining("why-internal-links-matter"));
    const own = screen.getByLabelText(en.schema.own);
    fireEvent.change(own, { target: { value: EVENT } });
    const focus = screen.getByLabelText(en.search.focus);
    fireEvent.change(focus, { target: { value: "internal links" } });
    fireEvent.blur(focus);
    await waitFor(() => expect(screen.getByRole("status")).toHaveTextContent(en.editor.saved));
    expect(edits).toEqual([
      {
        updated_at: POST.updated_at,
        title: POST.title,
        markdown: POST.markdown,
        settings: { focus_keyword: "internal links", schema: EVENT },
      },
    ]);
    expect(await axeViolations()).toEqual([]);
  });

  it("saves secondary keywords one per line, and the rarer robots choices", async () => {
    signedIn();
    const edits = blog(() => json(200, { checks: [CHECK] }));
    renderAt(`/posts/${POST.id}`);
    const secondary = await screen.findByLabelText(seoEditorWords.secondary);
    fireEvent.change(secondary, { target: { value: "link text\n\nsite structure" } });
    fireEvent.click(screen.getByText(en.settings.more.title));
    fireEvent.click(screen.getByText(seoEditorWords.advanced));
    fireEvent.click(screen.getByLabelText(seoEditorWords.restrictions.noarchive));
    const size = screen.getByLabelText(seoEditorWords.maxImage);
    fireEvent.change(size, { target: { value: "standard" } });
    fireEvent.blur(size);
    await waitFor(() => expect(edits).toHaveLength(1));
    expect(edits[0]).toMatchObject({
      settings: {
        secondary_keywords: ["link text", "site structure"],
        robots: "noarchive, max-image-preview:standard",
      },
    });
  });

  it("says why a post is suggested, and links the words in the text that name it", async () => {
    signedIn();
    const sitemaps: LinkSuggestion = {
      id: "en.what-a-sitemap-does",
      title: "What a sitemap does",
      path: "/blog/what-a-sitemap-does/",
      score: 6,
      shares: { tags: [], pillar: false, keyword: "search engines" },
      anchor: "search engines",
    };
    const edits = blog(() => json(200, { checks: [CHECK] }), [sitemaps]);
    renderAt(`/posts/${POST.id}`);
    fireEvent.click(await screen.findByText(seoEditorWords.links));
    expect(screen.getByText(seoEditorWords.why.keyword("search engines"))).toBeVisible();
    fireEvent.click(screen.getByRole("button", { name: seoEditorWords.addLink(sitemaps.title) }));
    expect(screen.getByText(seoEditorWords.added)).toBeVisible();
    fireEvent.blur(screen.getByLabelText(en.editor.title));
    await waitFor(() => expect(edits).toHaveLength(1));
    const linked = "[search engines](/blog/what-a-sitemap-does/)";
    expect(edits[0]).toMatchObject({ markdown: expect.stringContaining(linked) });
    expect(await axeViolations()).toEqual([]);
  });

  it("shows the post shared on Facebook and X, with a card's own title", async () => {
    signedIn();
    blog(() => json(200, { checks: [CHECK] }));
    renderAt(`/posts/${POST.id}`);
    fireEvent.click(await screen.findByText(seoEditorWords.share));
    const facebook = screen.getByText(seoEditorWords.facebook).closest("figure");
    // The site's name comes with the check, which may answer after the panel opens.
    await waitFor(() => expect(facebook).toHaveTextContent("WWW.EXAMPLE.COM"));
    expect(facebook).toHaveTextContent(POST.title);
    expect(facebook).toHaveTextContent(seoEditorWords.noImage);
    fireEvent.click(screen.getByText(en.settings.more.title));
    const fields = en.settings.more.fields;
    fireEvent.change(screen.getByLabelText(fields.xTitle.label), {
      target: { value: "Internal links, plainly" },
    });
    const x = screen.getByText(seoEditorWords.x).closest("figure");
    expect(x).toHaveTextContent("Internal links, plainly");
    expect(facebook).toHaveTextContent(POST.title);
    expect(await axeViolations()).toEqual([]);
  });
});
