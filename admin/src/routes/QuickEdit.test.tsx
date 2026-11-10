import { screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { redirectsBlog } from "../../test/redirects";
import { axeViolations, json, renderAt, signedIn } from "../../test/render";
import { row, rowsPage } from "../../test/rows";
import type { AdminPost } from "../api/posts";
import { quickEditWords as words } from "../strings/quickEditWords";

const LINKS = row({ tags: ["internal links"] });

/** The post behind the row, as the Admin API reads it now. */
const POST: AdminPost = {
  id: LINKS.id,
  slug: LINKS.slug,
  title: LINKS.title,
  author: "asif",
  status: "published",
  markdown: "Links between posts help readers.\n",
  updated_at: "2026-10-12T11:05:00Z",
  settings: {
    co_authors: [],
    featured: false,
    review_state: "ready",
    access: "public",
    password_protected: false,
    tags: ["internal links"],
    share_cards: {},
    aliases: [],
    excerpt: "Why links between posts matter.",
  },
};

const CATEGORY = {
  id: "on-page-seo",
  slug: "on-page-seo",
  names: { en: "On-page SEO" },
  posts: 4,
  updated_at: "2026-10-12T11:00:00Z",
};

/** A blog that lists the one row, reads its post, and answers a save with `saved`. */
function blog(saved: () => Response) {
  return redirectsBlog({
    "GET /posts/": () => rowsPage([LINKS]),
    "GET /posts/01JA/": () => json(200, { posts: [POST] }),
    "GET /categories/": () => json(200, { categories: [CATEGORY] }),
    "PUT /posts/01JA/": saved,
  });
}

/** Opens quick edit on the row and gives its form. */
async function open() {
  renderAt("/posts");
  await userEvent.click(await screen.findByRole("button", { name: words.open(LINKS.title) }));
  return screen.findByRole("form", { name: words.label(POST.title) });
}

describe("quick edit in the posts table", () => {
  it("saves the description, tags and category from the copy read now", async () => {
    signedIn();
    const sent = blog(() => json(200, { posts: [POST] }));
    const form = await open();
    expect(within(form).getByLabelText(words.description)).toHaveValue(POST.settings.excerpt);
    expect(await axeViolations()).toEqual([]);
    const description = within(form).getByLabelText(words.description);
    await userEvent.clear(description);
    await userEvent.type(description, "What links between posts do for readers.");
    await userEvent.type(within(form).getByLabelText(words.tags), ", Site structure");
    await within(form).findByRole("option", { name: "On-page SEO" });
    await userEvent.selectOptions(within(form).getByLabelText(words.category), "on-page-seo");
    await userEvent.click(within(form).getByRole("button", { name: words.save }));
    expect(await screen.findByText(words.saved(POST.title))).toBeVisible();
    expect(screen.queryByRole("form", { name: words.label(POST.title) })).toBeNull();
    const put = sent.find((request) => request.method === "PUT");
    expect(await put?.json()).toEqual({
      posts: [
        {
          updated_at: POST.updated_at,
          settings: {
            excerpt: "What links between posts do for readers.",
            tags: ["internal links", "Site structure"],
            category: "on-page-seo",
          },
        },
      ],
    });
  });

  it("keeps the form open and says why when the save is refused", async () => {
    signedIn();
    const stale = {
      type: "https://khalid.dev/problems/conflict",
      title: "Someone else changed this first",
      status: 409,
      detail: "Lena changed this post after the copy you edited.",
      fix: "Close quick edit, open it again, and make your change once more.",
    };
    blog(() => json(409, stale));
    const form = await open();
    await userEvent.type(within(form).getByLabelText(words.title), " today");
    await userEvent.click(within(form).getByRole("button", { name: words.save }));
    expect(await within(form).findByText(stale.title)).toBeVisible();
    expect(within(form).getByLabelText(words.title)).toHaveValue(`${POST.title} today`);
  });

  it("closes without a request when nothing changed", async () => {
    signedIn();
    const sent = blog(() => json(500, {}));
    const form = await open();
    await userEvent.click(within(form).getByRole("button", { name: words.save }));
    expect(await screen.findByText(words.nothing)).toBeVisible();
    expect(sent.filter((request) => request.method === "PUT")).toHaveLength(0);
  });
});
