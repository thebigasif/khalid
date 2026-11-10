import { screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { adminBlog } from "../../test/blog";
import { axeViolations, json, renderAt, signedIn } from "../../test/render";
import type { AdminDelivery, AdminWebhook } from "../api/webhooks";
import { en } from "../strings/en";

const ZAPIER: AdminWebhook = {
  id: 3,
  event: "post.published",
  url: "https://hooks.zapier.com/hooks/catch/123/abc/",
  created_at: "2026-10-28T09:00:00Z",
};

/** The three rows from the mockup, newest first, as the API lists them. */
const SENT: AdminDelivery[] = [
  {
    id: 9,
    event: "post.published",
    state: "delivered",
    state_at: "2026-10-31T09:12:00Z",
    attempts: [{ at: "2026-10-31T09:12:00Z", status: 200 }],
  },
  {
    id: 8,
    event: "post.published",
    state: "pending",
    state_at: "2026-10-30T16:46:00Z",
    attempts: [
      { at: "2026-10-30T16:40:00Z", status: 503 },
      { at: "2026-10-30T16:41:00Z", reason: "the connection timed out" },
    ],
  },
  {
    id: 7,
    event: "post.published",
    state: "gave_up",
    state_at: "2026-10-29T07:36:00Z",
    attempts: [{ at: "2026-10-28T08:00:00Z", status: 404 }],
  },
];

const words = en.webhooks.sent;

describe("the webhook activity screen", () => {
  it("opens from the Webhooks screen and says where each message stands", async () => {
    signedIn();
    adminBlog({
      "GET /webhooks/": () => json(200, { webhooks: [ZAPIER] }),
      "GET /webhooks/3/deliveries/": () => json(200, { deliveries: SENT }),
    });
    renderAt("/webhooks");
    const label = en.webhooks.list.seeLabel("A post goes live", "hooks.zapier.com");
    await userEvent.click(await screen.findByRole("link", { name: label }));
    expect(await screen.findByRole("heading", { level: 1, name: words.title })).toBeVisible();
    expect(screen.getByText(/sends "A post goes live" to hooks.zapier.com/)).toBeVisible();
    const table = screen.getByRole("table", { name: words.list });
    const [, delivered, pending, gaveUp] = within(table).getAllByRole("row");
    expect(delivered).toHaveTextContent(words.delivered);
    expect(delivered).toHaveTextContent("answered 200");
    expect(pending).toHaveTextContent("Trying again at");
    expect(pending).toHaveTextContent("could not reach it: the connection timed out");
    expect(gaveUp).toHaveTextContent(words.gaveUp);
    expect(gaveUp).toHaveTextContent(words.gaveUpHint);
    expect(await axeViolations()).toEqual([]);
  });

  it("shows older messages a page at a time", async () => {
    signedIn();
    const [newest, , oldest] = SENT;
    const sent = adminBlog({
      "GET /webhooks/": () => json(200, { webhooks: [ZAPIER] }),
      "GET /webhooks/3/deliveries/": (request) =>
        new URL(request.url).searchParams.get("before") === "9"
          ? json(200, { deliveries: [oldest], next: null })
          : json(200, { deliveries: [newest], next: 9 }),
    });
    renderAt("/webhooks/3");
    expect(await screen.findByText("answered 200", { exact: false })).toBeVisible();
    await userEvent.click(screen.getByRole("button", { name: words.older }));
    expect(await screen.findByText(words.gaveUp)).toBeVisible();
    expect(screen.queryByText("answered 200", { exact: false })).toBeNull();
    expect(sent.some((request) => request.url.includes("before=9"))).toBe(true);
    expect(screen.queryByRole("button", { name: words.older })).toBeNull();
    await userEvent.click(screen.getByRole("button", { name: words.newest }));
    expect(await screen.findByText("answered 200", { exact: false })).toBeVisible();
    expect(await axeViolations()).toEqual([]);
  });

  it("says so when the webhook has sent nothing yet", async () => {
    signedIn();
    adminBlog({
      "GET /webhooks/": () => json(200, { webhooks: [ZAPIER] }),
      "GET /webhooks/3/deliveries/": () => json(200, { deliveries: [] }),
    });
    renderAt("/webhooks/3");
    expect(await screen.findByText(words.none)).toBeVisible();
  });

  it("explains a webhook that is not on in plain words", async () => {
    signedIn();
    adminBlog({ "GET /webhooks/": () => json(200, { webhooks: [ZAPIER] }) });
    renderAt("/webhooks/12");
    expect(await screen.findByText(words.gone.what)).toBeVisible();
    expect(screen.getByRole("link", { name: words.back })).toBeVisible();
    expect(await axeViolations()).toEqual([]);
  });
});
