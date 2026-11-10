import { screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { adminBlog } from "../../test/blog";
import { axeViolations, json, renderAt, signedIn } from "../../test/render";
import type { AdminTag } from "../api/terms";
import { en } from "../strings/en";
import { shellWords } from "../strings/shellWords";
import { termWords } from "../strings/termWords";

const SEEN = "2026-11-10T09:00:00Z";

/** The tag "internal links" in English, unless `fields` say otherwise. */
function tag(fields: Partial<AdminTag> = {}): AdminTag {
  return {
    id: "en.internal-links",
    slug: "internal-links",
    language: "en",
    name: "internal links",
    posts: 3,
    updated_at: SEEN,
    ...fields,
  };
}

const GERMAN = tag({
  id: "de.interne-links",
  slug: "interne-links",
  language: "de",
  name: "interne Links",
  posts: 1,
});

const WORDS = termWords.tags;

describe("the Tags and categories screen", () => {
  it("lists every tag with its language by name and how many posts have it", async () => {
    signedIn();
    adminBlog({ "GET /tags/": () => json(200, { tags: [tag(), GERMAN] }) });
    renderAt("/");
    await userEvent.click(await screen.findByRole("link", { name: shellWords.tags }));
    const table = await screen.findByRole("table", { name: WORDS.title });
    const [, english, german] = within(table).getAllByRole("row");
    expect(english).toHaveTextContent("English");
    expect(english).toHaveTextContent("3 posts");
    expect(german).toHaveTextContent("German");
    expect(german).toHaveTextContent("1 post");
    expect(screen.getAllByRole("heading", { level: 1 })).toHaveLength(1);
    expect(await axeViolations()).toEqual([]);
  });

  it("says where tags come from when no post has one", async () => {
    signedIn();
    adminBlog({ "GET /tags/": () => json(200, { tags: [] }) });
    renderAt("/tags");
    expect(await screen.findByText(WORDS.none)).toBeVisible();
  });

  it("says when this server cannot list tags yet", async () => {
    signedIn();
    adminBlog({});
    renderAt("/tags");
    expect(await screen.findByRole("alert")).toHaveTextContent(en.problems.notYet.what);
  });
});
