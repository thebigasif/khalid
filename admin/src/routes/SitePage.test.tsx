import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { type Answers, adminBlog, noContent } from "../../test/blog";
import { axeViolations, json, renderAt, signedIn } from "../../test/render";
import type { AdminPage, PageEdit } from "../api/pages";
import { pagesWords } from "../strings/pagesWords";

const UEBER: AdminPage = {
  id: "de.ueber-uns",
  slug: "ueber-uns",
  language: "de",
  title: "Über Field Notes",
  description: "Who writes Field Notes.",
  markdown: "Field Notes is a small blog.\n",
  updated_at: "2026-10-30T09:00:00Z",
};

/** A blog with the German About page that records every edit, then answers with `more`. */
function blog(more: Answers = {}) {
  const edits: PageEdit[] = [];
  const sent = adminBlog({
    "GET /pages/": () => json(200, { pages: [UEBER] }),
    "GET /pages/de.ueber-uns/": () => json(200, { pages: [UEBER] }),
    "PUT /pages/de.ueber-uns/": async (request) => {
      const body: { pages: PageEdit[] } = await request.json();
      edits.push(...body.pages);
      return json(200, { pages: [{ ...UEBER, updated_at: "2026-10-31T10:00:00Z" }] });
    },
    ...more,
  });
  return { sent, edits };
}

const FIELDS = pagesWords.fields;
const EDIT = pagesWords.edit;

describe("a page's own screen", () => {
  it("opens from the list and saves only the changed fields", async () => {
    signedIn();
    const { edits } = blog();
    renderAt("/pages");
    const edit = pagesWords.list.editLabel("Über Field Notes", "German");
    await userEvent.click(await screen.findByRole("link", { name: edit }));
    expect(
      await screen.findByRole("heading", { level: 1, name: "Über Field Notes" }),
    ).toBeVisible();
    await userEvent.type(screen.getByLabelText(FIELDS.markdown.label), "Since 2026.");
    await userEvent.clear(screen.getByLabelText(FIELDS.description.label));
    await userEvent.click(screen.getByRole("button", { name: EDIT.save }));
    expect(await screen.findByText(EDIT.saved)).toBeVisible();
    expect(edits).toEqual([
      {
        updated_at: UEBER.updated_at,
        description: "",
        markdown: "Field Notes is a small blog.\nSince 2026.",
      },
    ]);
    expect(await axeViolations()).toEqual([]);
  });

  it("sends nothing when nothing changed", async () => {
    signedIn();
    const { edits } = blog();
    renderAt("/pages/de.ueber-uns");
    await userEvent.click(await screen.findByRole("button", { name: EDIT.save }));
    expect(await screen.findByText(EDIT.unchanged)).toBeVisible();
    expect(edits).toEqual([]);
  });

  it("shows the API's words when someone changed the page first", async () => {
    signedIn();
    const stale = {
      type: "https://khalid.dev/problems/conflict",
      title: "Someone else changed this page first.",
      status: 409,
      detail: "Saving now would undo their change.",
      fix: "Reload the page and make your change again.",
    };
    blog({ "PUT /pages/de.ueber-uns/": () => json(409, stale) });
    renderAt("/pages/de.ueber-uns");
    await userEvent.type(await screen.findByLabelText(FIELDS.title.label), "!");
    await userEvent.click(screen.getByRole("button", { name: EDIT.save }));
    expect(await screen.findByRole("alert")).toHaveTextContent(stale.title);
  });

  it("deletes the page with the copy's time, and goes back to the list", async () => {
    signedIn();
    const { sent } = blog({ "DELETE /pages/de.ueber-uns/": noContent });
    renderAt("/pages/de.ueber-uns");
    const remove = EDIT.remove.submit("Über Field Notes");
    await userEvent.click(await screen.findByRole("button", { name: remove }));
    expect(await screen.findByRole("heading", { level: 1, name: pagesWords.title })).toBeVisible();
    const deleted = sent.find((request) => request.method === "DELETE");
    expect(deleted?.url).toContain("updated_at=2026-10-30T09%3A00%3A00Z");
  });
});
