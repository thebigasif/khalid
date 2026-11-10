import { screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { adminBlog } from "../../test/blog";
import { axeViolations, json, renderAt, signedIn } from "../../test/render";
import type { AdminPage, NewPage } from "../api/pages";
import { en } from "../strings/en";
import { pagesWords } from "../strings/pagesWords";

/** The demo site's About page in English, unless `fields` say otherwise. */
function page(fields: Partial<AdminPage> = {}): AdminPage {
  return {
    id: "en.about",
    slug: "about",
    language: "en",
    title: "About Field Notes",
    description: "Who writes Field Notes on SEO, and why.",
    markdown: "Field Notes is a small blog about search.\n",
    updated_at: "2026-10-30T09:00:00Z",
    ...fields,
  };
}

const UEBER = page({
  id: "de.ueber-uns",
  slug: "ueber-uns",
  language: "de",
  title: "Über Field Notes",
});

/** A blog that keeps `kept` and adds every page it is sent. */
function blog(kept: AdminPage[]) {
  let list = [...kept];
  const made: NewPage[] = [];
  adminBlog({
    "GET /pages/": () => json(200, { pages: list }),
    "POST /pages/": async (request) => {
      const body: { pages: NewPage[] } = await request.json();
      made.push(...body.pages);
      const [{ title, slug } = page()] = body.pages;
      const added = page({ id: `en.${slug}`, title, slug, description: null });
      list = [...list, added];
      return json(201, { pages: [added] });
    },
  });
  return made;
}

const ADD = pagesWords.fields;

describe("the Pages screen", () => {
  it("opens from the menu and lists every page with its language by name", async () => {
    signedIn();
    blog([page(), UEBER]);
    renderAt("/");
    await userEvent.click(await screen.findByRole("link", { name: pagesWords.nav }));
    const table = await screen.findByRole("table", { name: pagesWords.list.title });
    const [, about, ueber] = within(table).getAllByRole("row");
    expect(about).toHaveTextContent("About Field Notes");
    expect(about).toHaveTextContent("English");
    expect(ueber).toHaveTextContent("German");
    expect(screen.getAllByRole("heading", { level: 1 })).toHaveLength(1);
    expect(await axeViolations()).toEqual([]);
  });

  it("adds a page, leaving out an empty language and description", async () => {
    signedIn();
    const made = blog([page()]);
    renderAt("/pages");
    await userEvent.type(await screen.findByLabelText(ADD.title.label), "Contact");
    await userEvent.type(screen.getByLabelText(ADD.slug.label), "contact");
    await userEvent.type(screen.getByLabelText(ADD.markdown.label), "Write to us.");
    await userEvent.click(screen.getByRole("button", { name: pagesWords.add.submit }));
    expect(await screen.findByText(pagesWords.add.added("Contact"))).toBeVisible();
    const contact = { title: "Contact", slug: "contact", markdown: "Write to us." };
    expect(made).toEqual([{ ...contact, language: null, description: null }]);
    const table = await screen.findByRole("table", { name: pagesWords.list.title });
    expect(await within(table).findByText("contact")).toBeVisible();
  });

  it("says what went wrong when the address is taken", async () => {
    signedIn();
    const taken = {
      type: "https://khalid.dev/problems/conflict",
      title: "Another page in that language has this address.",
      status: 409,
      detail: "Two pages cannot share one address.",
      fix: "Pick another address.",
    };
    adminBlog({
      "GET /pages/": () => json(200, { pages: [page()] }),
      "POST /pages/": () => json(409, taken),
    });
    renderAt("/pages");
    await userEvent.type(await screen.findByLabelText(ADD.title.label), "About us");
    await userEvent.type(screen.getByLabelText(ADD.slug.label), "about");
    await userEvent.click(screen.getByRole("button", { name: pagesWords.add.submit }));
    expect(await screen.findByRole("alert")).toHaveTextContent(taken.title);
    expect(screen.getByLabelText(ADD.title.label)).toHaveValue("About us");
  });

  it("says when this server cannot keep pages yet", async () => {
    signedIn();
    adminBlog({});
    renderAt("/pages");
    expect(await screen.findByRole("alert")).toHaveTextContent(en.problems.notYet.what);
  });
});
