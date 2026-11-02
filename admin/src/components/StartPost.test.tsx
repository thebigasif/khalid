import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { json, renderAt, serve, signedIn } from "../../test/render";
import { listing, rowsPage } from "../../test/rows";
import { en } from "../strings/en";

const DRAFT = {
  id: "01JC",
  slug: "why-dough-sticks",
  title: "Why dough sticks",
  status: "draft",
  markdown: "",
  updated_at: "2026-10-27T11:00:00Z",
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

/** Whether `request` asks who the session cookie signs in. */
function isSession(request: Request): boolean {
  return new URL(request.url).pathname.endsWith("/session/");
}

describe("starting a post", () => {
  it("starts a draft with one idempotency key and opens it in the editor", async () => {
    signedIn();
    const sent: Request[] = [];
    serve((request) => {
      if (listing(request) !== null) {
        return rowsPage([]);
      }
      if (isSession(request)) {
        return json(401, {});
      }
      sent.push(request);
      return json(request.method === "POST" ? 201 : 200, { posts: [DRAFT] });
    });
    const { router } = renderAt("/");
    await userEvent.type(await screen.findByLabelText(en.start.label), "Why dough sticks");
    await userEvent.click(screen.getByRole("button", { name: en.start.submit }));
    expect(await screen.findByLabelText(en.editor.title)).toHaveValue("Why dough sticks");
    expect(router.state.location.pathname).toContain("/posts/01JC");
    expect(sent[0]?.method).toBe("POST");
    expect(sent[0]?.headers.get("Idempotency-Key")).toMatch(/^[0-9a-f]{32}$/);
    // Signed in with a token, nobody's author profile is known, so none is sent.
    expect(await sent[0]?.clone().json()).toEqual({
      posts: [{ title: "Why dough sticks", markdown: "" }],
    });
  });

  it("names the signed-in person's author profile as the new post's author", async () => {
    signedIn();
    const bodies: unknown[] = [];
    serve(async (request) => {
      if (listing(request) !== null) {
        return rowsPage([]);
      }
      if (isSession(request)) {
        const account = { id: 1, email: "lena@example.com", name: "Lena", role: "writer" };
        return json(200, { account: { ...account, author: "de.lena-vogt" }, csrf: "csrf-test" });
      }
      if (request.method === "POST") {
        bodies.push(await request.json());
      }
      return json(request.method === "POST" ? 201 : 200, { posts: [DRAFT] });
    });
    renderAt("/");
    await userEvent.type(await screen.findByLabelText(en.start.label), "Why dough sticks");
    await userEvent.click(screen.getByRole("button", { name: en.start.submit }));
    expect(await screen.findByLabelText(en.editor.title)).toHaveValue("Why dough sticks");
    expect(bodies).toEqual([
      { posts: [{ title: "Why dough sticks", markdown: "", author: "lena-vogt" }] },
    ]);
  });

  it("explains a draft the blog could not make", async () => {
    signedIn();
    serve((request) => {
      if (listing(request) !== null) {
        return rowsPage([]);
      }
      throw new TypeError("Failed to fetch");
    });
    renderAt("/");
    await userEvent.type(await screen.findByLabelText(en.start.label), "Why dough sticks");
    await userEvent.click(screen.getByRole("button", { name: en.start.submit }));
    expect(await screen.findByRole("alert")).toHaveTextContent(en.problems.unreachable.what);
  });
});
