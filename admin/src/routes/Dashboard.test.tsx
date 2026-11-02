import { screen, within } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { visitors } from "../../test/analytics";
import { axeViolations, json, renderAt, serve, signedIn } from "../../test/render";
import { listing, row, rowsPage } from "../../test/rows";
import type { PostRow } from "../api/rows";
import { dashboardWords as words } from "../strings/dashboardWords";
import { en } from "../strings/en";

const STUCK = row({
  id: "01JB",
  title: "Planning a topic cluster in an afternoon",
  status: "draft",
  gate: "refuses",
  lint: { blocks: 1, warnings: 2 },
});

const DRAFT = row({ id: "01JC", title: "Notes on anchor text", status: "draft" });

type Blog = {
  blocked?: PostRow[];
  drafts?: PostRow[];
  scheduled?: number;
  /** How many comments wait for a yes; none when the blog cannot list them. */
  waiting?: number;
  /** Whether the blog counts its visits. */
  counts?: boolean;
};

/** A blog with five live posts and what `blog` says, which records every request. */
function blog({ blocked = [], drafts = [DRAFT], scheduled = 0, waiting, counts }: Blog = {}) {
  const sent: Request[] = [];
  serve((request) => {
    sent.push(request);
    const path = new URL(request.url).pathname;
    if (path === "/api/admin/v1/comments/" && waiting !== undefined) {
      const comment = { id: 1, text: "Thanks", status: "pending" };
      return json(200, { comments: Array.from({ length: waiting }, () => comment) });
    }
    if (path === "/api/admin/v1/analytics/visitors/" && counts === true) {
      return json(200, visitors());
    }
    const query = listing(request);
    const pages: Record<string, () => Response> = {
      "gate:refuses": () => rowsPage(blocked),
      "status:published": () => rowsPage([row()], 5),
      "status:scheduled": () => rowsPage([], scheduled),
      "status:draft": () => rowsPage(drafts, drafts.length === 0 ? 0 : 2),
    };
    return pages[query?.get("filter") ?? ""]?.() ?? json(404, {});
  });
  return sent;
}

/** The number card whose label is `label`. */
function card(label: string) {
  const item = screen.getByText(label).closest("li");
  if (item === null) {
    throw new Error(`no card for ${label}`);
  }
  return item;
}

afterEach(() => {
  document.head.querySelector("meta[name=khalid-name]")?.remove();
});

describe("the dashboard", () => {
  it("greets the blog by name and shows its numbers", async () => {
    const meta = document.createElement("meta");
    meta.name = "khalid-name";
    meta.content = "Field notes";
    document.head.append(meta);
    signedIn();
    blog({ counts: true });
    renderAt("/");
    const title = words.greeting("Field notes");
    expect(await screen.findByRole("heading", { level: 1, name: title })).toBeVisible();
    expect(screen.getAllByRole("heading", { level: 1 })).toHaveLength(1);
    expect(screen.getByText(en.dashboard.calm)).toBeVisible();
    expect(card(words.live)).toHaveTextContent("5");
    expect(card(words.drafts)).toHaveTextContent("2");
    expect(card(words.failing)).toHaveTextContent(`0${words.allPass}`);
    expect(await screen.findByText(words.visits)).toBeVisible();
    expect(card(words.visits)).toHaveTextContent("321");
    const analytics = within(card(words.visits)).getByRole("link", { name: words.visitsLink });
    expect(analytics).toHaveAttribute("href", "/admin/analytics");
    expect(screen.getByRole("link", { name: en.skip })).toHaveAttribute("href", "#main");
    expect(await axeViolations()).toEqual([]);
  });

  it("offers to connect Search Console when the blog cannot count visits", async () => {
    signedIn();
    blog();
    renderAt("/");
    const connect = await screen.findByRole("link", { name: words.connect });
    expect(connect).toHaveAttribute("href", "/admin/search-console");
    expect(card(words.searchClicks)).toHaveTextContent(words.connectWhy);
    expect(screen.queryByText(words.visits)).toBeNull();
  });

  it("lists the recent drafts with where each stands, who changed it and when", async () => {
    signedIn();
    const sent = blog();
    renderAt("/");
    const drafts = await screen.findByRole("region", { name: words.recentDrafts });
    const first = within(drafts).getAllByRole("row")[1];
    expect(first).toHaveTextContent("Notes on anchor text");
    expect(first).toHaveTextContent(en.status.draft);
    expect(first).toHaveTextContent("Asif");
    expect(first).toHaveTextContent("12 October 2026");
    const link = within(drafts).getByRole("link", { name: "Notes on anchor text" });
    expect(link).toHaveAttribute("href", "/admin/posts/01JC");
    const asked = sent.map(listing).find((query) => query?.get("filter") === "status:draft");
    expect(asked?.get("order")).toBe("updated_at desc");
    expect(asked?.get("limit")).toBe("5");
  });

  it("puts failing posts, scheduled posts and waiting comments under Needs attention", async () => {
    signedIn();
    blog({ blocked: [STUCK], scheduled: 2, waiting: 3 });
    renderAt("/");
    const focus = await screen.findByRole("region", { name: words.attention });
    const stuck = within(focus).getByRole("link", { name: STUCK.title });
    expect(stuck).toHaveAttribute("href", "/admin/posts/01JB");
    const all = within(focus).getByRole("link", { name: en.dashboard.showBlocked });
    expect(all).toHaveAttribute("href", "/admin/posts?checks=blocked");
    const soon = await within(focus).findByRole("link", { name: words.scheduled(2) });
    expect(soon).toHaveAttribute("href", "/admin/posts?stage=scheduled");
    const comments = await within(focus).findByRole("link", { name: words.waiting(3) });
    expect(comments).toHaveAttribute("href", "/admin/comments");
    expect(screen.getByText(en.dashboard.blockedLede(1))).toBeVisible();
    expect(card(words.failing)).toHaveTextContent("1");
    expect(screen.getByRole("link", { name: words.failingLink })).toHaveAttribute(
      "href",
      "/admin/posts?checks=blocked",
    );
    expect(await axeViolations()).toEqual([]);
  });

  it("gives one next step when nothing needs you and there are no drafts", async () => {
    signedIn();
    blog({ drafts: [], waiting: 0 });
    renderAt("/");
    const focus = await screen.findByRole("region", { name: words.attention });
    expect(within(focus).getByText(words.allClear)).toBeVisible();
    expect(within(focus).getByRole("link", { name: words.startPost })).toHaveAttribute(
      "href",
      "/admin/new",
    );
    const drafts = screen.getByRole("region", { name: words.recentDrafts });
    expect(within(drafts).getByText(words.noDrafts)).toBeVisible();
    expect(within(drafts).getByRole("link", { name: en.dashboard.firstPost })).toBeVisible();
    const quick = screen.getByRole("region", { name: words.quick });
    expect(within(quick).getAllByRole("link")).toHaveLength(4);
  });

  it("explains a list the blog would not send", async () => {
    signedIn();
    serve(() =>
      json(403, {
        type: "https://khalid.dev/problems/scope",
        title: "This key cannot list posts.",
        status: 403,
        detail: "Listing posts needs posts:write.",
        fix: "Sign in with a staff token that has the editor role.",
      }),
    );
    renderAt("/");
    expect(await screen.findByRole("alert")).toHaveTextContent("This key cannot list posts.");
  });
});
