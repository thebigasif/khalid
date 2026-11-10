import { screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { adminBlog } from "../../test/blog";
import { axeViolations, json, renderAt, signedIn } from "../../test/render";
import type { Design, DesignChange } from "../api/design";
import { designWords } from "../strings/designWords";
import { en } from "../strings/en";

const SYSTEM = 'system-ui, -apple-system, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif';

// Khalid's default tokens, as a site with no theme file reads them.
const DEMO: Design = {
  tokens: {
    "colors.background": "#ffffff",
    "colors.surface": "#f6f7f9",
    "colors.text": "#1f2328",
    "colors.muted": "#57606a",
    "colors.link": "#0b57d0",
    "colors.accent": "#0b57d0",
    "colors.border": "#d0d7de",
    "fonts.body": SYSTEM,
    "fonts.heading": SYSTEM,
    "fonts.mono": "ui-monospace, SFMono-Regular, Menlo, Consolas, monospace",
    "shape.radius": "0.375rem",
    "shape.measure": "40rem",
    "brand.logo": "",
    "brand.icon": "",
  },
  locked: ["colors.link"],
  fonts: [
    { label: "System font", stack: SYSTEM },
    { label: "Serif", stack: 'Georgia, "Times New Roman", serif' },
  ],
  contrast: [],
  updated_at: "2026-11-01T09:00:00.123456789Z",
};

/** A blog with the default design, recording each change. */
function blog() {
  const changes: DesignChange[] = [];
  let design = DEMO;
  adminBlog({
    "GET /design/": () => json(200, { design }),
    "PUT /design/": async (request) => {
      const body: { design: DesignChange } = await request.json();
      changes.push(body.design);
      const tokens = { ...design.tokens, ...body.design.tokens };
      const locked = body.design.locked ?? design.locked;
      design = { ...design, tokens, locked, updated_at: "2026-11-10T10:00:00Z" };
      return json(200, { design });
    },
  });
  return changes;
}

const WORDS = designWords;

/** The row of the colour table for `usedFor`. */
function row(usedFor: string): HTMLElement {
  const header = screen.getByRole("rowheader", { name: usedFor });
  const found = header.closest("tr");
  if (found === null) {
    throw new Error(`no row for ${usedFor}`);
  }
  return found;
}

describe("the Design screen", () => {
  it("shows every colour with its contrast beside it", async () => {
    signedIn();
    blog();
    renderAt("/");
    await userEvent.click(await screen.findByRole("link", { name: en.nav.design }));
    const text = await screen.findByLabelText(WORDS.colorLabel(WORDS.usedFor["colors.text"]));
    expect(text).toHaveValue("#1f2328");
    expect(within(row(WORDS.usedFor["colors.text"])).getByText("15.7 to 1")).toBeVisible();
    expect(within(row(WORDS.usedFor["colors.border"])).getByText(WORDS.notText)).toBeVisible();
    expect(screen.getByText(WORDS.allReadable)).toBeVisible();
    const keepLink = WORDS.keepLabel(WORDS.usedFor["colors.link"]);
    expect(screen.getByRole("checkbox", { name: keepLink })).toBeChecked();
    expect(screen.getByLabelText(WORDS.body)).toHaveValue(SYSTEM);
    expect(screen.getByText(WORDS.noLogo)).toBeVisible();
    expect(await axeViolations()).toEqual([]);
  });

  it("scores a colour as it is typed, then saves only what changed", async () => {
    signedIn();
    const changes = blog();
    renderAt("/design");
    const label = WORDS.colorLabel(WORDS.usedFor["colors.text"]);
    const text = await screen.findByLabelText(label);
    await userEvent.clear(text);
    await userEvent.type(text, "#ffffff");
    const textRow = row(WORDS.usedFor["colors.text"]);
    expect(within(textRow).getByText(WORDS.tooFaint(1))).toBeVisible();
    expect(screen.getByText(WORDS.someFaint(1))).toBeVisible();
    await userEvent.clear(text);
    await userEvent.type(text, "#000000");
    expect(within(textRow).getByText(WORDS.ratio(21))).toBeVisible();
    expect(screen.getByText(WORDS.allReadable)).toBeVisible();
    const keepText = WORDS.keepLabel(WORDS.usedFor["colors.text"]);
    await userEvent.click(screen.getByRole("checkbox", { name: keepText }));
    await userEvent.selectOptions(screen.getByLabelText(WORDS.heading), "Serif");
    expect(await axeViolations()).toEqual([]);
    await userEvent.click(screen.getByRole("button", { name: WORDS.save }));
    expect(await screen.findByText(WORDS.live)).toBeVisible();
    expect(changes).toEqual([
      {
        updated_at: DEMO.updated_at,
        tokens: { "colors.text": "#000000", "fonts.heading": 'Georgia, "Times New Roman", serif' },
        locked: ["colors.link", "colors.text"],
      },
    ]);
    expect(screen.getByLabelText(label)).toHaveValue("#000000");
  });

  it("sends nothing unchanged, and shows the server's words for a refusal", async () => {
    signedIn();
    const changes: DesignChange[] = [];
    adminBlog({
      "GET /design/": () => json(200, { design: DEMO }),
      "PUT /design/": async (request) => {
        const body: { design: DesignChange } = await request.json();
        changes.push(body.design);
        const detail = 'This change was refused: colors.link cannot be "blue".';
        return json(422, { title: "The request body is not valid", detail, fix: "Change it." });
      },
    });
    renderAt("/design");
    await userEvent.click(await screen.findByRole("button", { name: WORDS.save }));
    expect(await screen.findByText(WORDS.unchanged)).toBeVisible();
    expect(changes).toEqual([]);
    const link = screen.getByLabelText(WORDS.colorLabel(WORDS.usedFor["colors.link"]));
    await userEvent.clear(link);
    await userEvent.type(link, "blue");
    expect(within(row(WORDS.usedFor["colors.link"])).getByText(WORDS.notColor)).toBeVisible();
    await userEvent.click(screen.getByRole("button", { name: WORDS.save }));
    expect(await screen.findByText(/colors.link cannot be "blue"/)).toBeVisible();
    expect(changes).toHaveLength(1);
  });
});
