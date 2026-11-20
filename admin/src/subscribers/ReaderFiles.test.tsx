import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it, vi } from "vitest";
import { adminBlog } from "../../test/blog";
import { axeViolations, json, renderAt, signedIn } from "../../test/render";
import { rowsPage } from "../../test/rows";
import { subscriberWords } from "../strings/subscriberWords";

const words = subscriberWords.files;

const SUBSTACK = "email,active_subscription,created_at\nkim@example.com,true,2021-03-04\n";

const EXPORT = "email,status,created_at,lists,labels\nlena@example.com,confirmed,,main,\n";

/** A blog with two newsletters whose import counts `added` readers, and that records each import. */
function blog(added: number) {
  const imports: { url: URL; body: string }[] = [];
  const pagination = { page: 1, limit: 50, pages: 1, total: 0, next: null, prev: null };
  adminBlog({
    "GET /newsletters/": () =>
      json(200, {
        newsletters: [
          { name: "main", confirmed: 2, pending: 0, form: "<form></form>" },
          { name: "weekly", confirmed: 1, pending: 0, form: "<form></form>" },
        ],
        sending: "ready",
        sends: [],
      }),
    "GET /subscribers/": () => json(200, { subscribers: [], meta: { pagination } }),
    "GET /posts/": () => rowsPage([]),
    "GET /subscribers/export/": () =>
      new Response(EXPORT, { headers: { "Content-Type": "text/csv" } }),
    "POST /subscribers/import/": async (request) => {
      const url = new URL(request.url);
      imports.push({ url, body: await request.text() });
      const skipped = [{ line: 5, why: "its address cannot take mail" }];
      const saved = url.searchParams.get("check") !== "true";
      return json(200, { source: "substack", added, already: 12, skipped, saved });
    },
  });
  return imports;
}

/** Picks the Substack file on the screen. */
async function pick() {
  const file = new File([SUBSTACK], "email_list.csv", { type: "text/csv" });
  await userEvent.upload(await screen.findByLabelText(words.file), file);
}

describe("moving readers in or out", () => {
  it("shows how many readers a file adds before it saves any", async () => {
    signedIn();
    const imports = blog(1204);
    renderAt("/subscribers");
    await pick();
    expect(await screen.findByText(words.counted("Substack", 1204))).toBeVisible();
    expect(screen.getByText(/12 readers are here already/)).toBeVisible();
    expect(screen.getByText(words.skipped(1))).toBeVisible();
    expect(imports.map(({ url }) => url.searchParams.get("check"))).toEqual(["true"]);
    expect(imports[0]?.body).toBe(SUBSTACK);
    expect(await axeViolations()).toEqual([]);

    await userEvent.click(screen.getByRole("button", { name: words.add(1204) }));
    expect(await screen.findByText(words.added(1204))).toBeVisible();
    expect(imports.map(({ url }) => url.searchParams.get("check"))).toEqual(["true", null]);
    expect(imports[1]?.url.searchParams.get("list")).toBe("main");
  });

  it("counts again when the newsletter changes, and offers no yes when nothing is new", async () => {
    signedIn();
    const imports = blog(0);
    renderAt("/subscribers");
    await pick();
    expect(await screen.findByText(words.nothingNew, { exact: false })).toBeVisible();
    await userEvent.selectOptions(screen.getByLabelText(words.into), "weekly");
    await vi.waitFor(() => expect(imports).toHaveLength(2));
    expect(imports[1]?.url.searchParams.get("list")).toBe("weekly");
    expect(screen.queryByRole("button", { name: words.add(0) })).toBeNull();
  });

  it("downloads every reader as a CSV file", async () => {
    signedIn();
    blog(0);
    const saved: Blob[] = [];
    URL.createObjectURL = (made: Blob | MediaSource) => {
      if (made instanceof Blob) {
        saved.push(made);
      }
      return "blob:subscribers";
    };
    URL.revokeObjectURL = () => undefined;
    renderAt("/subscribers");
    await userEvent.click(await screen.findByRole("button", { name: words.download }));
    await vi.waitFor(() => expect(saved).toHaveLength(1));
    expect(await saved[0]?.text()).toBe(EXPORT);
  });
});
