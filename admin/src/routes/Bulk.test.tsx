import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { redirectsBlog } from "../../test/redirects";
import { axeViolations, json, renderAt, signedIn } from "../../test/render";
import { row, rowsPage } from "../../test/rows";
import { en } from "../strings/en";

const LINKS = row();
const CLUSTER = row({
  id: "01JB",
  slug: "planning-a-topic-cluster",
  title: "Planning a topic cluster in an afternoon",
  author: "lena-vogt",
});

/** The problem a post someone else changed meanwhile gets. */
const STALE = {
  type: "https://khalid.dev/problems/conflict",
  title: "Someone else changed this first",
  status: 409,
  detail: "Lena changed this post at 2026-10-31 09:30 UTC, after the copy you edited.",
  fix: "Fetch the post again, make your change to it, and send its new updated_at.",
};

/** Ticks both posts on the page. */
async function tickBoth() {
  await userEvent.click(await screen.findByRole("checkbox", { name: en.bulk.tick(LINKS.title) }));
  await userEvent.click(screen.getByRole("checkbox", { name: en.bulk.tick(CLUSTER.title) }));
}

describe("changing many posts at once", () => {
  it("sets the author of each ticked post and says which was left", async () => {
    signedIn();
    const sent = redirectsBlog({
      "GET /posts/": () => rowsPage([LINKS, CLUSTER]),
      "PUT /posts/01JA/": () => json(200, { posts: [{ id: LINKS.id }] }),
      "PUT /posts/01JB/": () => json(409, STALE),
    });
    renderAt("/posts");
    await tickBoth();
    const form = screen.getByRole("form", { name: en.bulk.label });
    expect(within(form).getByText(en.bulk.ticked(2))).toBeVisible();
    expect(await axeViolations()).toEqual([]);
    await userEvent.selectOptions(within(form).getByLabelText(en.bulk.newAuthor), "lena-vogt");
    await userEvent.click(within(form).getByRole("button", { name: en.bulk.submit(2) }));
    expect(await screen.findByText(en.bulk.done(1))).toBeVisible();
    expect(screen.getByText(en.bulk.left(CLUSTER.title, STALE.title))).toBeVisible();
    const put = sent.find((request) => request.method === "PUT" && request.url.includes("01JA"));
    expect(await put?.json()).toEqual({
      posts: [{ updated_at: LINKS.updated_at, author: "lena-vogt" }],
    });
    expect(screen.queryByRole("form", { name: en.bulk.label })).toBeNull();
  });

  it("moves the ticked posts to the trash from the copy each row was read from", async () => {
    signedIn();
    const sent = redirectsBlog({
      "GET /posts/": () => rowsPage([LINKS, CLUSTER]),
      "DELETE /posts/01JA/": () => new Response(null, { status: 204 }),
      "DELETE /posts/01JB/": () => new Response(null, { status: 204 }),
    });
    renderAt("/posts");
    await tickBoth();
    const form = screen.getByRole("form", { name: en.bulk.label });
    await userEvent.selectOptions(within(form).getByLabelText(en.bulk.action), "trash");
    expect(within(form).queryByLabelText(en.bulk.newAuthor)).toBeNull();
    await userEvent.click(within(form).getByRole("button", { name: en.bulk.submit(2) }));
    expect(await screen.findByText(en.bulk.done(2))).toBeVisible();
    const deletes = sent.filter((request) => request.method === "DELETE");
    expect(deletes).toHaveLength(2);
    const seen = new URL(deletes[0]?.url ?? "").searchParams.get("updated_at");
    expect(seen).toBe(LINKS.updated_at);
  });

  it("says a trashed live post's address is gone for good unless told otherwise", async () => {
    signedIn();
    const live = row({ path: "/blog/why-internal-links-matter/" });
    const sent = redirectsBlog({
      "GET /posts/": () => rowsPage([live]),
      "DELETE /posts/01JA/": () => new Response(null, { status: 204 }),
      "POST /redirects/": () => json(201, { redirects: [] }),
    });
    renderAt("/posts");
    await userEvent.click(await screen.findByRole("checkbox", { name: en.bulk.tick(live.title) }));
    const form = screen.getByRole("form", { name: en.bulk.label });
    await userEvent.selectOptions(within(form).getByLabelText(en.bulk.action), "trash");
    const gone = within(form).getByRole("radio", { name: new RegExp(en.bulk.gone) });
    expect(gone).toBeChecked();
    expect(await axeViolations()).toEqual([]);
    await userEvent.click(within(form).getByRole("button", { name: en.bulk.submit(1) }));
    expect(await screen.findByText(en.bulk.done(1))).toBeVisible();
    const post = sent.find((request) => request.method === "POST");
    expect(await post?.json()).toEqual({
      redirects: [{ from: "/blog/why-internal-links-matter/", status: 410 }],
    });
  });

  it("sends a trashed live post's readers to the post picked instead", async () => {
    signedIn();
    const live = row({ path: "/blog/why-internal-links-matter/" });
    const other = row({ ...CLUSTER, path: "/blog/planning-a-topic-cluster/" });
    const sent = redirectsBlog({
      "GET /posts/": () => rowsPage([live, other]),
      "DELETE /posts/01JA/": () => new Response(null, { status: 204 }),
      "POST /redirects/": () => json(409, STALE),
    });
    renderAt("/posts");
    await userEvent.click(await screen.findByRole("checkbox", { name: en.bulk.tick(live.title) }));
    const form = screen.getByRole("form", { name: en.bulk.label });
    await userEvent.selectOptions(within(form).getByLabelText(en.bulk.action), "trash");
    await userEvent.click(within(form).getByRole("radio", { name: new RegExp(en.bulk.moved) }));
    const target = within(form).getByLabelText(en.bulk.target);
    expect(target).toHaveValue("/blog/planning-a-topic-cluster/");
    expect(within(target).queryByText(live.title)).toBeNull();
    await userEvent.click(within(form).getByRole("button", { name: en.bulk.submit(1) }));
    const said = en.bulk.noFarewell(live.title, "/blog/why-internal-links-matter/", STALE.title);
    expect(await screen.findByText(said)).toBeVisible();
    const post = sent.find((request) => request.method === "POST");
    expect(await post?.json()).toEqual({
      redirects: [
        {
          from: "/blog/why-internal-links-matter/",
          to: "/blog/planning-a-topic-cluster/",
          status: 301,
        },
      ],
    });
  });
});

