import { screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { type Answers, adminBlog } from "../../test/blog";
import { axeViolations, json, renderAt, signedIn } from "../../test/render";
import type { AdminCategory, NewCategory } from "../api/terms";
import { termWords } from "../strings/termWords";

const SEEN = "2026-11-10T09:00:00Z";

/** The demo site's On-page SEO, unless `fields` say otherwise. */
function category(fields: Partial<AdminCategory> = {}): AdminCategory {
  return {
    id: "on-page-seo",
    slug: "on-page-seo",
    names: { en: "On-page SEO", de: "Onpage-SEO" },
    posts: 4,
    updated_at: SEEN,
    ...fields,
  };
}

const STRATEGY = category({
  id: "content-strategy",
  slug: "content-strategy",
  names: { en: "Content strategy", de: "Content-Strategie" },
  posts: 0,
});

/** The demo site's settings: English first, then German. */
const SETTINGS = { settings: { default_language: "en", languages: ["en", "de"] } };

/** A blog that keeps `kept` and adds every category it is sent. */
function blog(kept: AdminCategory[], more: Answers = {}) {
  let list = [...kept];
  const made: NewCategory[] = [];
  adminBlog({
    "GET /tags/": () => json(200, { tags: [] }),
    "GET /settings/": () => json(200, SETTINGS),
    "GET /categories/": () => json(200, { categories: list }),
    "POST /categories/": async (request) => {
      const body: { categories: NewCategory[] } = await request.json();
      made.push(...body.categories);
      const added = body.categories.map((each) => category({ ...each, id: each.slug, posts: 0 }));
      list = [...list, ...added];
      return json(201, { categories: added });
    },
    ...more,
  });
  return made;
}

const WORDS = termWords.categories;
const ADD = termWords.addCategory;

describe("the categories on the Tags and categories screen", () => {
  it("lists each category by its name in the site's first language, then its others", async () => {
    signedIn();
    blog([category(), STRATEGY]);
    renderAt("/tags");
    const table = await screen.findByRole("table", { name: WORDS.title });
    const [, onPage, strategy] = within(table).getAllByRole("row");
    expect(onPage).toHaveTextContent("On-page SEO");
    expect(onPage).toHaveTextContent(WORDS.other("German", "Onpage-SEO"));
    expect(onPage).toHaveTextContent("4 posts");
    expect(strategy).toHaveTextContent("No posts");
    expect(await axeViolations()).toEqual([]);
  });

  it("adds a category named in each of the site's languages, leaving out an empty name", async () => {
    signedIn();
    const made = blog([category()]);
    renderAt("/tags");
    await userEvent.type(await screen.findByLabelText(ADD.slug), "link-building");
    await userEvent.type(await screen.findByLabelText(ADD.name("English")), " Link building ");
    expect(screen.getByLabelText(ADD.name("German"))).toHaveValue("");
    await userEvent.click(screen.getByRole("button", { name: ADD.submit }));
    expect(await screen.findByText(ADD.added("Link building"))).toBeVisible();
    expect(made).toEqual([{ slug: "link-building", names: { en: "Link building" } }]);
    const table = await screen.findByRole("table", { name: WORDS.title });
    expect(await within(table).findByText("link-building")).toBeVisible();
    expect(screen.getByLabelText(ADD.slug)).toHaveValue("");
  });

  it("shows the API's words when the short name is taken", async () => {
    signedIn();
    const taken = {
      type: "https://khalid.dev/problems/conflict",
      title: "Another category has this short name.",
      status: 409,
      detail: "Two categories cannot share one short name.",
      fix: "Pick another short name.",
    };
    blog([category()], { "POST /categories/": () => json(409, taken) });
    renderAt("/tags");
    await userEvent.type(await screen.findByLabelText(ADD.slug), "on-page-seo");
    await userEvent.type(screen.getByLabelText(ADD.name("English")), "On-page");
    await userEvent.click(screen.getByRole("button", { name: ADD.submit }));
    expect(await screen.findByRole("alert")).toHaveTextContent(taken.title);
    expect(screen.getByLabelText(ADD.slug)).toHaveValue("on-page-seo");
  });

  it("asks names in the categories' languages when the settings cannot be read", async () => {
    signedIn();
    blog([category()], { "GET /settings/": () => json(403, {}) });
    renderAt("/tags");
    await screen.findByRole("table", { name: WORDS.title });
    expect(screen.getByLabelText(ADD.name("English"))).toBeVisible();
    expect(screen.getByLabelText(ADD.name("German"))).toBeRequired();
  });
});
