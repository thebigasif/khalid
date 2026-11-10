import { screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { adminBlog } from "../../test/blog";
import { axeViolations, json, renderAt, signedIn } from "../../test/render";
import type { SiteChange, SiteSettings } from "../api/site";
import { navWords as WORDS } from "../strings/navWords";

// The demo site's settings, with one link in the footer.
const DEMO: SiteSettings = {
  name: "Field Notes on SEO",
  address: "https://www.example.com",
  mount: "/blog/",
  default_language: "en",
  languages: ["en", "de"],
  template: { name: "classic", options: {} },
  nav: { primary: [], footer: [{ label: "About", href: "/blog/about/" }] },
  updated_at: "2026-11-10T09:00:00Z",
  restart_needed: false,
};

/** A blog with the demo settings that records each change and saves its menus. */
function blog() {
  const changes: SiteChange[] = [];
  let site = DEMO;
  adminBlog({
    "GET /settings/": () => json(200, { settings: site }),
    "PUT /settings/": async (request) => {
      const body: { settings: SiteChange } = await request.json();
      changes.push(body.settings);
      const nav = { primary: [], footer: [], ...site.nav, ...body.settings.nav };
      site = { ...site, nav, updated_at: "2026-11-10T09:05:00Z" };
      return json(200, { settings: site });
    },
  });
  return changes;
}

/** The card of `title`, such as the header menu's. */
async function card(title: string) {
  const heading = await screen.findByRole("heading", { name: title });
  const section = heading.closest("section");
  if (section === null) {
    throw new Error(`no card for ${title}`);
  }
  return within(section);
}

describe("the Navigation screen", () => {
  it("opens from the side menu and shows both menus", async () => {
    signedIn();
    blog();
    renderAt("/");
    await userEvent.click(await screen.findByRole("link", { name: WORDS.nav }));
    const header = await card(WORDS.menus.primary.title);
    expect(header.getByText(WORDS.menus.primary.none)).toBeVisible();
    const footer = await card(WORDS.menus.footer.title);
    expect(footer.getByText("About")).toBeVisible();
    expect(footer.getByText("/blog/about/")).toBeVisible();
    expect(await axeViolations()).toEqual([]);
  });

  it("adds and orders header links, and saves both menus together", async () => {
    signedIn();
    const changes = blog();
    renderAt("/navigation");
    const header = await card(WORDS.menus.primary.title);
    const add = header.getByRole("button", { name: WORDS.addTo("primary") });
    await userEvent.click(add);
    expect(header.getByText(WORDS.labelMissing)).toBeVisible();
    await userEvent.type(header.getByLabelText(WORDS.label.label), "Guides");
    await userEvent.type(header.getByLabelText(WORDS.href.label), "blog/guides/");
    await userEvent.click(add);
    expect(header.getByText(WORDS.hrefWrong)).toBeVisible();
    await userEvent.clear(header.getByLabelText(WORDS.href.label));
    await userEvent.type(header.getByLabelText(WORDS.href.label), "/blog/guides/");
    await userEvent.click(add);
    await userEvent.type(header.getByLabelText(WORDS.label.label), "Impressum");
    await userEvent.type(header.getByLabelText(WORDS.href.label), "/blog/de/impressum/");
    await userEvent.selectOptions(header.getByLabelText(WORDS.language.label), "de");
    await userEvent.click(add);
    await userEvent.click(header.getByRole("button", { name: WORDS.upNamed("Impressum (de)") }));
    expect(await axeViolations()).toEqual([]);
    await userEvent.click(screen.getByRole("button", { name: WORDS.save }));
    expect(await screen.findByText(WORDS.live)).toBeVisible();
    expect(changes).toEqual([
      {
        updated_at: DEMO.updated_at,
        nav: {
          primary: [
            { label: "Impressum", href: "/blog/de/impressum/", language: "de" },
            { label: "Guides", href: "/blog/guides/" },
          ],
          footer: DEMO.nav?.footer,
        },
      },
    ]);
    expect(await screen.findByText("/blog/guides/")).toBeVisible();
  });

  it("removes a footer link and saves the footer as an empty menu", async () => {
    signedIn();
    const changes = blog();
    renderAt("/navigation");
    const footer = await card(WORDS.menus.footer.title);
    await userEvent.click(footer.getByRole("button", { name: WORDS.removeNamed("About") }));
    expect(footer.getByText(WORDS.menus.footer.none)).toBeVisible();
    await userEvent.click(screen.getByRole("button", { name: WORDS.save }));
    expect(await screen.findByText(WORDS.live)).toBeVisible();
    expect(changes).toEqual([{ updated_at: DEMO.updated_at, nav: { primary: [], footer: [] } }]);
  });
});
