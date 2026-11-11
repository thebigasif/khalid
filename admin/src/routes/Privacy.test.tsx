import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { PRIVACY_DEMO, privacyBlog } from "../../test/privacy";
import { axeViolations, renderAt, signedIn } from "../../test/render";
import { privacyWords } from "../strings/privacyWords";

const WORDS = privacyWords;
const SAVE = "Save this part";

async function openTab(name: string) {
  await userEvent.click(await screen.findByRole("button", { name }));
  return screen.findByRole("heading", { name });
}

describe("the Privacy screen", () => {
  it("opens from the side menu on its settings, with the defaults a fresh site has", async () => {
    signedIn();
    privacyBlog();
    renderAt("/");
    await userEvent.click(await screen.findByRole("link", { name: WORDS.title }));
    expect(await screen.findByRole("heading", { name: WORDS.tabs.settings })).toBeVisible();
    expect(screen.getByLabelText(WORDS.settings.months)).toHaveValue(6);
    expect(screen.getByRole("radio", { name: WORDS.settings.gpcMarketing })).toBeChecked();
    expect(screen.getByRole("checkbox", { name: WORDS.settings.counts })).toBeChecked();
    expect(await axeViolations()).toEqual([]);
  });

  it("saves only the lifetime it changed, and says the blog uses it with no restart", async () => {
    signedIn();
    const changes = privacyBlog();
    renderAt("/privacy");
    await userEvent.click(await screen.findByRole("button", { name: SAVE }));
    expect(await screen.findByText(WORDS.unchanged)).toBeVisible();
    const months = screen.getByLabelText(WORDS.settings.months);
    await userEvent.clear(months);
    await userEvent.type(months, "12");
    await userEvent.click(screen.getByRole("button", { name: SAVE }));
    expect(await screen.findByText(WORDS.live)).toBeVisible();
    expect(changes).toEqual([
      { updated_at: PRIVACY_DEMO.updated_at, privacy: { lifetime_months: 12 } },
    ]);
    expect(screen.queryByText(WORDS.restart)).toBeNull();
  });

  it("tags a script as tracking no one, says it waits for a restart, and shows the tag after a reload", async () => {
    signedIn();
    const changes = privacyBlog();
    renderAt("/privacy");
    await openTab(WORDS.tabs.scripts);
    const script = screen.getByRole("listitem", { name: WORDS.scripts.script(1) });
    expect(within(script).getByText(WORDS.scripts.untagged)).toBeVisible();
    await userEvent.selectOptions(
      within(script).getByLabelText(WORDS.scripts.category),
      WORDS.categories["no-tracking"],
    );
    await userEvent.type(within(script).getByLabelText(WORDS.scripts.vendor), "Example Stats");
    await userEvent.click(screen.getByRole("button", { name: SAVE }));
    expect(await screen.findByText(WORDS.saved)).toBeVisible();
    expect(changes).toEqual([
      {
        updated_at: PRIVACY_DEMO.updated_at,
        code: [
          { head: PRIVACY_DEMO.code?.[0]?.head, category: "no-tracking", vendor: "Example Stats" },
        ],
      },
    ]);
    expect(await screen.findByText(WORDS.restart)).toBeVisible();
    const reloaded = await screen.findByRole("listitem", { name: WORDS.scripts.script(1) });
    expect(within(reloaded).getByLabelText(WORDS.scripts.category)).toHaveValue("no-tracking");
    expect(within(reloaded).queryByText(WORDS.scripts.untagged)).toBeNull();
    expect(await axeViolations()).toEqual([]);
  });

  it("says why there is no banner on a static site with no scripts", async () => {
    signedIn();
    privacyBlog({ ...PRIVACY_DEMO, mode: "static", code: [] });
    renderAt("/privacy");
    expect(await screen.findByText(WORDS.settings.countsStatic)).toBeVisible();
    expect(screen.queryByRole("checkbox", { name: WORDS.settings.counts })).toBeNull();
    await openTab(WORDS.tabs.scripts);
    expect(screen.getByText(WORDS.scripts.noScripts)).toBeVisible();
    expect(screen.getByText(WORDS.scripts.staticTitle)).toBeVisible();
    expect(screen.queryByRole("button", { name: SAVE })).toBeNull();
  });

  it("warns while contact details are missing, and stops once they are saved", async () => {
    signedIn();
    const changes = privacyBlog();
    renderAt("/privacy");
    await openTab(WORDS.tabs.contact);
    expect(screen.getByText(WORDS.contact.missing)).toBeVisible();
    await userEvent.type(screen.getByLabelText(WORDS.contact.name), "Field Notes Ltd");
    await userEvent.type(screen.getByLabelText(WORDS.contact.email), "privacy@example.com");
    await userEvent.click(screen.getByRole("button", { name: SAVE }));
    expect(await screen.findByText(WORDS.live)).toBeVisible();
    expect(changes.at(-1)?.privacy).toEqual({
      contact: { name: "Field Notes Ltd", email: "privacy@example.com" },
    });
    expect(screen.queryByText(WORDS.contact.missing)).toBeNull();
    expect(await axeViolations()).toEqual([]);
  });

  it("adds an imprint with its own lines, and warns while the contact lacks an address", async () => {
    signedIn();
    const contact = { name: "Notes GmbH", email: "hello@example.com" };
    const changes = privacyBlog({ ...PRIVACY_DEMO, privacy: { contact } });
    renderAt("/privacy");
    await openTab(WORDS.tabs.imprint);
    expect(screen.queryByLabelText(WORDS.imprint.phone)).toBeNull();
    expect(screen.queryByText(WORDS.imprint.missing)).toBeNull();
    await userEvent.click(screen.getByRole("checkbox", { name: WORDS.imprint.show }));
    expect(screen.getByText(WORDS.imprint.missing)).toBeVisible();
    await userEvent.type(screen.getByLabelText(WORDS.imprint.phone), "+49 30 1234567");
    await userEvent.type(screen.getByLabelText(WORDS.imprint.vatId), "DE123456789");
    await userEvent.click(screen.getByRole("button", { name: SAVE }));
    expect(await screen.findByText(WORDS.live)).toBeVisible();
    expect(changes.at(-1)?.privacy).toEqual({
      imprint: { phone: "+49 30 1234567", vat_id: "DE123456789" },
    });
    expect(await axeViolations()).toEqual([]);
  });

  it("keeps an imprint with no lines of its own, and removes it once switched off", async () => {
    signedIn();
    const contact = { name: "Notes GmbH", email: "hello@example.com", address: "Berlin" };
    const changes = privacyBlog({ ...PRIVACY_DEMO, privacy: { contact } });
    renderAt("/privacy");
    await openTab(WORDS.tabs.imprint);
    await userEvent.click(screen.getByRole("checkbox", { name: WORDS.imprint.show }));
    expect(screen.queryByText(WORDS.imprint.missing)).toBeNull();
    await userEvent.click(screen.getByRole("button", { name: SAVE }));
    expect(await screen.findByText(WORDS.live)).toBeVisible();
    expect(changes.at(-1)?.privacy).toEqual({ imprint: {} });
    const show = await screen.findByRole("checkbox", { name: WORDS.imprint.show });
    expect(show).toBeChecked();
    await userEvent.click(show);
    await userEvent.click(screen.getByRole("button", { name: SAVE }));
    await waitFor(() => expect(changes.at(-1)?.privacy).toEqual({ imprint: null }));
  });

  it("keeps each language's banner words apart and saves them as one part", async () => {
    signedIn();
    const changes = privacyBlog();
    renderAt("/privacy");
    await openTab(WORDS.tabs.text);
    await userEvent.type(screen.getByLabelText(WORDS.text.fields.title), "Your choices");
    await userEvent.selectOptions(screen.getByLabelText(WORDS.text.language), "de");
    expect(screen.getByLabelText(WORDS.text.fields.title)).toHaveValue("");
    await userEvent.type(screen.getByLabelText(WORDS.text.fields.title), "Ihre Wahl");
    await userEvent.click(screen.getByRole("button", { name: SAVE }));
    expect(await screen.findByText(WORDS.live)).toBeVisible();
    expect(changes.at(-1)?.privacy).toEqual({
      text: { en: { title: "Your choices" }, de: { title: "Ihre Wahl" } },
    });
  });
});
