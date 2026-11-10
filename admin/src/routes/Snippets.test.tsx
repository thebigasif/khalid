import { screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { adminBlog, noContent } from "../../test/blog";
import { axeViolations, json, renderAt, signedIn } from "../../test/render";
import type { Snippet } from "../api/snippets";
import { en } from "../strings/en";
import { shellWords } from "../strings/shellWords";
import { snippetWords } from "../strings/snippetWords";

const SEEN = "2026-11-03T09:00:00Z";

/** The English affiliate disclosure, unless `fields` say otherwise. */
function snippet(fields: Partial<Snippet> = {}): Snippet {
  return {
    id: "en.affiliate-disclosure",
    slug: "affiliate-disclosure",
    language: "en",
    title: "Affiliate disclosure",
    markdown: "Some links on this page are affiliate links.",
    updated_at: SEEN,
    ...fields,
  };
}

const GERMAN = snippet({
  id: "de.hinweis",
  slug: "hinweis",
  language: "de",
  title: "Hinweis",
  markdown: "Ein Hinweis zu Links.",
});

const WORDS = snippetWords.list;

describe("the Snippets screen", () => {
  it("lists every snippet with its language by name and the day it last changed", async () => {
    signedIn();
    adminBlog({ "GET /snippets/": () => json(200, { snippets: [snippet(), GERMAN] }) });
    renderAt("/");
    await userEvent.click(await screen.findByRole("link", { name: shellWords.snippets }));
    const table = await screen.findByRole("table", { name: WORDS.title });
    const [, english, german] = within(table).getAllByRole("row");
    expect(english).toHaveTextContent("Affiliate disclosure");
    expect(english).toHaveTextContent("English");
    expect(english).toHaveTextContent("3 November 2026");
    expect(german).toHaveTextContent("German");
    expect(screen.getAllByRole("heading", { level: 1 })).toHaveLength(1);
    expect(await axeViolations()).toEqual([]);
  });

  it("saves a new name and blocks on the copy it was read from", async () => {
    signedIn();
    let list = [snippet()];
    const sent: unknown[] = [];
    adminBlog({
      "GET /snippets/": () => json(200, { snippets: list }),
      "PUT /snippets/en.affiliate-disclosure/": async (request) => {
        sent.push(await request.json());
        list = [snippet({ title: "Disclosure", markdown: "We may earn a fee." })];
        return json(200, { snippets: list });
      },
    });
    renderAt("/snippets");
    await userEvent.click(
      await screen.findByRole("button", {
        name: WORDS.editLabel("Affiliate disclosure", "English"),
      }),
    );
    const title = screen.getByRole("heading", {
      name: snippetWords.edit.title("Affiliate disclosure"),
    });
    expect(title).toHaveFocus();
    expect(screen.queryByRole("table")).toBeNull();
    expect(screen.getByText(snippetWords.edit.lede("English"))).toBeVisible();
    expect(await axeViolations()).toEqual([]);
    const name = screen.getByLabelText(snippetWords.edit.name);
    await userEvent.clear(name);
    await userEvent.type(name, " Disclosure ");
    const blocks = screen.getByLabelText(snippetWords.edit.blocks);
    await userEvent.clear(blocks);
    await userEvent.type(blocks, "We may earn a fee.");
    await userEvent.click(screen.getByRole("button", { name: snippetWords.edit.submit }));
    expect(await screen.findByText(snippetWords.edit.done("Disclosure"))).toBeVisible();
    expect(sent).toEqual([
      { snippets: [{ title: "Disclosure", markdown: "We may earn a fee.", updated_at: SEEN }] },
    ]);
    const table = await screen.findByRole("table", { name: WORDS.title });
    expect(await within(table).findByText("Disclosure")).toBeVisible();
  });

  it("deletes a snippet, naming the copy it was read from", async () => {
    signedIn();
    let list = [snippet(), GERMAN];
    const sent = adminBlog({
      "GET /snippets/": () => json(200, { snippets: list }),
      "DELETE /snippets/de.hinweis/": () => {
        list = [snippet()];
        return noContent();
      },
    });
    renderAt("/snippets");
    await userEvent.click(
      await screen.findByRole("button", { name: WORDS.removeLabel("Hinweis", "German") }),
    );
    expect(screen.getByText(snippetWords.remove.lede)).toBeVisible();
    await userEvent.click(screen.getByRole("button", { name: snippetWords.remove.submit }));
    expect(await screen.findByText(snippetWords.remove.done("Hinweis"))).toBeVisible();
    const removal = sent.find((request) => request.method === "DELETE");
    expect(new URL(removal?.url ?? "").searchParams.get("updated_at")).toBe(SEEN);
    const table = await screen.findByRole("table", { name: WORDS.title });
    expect(within(table).getAllByRole("row")).toHaveLength(2);
  });

  it("keeps the edit open with the API's words when someone saved the snippet first", async () => {
    signedIn();
    const stale = {
      type: "https://khalid.dev/problems/conflict",
      title: "Someone changed this snippet first.",
      status: 409,
      detail: "Saving now could undo their change.",
      fix: "Go back to the list and edit the snippet again.",
    };
    adminBlog({
      "GET /snippets/": () => json(200, { snippets: [snippet()] }),
      "PUT /snippets/en.affiliate-disclosure/": () => json(409, stale),
    });
    renderAt("/snippets");
    await userEvent.click(
      await screen.findByRole("button", {
        name: WORDS.editLabel("Affiliate disclosure", "English"),
      }),
    );
    await userEvent.click(screen.getByRole("button", { name: snippetWords.edit.submit }));
    expect(await screen.findByRole("alert")).toHaveTextContent(stale.title);
    expect(screen.getByLabelText(snippetWords.edit.blocks)).toBeVisible();
  });

  it("goes back to the list without changing anything", async () => {
    signedIn();
    const sent = adminBlog({ "GET /snippets/": () => json(200, { snippets: [snippet()] }) });
    renderAt("/snippets");
    await userEvent.click(
      await screen.findByRole("button", {
        name: WORDS.removeLabel("Affiliate disclosure", "English"),
      }),
    );
    await userEvent.click(screen.getByRole("button", { name: snippetWords.back }));
    expect(await screen.findByRole("table", { name: WORDS.title })).toBeVisible();
    expect(sent.every((request) => request.method === "GET")).toBe(true);
  });

  it("says how to make a snippet when the site has none", async () => {
    signedIn();
    adminBlog({ "GET /snippets/": () => json(200, { snippets: [] }) });
    renderAt("/snippets");
    expect(await screen.findByText(en.snippets.none)).toBeVisible();
  });

  it("says when this server cannot list snippets yet", async () => {
    signedIn();
    adminBlog({});
    renderAt("/snippets");
    expect(await screen.findByRole("alert")).toHaveTextContent(en.problems.notYet.what);
  });
});
