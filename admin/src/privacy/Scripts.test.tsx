import { screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { PRIVACY_DEMO, privacyBlog } from "../../test/privacy";
import { axeViolations, renderAt, signedIn } from "../../test/render";
import { privacyWords } from "../strings/privacyWords";

const WORDS = privacyWords.scripts;
const SAVE = "Save this part";
const TAG = '<script src="https://cdn.example.org/tag.js"></script>';

async function openScripts() {
  const name = privacyWords.tabs.scripts;
  await userEvent.click(await screen.findByRole("button", { name }));
  await screen.findByRole("heading", { name });
}

describe("adding your own code on the Privacy screen", () => {
  it("adds a script to a blog with none, with the category it waits for", async () => {
    signedIn();
    const changes = privacyBlog({ ...PRIVACY_DEMO, code: [] });
    renderAt("/privacy");
    await openScripts();
    expect(screen.getByText(WORDS.noScripts)).toBeVisible();
    await userEvent.click(screen.getByRole("button", { name: WORDS.add }));
    const script = screen.getByRole("listitem", { name: WORDS.script(1) });
    // Typed as one value: user-event reads { and [ as keys to press.
    await userEvent.click(within(script).getByLabelText(WORDS.body));
    await userEvent.paste(TAG);
    await userEvent.selectOptions(
      within(script).getByLabelText(WORDS.category),
      privacyWords.categories.analytics,
    );
    await userEvent.type(within(script).getByLabelText(WORDS.vendor), "Example Analytics");
    expect(await axeViolations()).toEqual([]);
    await userEvent.click(screen.getByRole("button", { name: SAVE }));
    expect(await screen.findByText(privacyWords.restart)).toBeVisible();
    expect(changes).toEqual([
      {
        updated_at: PRIVACY_DEMO.updated_at,
        code: [{ body: TAG, category: "analytics", vendor: "Example Analytics" }],
      },
    ]);
  });

  it("changes a script's head code in place", async () => {
    signedIn();
    const changes = privacyBlog();
    renderAt("/privacy");
    await openScripts();
    const script = screen.getByRole("listitem", { name: WORDS.script(1) });
    const head = within(script).getByLabelText(WORDS.head);
    expect(head).toHaveValue(PRIVACY_DEMO.code?.[0]?.head);
    await userEvent.clear(head);
    await userEvent.click(head);
    await userEvent.paste(TAG);
    await userEvent.click(screen.getByRole("button", { name: SAVE }));
    expect(await screen.findByText(privacyWords.saved)).toBeVisible();
    expect(changes.at(-1)?.code).toEqual([{ head: TAG }]);
  });

  it("removes the last script and saves an empty list", async () => {
    signedIn();
    const changes = privacyBlog();
    renderAt("/privacy");
    await openScripts();
    await userEvent.click(screen.getByRole("button", { name: WORDS.remove(1) }));
    expect(screen.queryByRole("listitem", { name: WORDS.script(1) })).toBeNull();
    expect(screen.getByText(WORDS.noneLeft)).toBeVisible();
    await userEvent.click(screen.getByRole("button", { name: SAVE }));
    expect(await screen.findByText(privacyWords.saved)).toBeVisible();
    expect(changes.at(-1)?.code).toEqual([]);
  });

  it("leaves out a script added with no code", async () => {
    signedIn();
    const changes = privacyBlog();
    renderAt("/privacy");
    await openScripts();
    await userEvent.click(screen.getByRole("button", { name: WORDS.add }));
    const added = screen.getByRole("listitem", { name: WORDS.script(2) });
    await userEvent.type(within(added).getByLabelText(WORDS.vendor), "Nobody yet");
    await userEvent.click(screen.getByRole("button", { name: SAVE }));
    expect(await screen.findByText(privacyWords.unchanged)).toBeVisible();
    expect(changes).toEqual([]);
  });
});
