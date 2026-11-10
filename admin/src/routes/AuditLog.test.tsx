import { screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { redirectsBlog } from "../../test/redirects";
import { axeViolations, json, renderAt, signedIn } from "../../test/render";
import type { Change } from "../api/records";
import { en } from "../strings/en";

/** A change logged at place `id`, made by Maria Lopez's account at 09:30 on 31 October. */
function change(id: number, action: string, route: string, path: string): Change {
  const by = { by: "0000000000000002", by_name: "Maria Lopez" };
  return { id, at: "2026-10-31T09:30:00Z", ...by, action, route, path };
}

/** The same change made by a key revoked since, which has no name left. */
function byRevokedKey(logged: Change): Change {
  return { ...logged, by: "4f1c9a20b7e3d615", by_name: "4f1c9a20b7e3d615" };
}

const REMOVED = change(
  3,
  "removed",
  "/api/admin/v1/posts/{id}/",
  "/api/admin/v1/posts/en.what-a-sitemap-does/",
);
const RESTORED = byRevokedKey(
  change(
    2,
    "added",
    "/api/admin/v1/trash/{key}/restore/",
    "/api/admin/v1/trash/20261030T160500Z-en.hello/restore/",
  ),
);
const REDIRECT = change(1, "added", "/api/admin/v1/redirects/", "/api/admin/v1/redirects/");

describe("the audit log", () => {
  it("says who changed what and where, newest first", async () => {
    signedIn();
    redirectsBlog({ "GET /audit/": () => json(200, { changes: [REMOVED, RESTORED], next: null }) });
    renderAt("/");
    await userEvent.click(await screen.findByRole("link", { name: en.nav.audit }));
    expect(await screen.findByRole("heading", { level: 1, name: en.audit.title })).toBeVisible();
    const table = await screen.findByRole("table", { name: en.audit.title });
    const [, removed, restored] = within(table).getAllByRole("row");
    expect(removed).toHaveTextContent("Removed a post");
    expect(removed).toHaveTextContent("/posts/en.what-a-sitemap-does/");
    expect(removed).toHaveTextContent("Maria Lopez");
    expect(removed).not.toHaveTextContent("0000000000000002");
    expect(restored).toHaveTextContent("Put a post back from the trash");
    expect(restored).toHaveTextContent("4f1c9a20b7e3d615");
    expect(screen.queryByRole("button", { name: en.audit.older })).toBeNull();
    expect(await axeViolations()).toEqual([]);
  });

  it("shows older changes a page at a time", async () => {
    signedIn();
    const sent = redirectsBlog({
      "GET /audit/": (request) =>
        new URL(request.url).searchParams.get("before") === "2"
          ? json(200, { changes: [REDIRECT], next: null })
          : json(200, { changes: [REMOVED, RESTORED], next: 2 }),
    });
    renderAt("/audit");
    await userEvent.click(await screen.findByRole("button", { name: en.audit.older }));
    expect(await screen.findByText("Added a redirect")).toBeVisible();
    expect(screen.queryByText("Removed a post")).toBeNull();
    expect(sent.some((request) => request.url.includes("before=2"))).toBe(true);
    await userEvent.click(screen.getByRole("button", { name: en.audit.newest }));
    expect(await screen.findByText("Removed a post")).toBeVisible();
  });

  it("shows why a key without admin rights cannot read it", async () => {
    signedIn();
    const problem = {
      type: "https://khalid.dev/problems/forbidden",
      title: "This key cannot do that",
      status: 403,
      detail: "The key lacks admin:all.",
      fix: "Sign in as an owner or an admin.",
    };
    redirectsBlog({ "GET /audit/": () => json(403, problem) });
    renderAt("/audit");
    expect(await screen.findByRole("alert")).toHaveTextContent(problem.fix);
  });
});
