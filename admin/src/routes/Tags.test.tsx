import { screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { adminBlog, noContent } from "../../test/blog";
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

  it("renames a tag on the copy it was read from, then lists the new name", async () => {
    signedIn();
    let list = [tag()];
    const sent: unknown[] = [];
    adminBlog({
      "GET /tags/": () => json(200, { tags: list }),
      "PUT /tags/en.internal-links/": async (request) => {
        sent.push(await request.json());
        list = [tag({ id: "en.internal-linking", name: "internal linking" })];
        return json(200, { tags: list });
      },
    });
    renderAt("/tags");
    await userEvent.click(
      await screen.findByRole("button", { name: WORDS.renameLabel("internal links", "English") }),
    );
    const title = screen.getByRole("heading", {
      name: termWords.renameTag.title("internal links"),
    });
    expect(title).toHaveFocus();
    expect(screen.queryByRole("table")).toBeNull();
    expect(screen.getByText(termWords.renameTag.lede(3, "English"))).toBeVisible();
    expect(await axeViolations()).toEqual([]);
    const field = screen.getByLabelText(termWords.renameTag.label);
    await userEvent.clear(field);
    await userEvent.type(field, " internal linking ");
    await userEvent.click(screen.getByRole("button", { name: termWords.renameTag.submit(3) }));
    expect(
      await screen.findByText(termWords.renameTag.done("internal links", "internal linking")),
    ).toBeVisible();
    expect(sent).toEqual([{ tags: [{ name: "internal linking", updated_at: SEEN }] }]);
    const table = await screen.findByRole("table", { name: WORDS.title });
    expect(await within(table).findByText("internal linking")).toBeVisible();
  });

  it("removes a tag from every post, naming the copy it was read from", async () => {
    signedIn();
    let list = [tag(), GERMAN];
    const sent = adminBlog({
      "GET /tags/": () => json(200, { tags: list }),
      "DELETE /tags/de.interne-links/": () => {
        list = [tag()];
        return noContent();
      },
    });
    renderAt("/tags");
    await userEvent.click(
      await screen.findByRole("button", { name: WORDS.removeLabel("interne Links", "German") }),
    );
    expect(screen.getByText(termWords.removeTag.lede(1))).toBeVisible();
    await userEvent.click(screen.getByRole("button", { name: termWords.removeTag.submit(1) }));
    expect(await screen.findByText(termWords.removeTag.done("interne Links"))).toBeVisible();
    const removal = sent.find((request) => request.method === "DELETE");
    expect(new URL(removal?.url ?? "").searchParams.get("updated_at")).toBe(SEEN);
    const table = await screen.findByRole("table", { name: WORDS.title });
    expect(within(table).getAllByRole("row")).toHaveLength(2);
  });

  it("keeps the change open with the API's words when a post changed first", async () => {
    signedIn();
    const stale = {
      type: "https://khalid.dev/problems/conflict",
      title: "Someone changed a post with this tag first.",
      status: 409,
      detail: "Renaming now could undo their change.",
      fix: "Go back to the list and rename the tag again.",
    };
    adminBlog({
      "GET /tags/": () => json(200, { tags: [tag()] }),
      "PUT /tags/en.internal-links/": () => json(409, stale),
    });
    renderAt("/tags");
    await userEvent.click(
      await screen.findByRole("button", { name: WORDS.renameLabel("internal links", "English") }),
    );
    await userEvent.click(screen.getByRole("button", { name: termWords.renameTag.submit(3) }));
    expect(await screen.findByRole("alert")).toHaveTextContent(stale.title);
    expect(screen.getByLabelText(termWords.renameTag.label)).toBeVisible();
  });

  it("goes back to the list without changing anything", async () => {
    signedIn();
    const sent = adminBlog({ "GET /tags/": () => json(200, { tags: [tag()] }) });
    renderAt("/tags");
    await userEvent.click(
      await screen.findByRole("button", { name: WORDS.removeLabel("internal links", "English") }),
    );
    await userEvent.click(screen.getByRole("button", { name: termWords.back }));
    expect(await screen.findByRole("table", { name: WORDS.title })).toBeVisible();
    expect(sent.every((request) => request.method === "GET")).toBe(true);
  });

  it("says where tags come from when no post has one", async () => {
    signedIn();
    adminBlog({ "GET /tags/": () => json(200, { tags: [] }) });
    renderAt("/tags");
    expect(await screen.findByText(WORDS.none)).toBeVisible();
  });

  it("says when this server cannot list tags yet", async () => {
    signedIn();
    adminBlog({ "GET /categories/": () => json(200, { categories: [] }) });
    renderAt("/tags");
    expect(await screen.findByRole("alert")).toHaveTextContent(en.problems.notYet.what);
  });
});
