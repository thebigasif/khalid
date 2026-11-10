import { fireEvent, screen, waitFor, within } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { axeViolations, json, renderAt, serve, signedIn } from "../../test/render";
import type { DraftComment, NewDraftComment } from "../api/draftComments";
import type { AdminPost } from "../api/posts";
import { moment } from "../strings/dates";
import { draftCommentWords as words } from "../strings/draftCommentWords";

const DRAFT: AdminPost = {
  id: "01JB",
  slug: "why-internal-links-matter",
  title: "Why internal links matter",
  status: "draft",
  markdown: "Internal links help readers and Google find the rest.\n\nName the tool here.\n",
  updated_at: "2026-11-10T10:00:00Z",
  settings: {
    co_authors: [],
    featured: false,
    review_state: "draft",
    access: "public",
    password_protected: false,
    share_cards: {},
    aliases: [],
  },
};

const ASKED: DraftComment = {
  id: 4,
  parent: null,
  block: 2,
  author: "maria",
  text: "Can we name the tool here?",
  resolved: false,
  created_at: "2026-11-10T10:42:00Z",
};

const NONE_KEPT = {
  type: "https://khalid.dev/problems/invalid-body",
  title: "This site keeps no comments on drafts.",
  status: 422,
  detail: "Comments on drafts are kept by the site's server, in its database.",
  fix: "Send the request to the running khalid serve.",
};

/** A blog keeping `kept` on DRAFT, recording each comment and change it is sent. */
function blog(kept: DraftComment[], keeps = true) {
  const added: NewDraftComment[] = [];
  const resolved: boolean[] = [];
  serve(async (request) => {
    const path = new URL(request.url).pathname;
    if (!path.includes("/draft-comments/")) {
      return json(200, { posts: [DRAFT] });
    }
    if (request.method === "GET") {
      return json(200, { draft_comments: kept });
    }
    if (!keeps) {
      return json(422, NONE_KEPT);
    }
    if (request.method === "PUT") {
      const body: { draft_comments: { resolved: boolean }[] } = await request.json();
      const state = body.draft_comments[0]?.resolved ?? false;
      resolved.push(state);
      kept.splice(0, kept.length, ...kept.map((c) => (c.id === 4 ? { ...c, resolved: state } : c)));
      return new Response(null, { status: 204 });
    }
    const body: { draft_comments: NewDraftComment[] } = await request.json();
    const comment = body.draft_comments[0] ?? { text: "" };
    added.push(comment);
    const saved = { ...ASKED, ...comment, id: 10 + added.length, author: "asif" };
    kept.push(saved);
    return json(201, { draft_comments: [saved] });
  });
  return { added, resolved };
}

const showComments = async () => {
  renderAt("/posts/01JB");
  fireEvent.click(await screen.findByRole("button", { name: words.show }));
};

describe("comments on a draft", () => {
  it("says how many threads are open, then shows each with its part and author", async () => {
    signedIn();
    blog([ASKED]);
    renderAt("/posts/01JB");
    expect(await screen.findByText(words.open(1))).toBeVisible();
    fireEvent.click(screen.getByRole("button", { name: words.show }));
    const open = await screen.findByRole("list", { name: words.openList });
    expect(within(open).getByText(words.aboutPart(2, "Name the tool here."))).toBeVisible();
    expect(within(open).getByText(words.byline("maria", moment(ASKED.created_at)))).toBeVisible();
    expect(within(open).getByText(ASKED.text)).toBeVisible();
    expect(await axeViolations()).toEqual([]);
  });

  it("adds a comment about one part of the text", async () => {
    signedIn();
    const { added } = blog([]);
    await showComments();
    const form = await screen.findByRole("form", { name: words.newTitle });
    fireEvent.change(within(form).getByLabelText(words.about), { target: { value: "1" } });
    fireEvent.change(within(form).getByLabelText(words.comment), { target: { value: "Shorter?" } });
    fireEvent.click(within(form).getByRole("button", { name: words.add }));
    await waitFor(() => expect(added).toEqual([{ text: "Shorter?", block: 1 }]));
    const open = await screen.findByRole("list", { name: words.openList });
    expect(await within(open).findByText("Shorter?")).toBeVisible();
    expect(within(form).getByLabelText(words.comment)).toHaveValue("");
  });

  it("answers a thread", async () => {
    signedIn();
    const { added } = blog([ASKED]);
    await showComments();
    const open = await screen.findByRole("list", { name: words.openList });
    fireEvent.change(within(open).getByLabelText(words.answer), { target: { value: "Yes." } });
    fireEvent.click(within(open).getByRole("button", { name: words.sendAnswer }));
    await waitFor(() => expect(added).toEqual([{ text: "Yes.", parent: 4 }]));
    const answers = await within(open).findByRole("list", { name: words.answers });
    expect(within(answers).getByText("Yes.")).toBeVisible();
  });

  it("marks a thread done, and opens it again", async () => {
    signedIn();
    const { resolved } = blog([ASKED]);
    await showComments();
    fireEvent.click(await screen.findByRole("button", { name: words.done }));
    expect(await screen.findByText(words.noneOpen)).toBeVisible();
    fireEvent.click(screen.getByRole("button", { name: words.showDone(1) }));
    const done = screen.getByRole("list", { name: words.doneList });
    fireEvent.click(within(done).getByRole("button", { name: words.reopen }));
    expect(await screen.findByText(words.open(1))).toBeVisible();
    expect(resolved).toEqual([true, false]);
  });

  it("says why when the site keeps no comments, and keeps the text typed", async () => {
    signedIn();
    blog([], false);
    await showComments();
    const form = await screen.findByRole("form", { name: words.newTitle });
    fireEvent.change(within(form).getByLabelText(words.comment), { target: { value: "Hello" } });
    fireEvent.click(within(form).getByRole("button", { name: words.add }));
    expect(await screen.findByText(NONE_KEPT.title)).toBeVisible();
    expect(within(form).getByLabelText(words.comment)).toHaveValue("Hello");
  });
});