describe("tagging and filing many posts at once", () => {
  const TAGGED = row({ ...CLUSTER, tags: ["Internal links", "clusters"] });

  it("adds a tag only to the posts that lack it, after the tags they have", async () => {
    signedIn();
    const sent = redirectsBlog({
      "GET /posts/": () => rowsPage([LINKS, TAGGED]),
      "PUT /posts/01JA/": () => json(200, { posts: [{ id: LINKS.id }] }),
    });
    renderAt("/posts");
    await tickBoth();
    const form = screen.getByRole("form", { name: en.bulk.label });
    await userEvent.selectOptions(within(form).getByLabelText(en.bulk.action), "addTag");
    await userEvent.type(within(form).getByLabelText(en.bulk.newTag), " internal links ");
    expect(await axeViolations()).toEqual([]);
    await userEvent.click(within(form).getByRole("button", { name: en.bulk.submit(2) }));
    expect(await screen.findByText(en.bulk.done(1))).toBeVisible();
    const puts = sent.filter((request) => request.method === "PUT");
    expect(puts).toHaveLength(1);
    expect(await puts[0]?.json()).toEqual({
      posts: [{ updated_at: LINKS.updated_at, settings: { tags: ["internal links"] } }],
    });
  });

  it("removes a tag the ticked posts have, whatever its capitals", async () => {
    signedIn();
    const sent = redirectsBlog({
      "GET /posts/": () => rowsPage([LINKS, TAGGED]),
      "PUT /posts/01JB/": () => json(200, { posts: [{ id: TAGGED.id }] }),
    });
    renderAt("/posts");
    await tickBoth();
    const form = screen.getByRole("form", { name: en.bulk.label });
    await userEvent.selectOptions(within(form).getByLabelText(en.bulk.action), "removeTag");
    expect(within(form).getByLabelText(en.bulk.oldTag)).toHaveValue("Internal links");
    await userEvent.click(within(form).getByRole("button", { name: en.bulk.submit(2) }));
    expect(await screen.findByText(en.bulk.done(1))).toBeVisible();
    const put = sent.find((request) => request.method === "PUT");
    expect(await put?.json()).toEqual({
      posts: [{ updated_at: TAGGED.updated_at, settings: { tags: ["clusters"] } }],
    });
  });

  it("files the ticked posts under a category the site has, or under none", async () => {
    signedIn();
    const category = {
      id: "on-page-seo",
      slug: "on-page-seo",
      names: { en: "On-page SEO" },
      posts: 4,
      updated_at: "2026-10-12T11:00:00Z",
    };
    const sent = redirectsBlog({
      "GET /posts/": () => rowsPage([LINKS, { ...TAGGED, category: "on-page-seo" }]),
      "GET /categories/": () => json(200, { categories: [category] }),
      "PUT /posts/01JA/": () => json(200, { posts: [{ id: LINKS.id }] }),
    });
    renderAt("/posts");
    await tickBoth();
    const form = screen.getByRole("form", { name: en.bulk.label });
    const action = within(form).getByLabelText(en.bulk.action);
    await waitFor(() => expect(within(action).getByText(en.bulk.category)).toBeVisible());
    await userEvent.selectOptions(action, "category");
    const picked = within(form).getByLabelText(en.bulk.newCategory);
    expect(within(picked).getByText(en.bulk.noCategory)).toBeVisible();
    expect(picked).toHaveValue("on-page-seo");
    await userEvent.click(within(form).getByRole("button", { name: en.bulk.submit(2) }));
    expect(await screen.findByText(en.bulk.done(1))).toBeVisible();
    const put = sent.find((request) => request.method === "PUT");
    expect(await put?.json()).toEqual({
      posts: [{ updated_at: LINKS.updated_at, settings: { category: "on-page-seo" } }],
    });
  });
});
