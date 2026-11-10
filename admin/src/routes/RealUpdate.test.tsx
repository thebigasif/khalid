import { fireEvent, screen, waitFor } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { axeViolations, json, renderAt, serve, signedIn } from "../../test/render";
import type { AdminPost, PostEdit } from "../api/posts";
import { en } from "../strings/en";

const words = en.realUpdate;

const LIVE: AdminPost = {
  id: "01JB",
  slug: "planning-a-topic-cluster",
  title: "Planning a topic cluster",
  status: "published",
  markdown: "A topic cluster is one main post and the posts that link to it.\n",
  updated_at: "2026-10-27T10:42:00Z",
  updated: "2026-10-31",
  updated_note: "Added a worked example with five posts.",
  settings: {
    co_authors: [],
    featured: false,
    review_state: "ready",
    access: "public",
    password_protected: false,
    share_cards: {},
    aliases: [],
    tags: [],
  },
};

/** A blog holding `post`, which dates an edit marked real_update today, as the API does. */
function blog(post: AdminPost) {
  const edits: PostEdit[] = [];
  serve(async (request) => {
    if (request.method !== "PUT") {
      return json(200, { posts: [post] });
    }
    const body: { posts: PostEdit[] } = await request.json();
    const edit = body.posts[0] ?? { updated_at: "" };
    edits.push(edit);
    const note = edit.real_update?.note;
    const dated = note === undefined ? {} : { updated: "2026-11-10", updated_note: note };
    return json(200, { posts: [{ ...post, ...dated, updated_at: "2026-11-10T09:00:00Z" }] });
  });
  return edits;
}

/** Changes the post's text in the markdown view and leaves the field, which saves. */
async function changeText(value: string) {
  fireEvent.click(await screen.findByRole("button", { name: en.editor.markdown }));
  const text = screen.getByRole("textbox", { name: en.editor.text });
  fireEvent.change(text, { target: { value } });
  fireEvent.blur(text);
}

describe("the honest update switch", () => {
  it("dates a live post today only when the writer marks a real update", async () => {
    signedIn();
    const edits = blog(LIVE);
    renderAt("/posts/01JB");
    const last = words.last("31 October 2026", LIVE.updated_note ?? "");
    expect(await screen.findByText(last)).toBeVisible();
    fireEvent.click(screen.getByRole("checkbox", { name: words.on }));
    const note = screen.getByRole("textbox", { name: words.note });
    fireEvent.change(note, { target: { value: "Added a sheet to plan the links on." } });
    expect(await axeViolations()).toEqual([]);
    await changeText(`${LIVE.markdown}\nPlan the links on one sheet.\n`);
    await waitFor(() => expect(edits).toHaveLength(1));
    expect(edits[0]?.real_update).toEqual({ note: "Added a sheet to plan the links on." });
    expect(
      await screen.findByText(
        words.last("10 November 2026", "Added a sheet to plan the links on."),
      ),
    ).toBeVisible();
    expect(screen.getByRole("checkbox", { name: words.on })).not.toBeChecked();
  });

  it("leaves the date alone for an edit not marked", async () => {
    signedIn();
    const edits = blog(LIVE);
    renderAt("/posts/01JB");
    await changeText("Fixed a typo.\n");
    await waitFor(() => expect(edits).toHaveLength(1));
    expect(edits[0]).not.toHaveProperty("real_update");
  });

  it("keeps the switch on for the text when only a setting changed", async () => {
    signedIn();
    const edits = blog(LIVE);
    renderAt("/posts/01JB");
    fireEvent.click(await screen.findByRole("checkbox", { name: words.on }));
    fireEvent.change(screen.getByRole("textbox", { name: words.note }), {
      target: { value: "Rewrote the opening." },
    });
    const title = screen.getByLabelText(en.editor.title);
    fireEvent.change(title, { target: { value: "Planning a topic cluster in an afternoon" } });
    fireEvent.blur(title);
    await waitFor(() => expect(edits).toHaveLength(1));
    expect(edits[0]).not.toHaveProperty("real_update");
    expect(screen.getByRole("checkbox", { name: words.on })).toBeChecked();
  });

  it("says why an empty or long note leaves the date as it is", async () => {
    signedIn();
    const edits = blog(LIVE);
    renderAt("/posts/01JB");
    fireEvent.click(await screen.findByRole("checkbox", { name: words.on }));
    expect(screen.getByText(words.empty)).toBeVisible();
    fireEvent.change(screen.getByRole("textbox", { name: words.note }), {
      target: { value: "x".repeat(161) },
    });
    expect(screen.getByText(words.tooLong)).toBeVisible();
    await changeText("A longer post.\n");
    await waitFor(() => expect(edits).toHaveLength(1));
    expect(edits[0]).not.toHaveProperty("real_update");
  });

  it("is not offered before the post goes live", async () => {
    signedIn();
    blog({ ...LIVE, status: "draft", updated: null, updated_note: null });
    renderAt("/posts/01JB");
    await screen.findByRole("button", { name: en.editor.markdown });
    expect(screen.queryByRole("checkbox", { name: words.on })).toBeNull();
  });
});
