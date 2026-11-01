import { screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { adminBlog } from "../../test/blog";
import { axeViolations, json, renderAt, signedIn } from "../../test/render";
import type { SiteChange, SiteSettings } from "../api/site";
import type { Template } from "../api/templates";
import { en } from "../strings/en";

// Classic as the engine lists it, and a stand-in for a template another
// builder adds, with one option of each kind.
const TEMPLATES: Template[] = [
  {
    name: "classic",
    summary: "Today's look: a calm column of text.",
    reader: [],
    options: [],
  },
  {
    name: "ledger",
    summary: "A data-led blog with no pictures in the lists.",
    reader: [{ key: "reading_time", on: false }],
    options: [
      {
        key: "logo",
        help: "Whether the header shows the logo.",
        takes: { kind: "switch", default: true },
      },
      {
        key: "contents",
        help: "Where a long post's contents go.",
        takes: { kind: "choice", allowed: ["inline", "none"], default: "inline" },
      },
      {
        key: "related_count",
        help: "How many related posts show.",
        takes: { kind: "count", least: 3, most: 10, default: 5 },
      },
      {
        key: "stats",
        help: "Short facts under the blog's name.",
        takes: { kind: "lines", most: 3 },
      },
      {
        key: "share_networks",
        help: "The networks the share links go to.",
        takes: { kind: "picks", allowed: ["x", "email"], default: ["email"] },
      },
    ],
  },
];

const DEMO: SiteSettings = {
  name: "Field Notes on SEO",
  address: "https://www.example.com",
  mount: "/blog/",
  default_language: "en",
  languages: ["en", "de"],
  template: { name: "classic", options: {} },
  updated_at: "2026-11-01T09:00:00Z",
  restart_needed: false,
};

/** A blog with the demo settings and two templates, recording each change. */
function blog(refuse = false) {
  const changes: SiteChange[] = [];
  let site = DEMO;
  adminBlog({
    "GET /settings/": () => json(200, { settings: site }),
    "GET /templates/": () => json(200, { templates: TEMPLATES }),
    "PUT /settings/": async (request) => {
      const body: { settings: SiteChange } = await request.json();
      changes.push(body.settings);
      if (refuse) {
        const title = "site.toml cannot take this template.name: there is no such template.";
        return json(422, { title, detail: "The site would not start.", fix: "Send it again." });
      }
      const template = body.settings.template ?? DEMO.template;
      site = { ...DEMO, template, restart_needed: true };
      return json(200, { settings: site });
    },
  });
  return changes;
}

const WORDS = en.look;

describe("the Look screen", () => {
  it("lists every template from the server, with what each turns off", async () => {
    signedIn();
    blog();
    renderAt("/");
    await userEvent.click(await screen.findByRole("link", { name: en.nav.look }));
    const classic = await screen.findByRole("radio", { name: /Classic/ });
    expect(classic).toBeChecked();
    expect(screen.getByText(WORDS.inUse)).toBeVisible();
    const ledger = screen.getByRole("radio", { name: "Ledger" });
    expect(ledger).not.toBeChecked();
    expect(screen.getByText(WORDS.reader.reading_time ?? "")).toBeVisible();
    expect(screen.getByText(WORDS.noOptions)).toBeVisible();
    expect(await axeViolations()).toEqual([]);
  });

  it("shows a template's options at their defaults, and saves only what differs", async () => {
    signedIn();
    const changes = blog();
    renderAt("/look");
    await userEvent.click(await screen.findByRole("radio", { name: "Ledger" }));
    const options = screen.getByRole("region", { name: WORDS.optionsFor("Ledger") });
    expect(within(options).getByRole("checkbox", { name: "Logo" })).toBeChecked();
    expect(within(options).getByLabelText("Contents")).toHaveValue("inline");
    expect(within(options).getByLabelText("Related count")).toHaveValue(5);
    await userEvent.click(within(options).getByRole("checkbox", { name: "Logo" }));
    await userEvent.type(within(options).getByLabelText("Stats"), "12,000 readers");
    await userEvent.click(within(options).getByRole("checkbox", { name: "x" }));
    expect(await axeViolations()).toEqual([]);
    await userEvent.click(screen.getByRole("button", { name: WORDS.save }));
    expect(await screen.findByText(WORDS.saved)).toBeVisible();
    expect(changes).toEqual([
      {
        updated_at: DEMO.updated_at,
        template: {
          name: "ledger",
          options: { logo: false, stats: ["12,000 readers"], share_networks: ["email", "x"] },
        },
      },
    ]);
    expect(await screen.findByText(WORDS.restart)).toBeVisible();
  });

  it("sends nothing for the same look, and shows the server's words for a refusal", async () => {
    signedIn();
    const changes = blog(true);
    renderAt("/look");
    await userEvent.click(await screen.findByRole("button", { name: WORDS.save }));
    expect(await screen.findByText(WORDS.unchanged)).toBeVisible();
    expect(changes).toEqual([]);
    await userEvent.click(screen.getByRole("radio", { name: "Ledger" }));
    await userEvent.click(screen.getByRole("button", { name: WORDS.save }));
    expect(await screen.findByText(/there is no such template/)).toBeVisible();
    expect(changes).toHaveLength(1);
  });
});
