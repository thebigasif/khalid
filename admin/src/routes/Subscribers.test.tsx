import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { adminBlog, noContent } from "../../test/blog";
import { axeViolations, json, renderAt, signedIn } from "../../test/render";
import { row, rowsPage } from "../../test/rows";
import type { AdminSubscriber, Newsletters } from "../api/subscribers";
import { subscriberWords } from "../strings/subscriberWords";

const words = subscriberWords;

/** A reader on main who confirmed, unless `fields` say otherwise. */
function reader(fields: Partial<AdminSubscriber> = {}): AdminSubscriber {
  return {
    id: 1,
    email: "lena@example.com",
    status: "confirmed",
    lists: ["main", "weekly"],
    labels: [],
    created_at: "2026-11-01T09:00:00Z",
    confirmed_at: "2026-11-01T09:05:00Z",
    unsubscribed_at: null,
    ...fields,
  };
}

const OMAR = reader({ id: 2, email: "omar@example.com", status: "pending", lists: ["main"] });

/** Two newsletters, one email sent, and the site ready to send unless told otherwise. */
function newsletters(sending: Newsletters["sending"] = "ready"): Newsletters {
  return {
    newsletters: [
      { name: "main", confirmed: 212, pending: 4, form: '<form class="khalid-subscribe"></form>' },
      { name: "weekly", confirmed: 58, pending: 0, form: "<form></form>" },
    ],
    sending,
    sends: [
      {
        key: "post:en/how-to-write-a-title-tag",
        list: "main",
        started_at: "2026-11-02T08:00:00Z",
        sent: 209,
        failed: 1,
        unsubscribed: 2,
      },
    ],
  };
}

/** A blog with `kept` readers that records what the screen sends. */
function blog(kept: AdminSubscriber[], sending: Newsletters["sending"] = "ready") {
  let list = [...kept];
  const bodies: unknown[] = [];
  const page = (readers: AdminSubscriber[]) => {
    const pagination = {
      page: 1,
      limit: 50,
      pages: 1,
      total: readers.length,
      next: null,
      prev: null,
    };
    return json(200, { subscribers: readers, meta: { pagination } });
  };
  const sent = adminBlog({
    "GET /newsletters/": () => json(200, newsletters(sending)),
    "GET /subscribers/": (request) => {
      const status = new URL(request.url).searchParams.get("status");
      return page(list.filter((each) => status === null || each.status === status));
    },
    "GET /posts/": () =>
      rowsPage([row({ id: "en.what-a-sitemap-does", title: "What a sitemap does" })]),
    "POST /newsletters/main/sends/": async (request) => {
      bodies.push(await request.json());
      return json(202, { key: "post:en/what-a-sitemap-does" });
    },
    "PUT /subscribers/1/labels/paid/": () => {
      list = list.map((each) => (each.id === 1 ? { ...each, labels: ["paid"] } : each));
      return json(200, { subscribers: list.filter((each) => each.id === 1) });
    },
    "DELETE /subscribers/2/": () => {
      list = list.filter((each) => each.id !== 2);
      return noContent();
    },
  });
  return { sent, bodies };
}

describe("the Subscribers screen", () => {
  it("shows the newsletters, what went out and the readers", async () => {
    signedIn();
    blog([reader(), OMAR]);
    renderAt("/");
    await userEvent.click(await screen.findByRole("link", { name: words.nav }));
    expect(await screen.findByRole("heading", { level: 1, name: words.title })).toBeVisible();
    expect(await screen.findByText("212")).toBeVisible();
    expect(screen.getByText(/4 more have not opened/)).toBeVisible();
    expect(screen.getByRole("rowheader", { name: "en/how-to-write-a-title-tag" })).toBeVisible();
    expect(await screen.findByRole("button", { name: words.send.submit(212) })).toBeVisible();
    expect(await axeViolations()).toEqual([]);
  });

  it("sends a post to the labelled readers of a newsletter", async () => {
    signedIn();
    const { bodies } = blog([reader()]);
    renderAt("/subscribers");
    const post = await screen.findByRole("combobox", { name: words.send.post });
    await userEvent.selectOptions(post, "en.what-a-sitemap-does");
    await userEvent.type(screen.getByLabelText(words.send.labels), "paid, Paid, early");
    await userEvent.click(screen.getByRole("button", { name: words.send.submitLabelled }));
    expect(await screen.findByText(words.send.started("What a sitemap does"))).toBeVisible();
    expect(bodies).toEqual([{ post: "en.what-a-sitemap-does", labels: ["paid", "early"] }]);
  });

  it("says what the site lacks before it can send, and offers no form", async () => {
    signedIn();
    blog([reader()], "no_postal_address");
    renderAt("/subscribers");
    const said = words.send.notReady.no_postal_address.what;
    expect(await screen.findByText(said)).toBeVisible();
    expect(screen.queryByRole("combobox", { name: words.send.post })).toBeNull();
  });

  it("shows the API's words when it refuses", async () => {
    signedIn();
    const lacks = {
      type: "https://khalid.dev/problems/forbidden",
      title: "This key does not have the subscribers:write scope.",
      status: 403,
      detail: "Each key may do only what it was made for.",
      fix: "Use a key with subscribers:write.",
    };
    adminBlog({ "GET /newsletters/": () => json(403, lacks) });
    renderAt("/subscribers");
    expect(await screen.findByRole("alert")).toHaveTextContent(lacks.title);
  });
});
