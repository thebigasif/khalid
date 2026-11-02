import { fireEvent, screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { axeViolations, json, renderAt, serve, signedIn } from "../../test/render";
import { listing, row, rowsPage } from "../../test/rows";
import { en } from "../strings/en";
import { shellWords } from "../strings/shellWords";

/** A blog whose post list holds one post, and which answers 404 to everything else. */
function blog() {
  serve((request) => (listing(request) === null ? json(404, {}) : rowsPage([row()])));
}

async function openWithKeys() {
  await screen.findByRole("button", { name: shellWords.search.label });
  await userEvent.keyboard("{Control>}k{/Control}");
  return screen.getByRole("dialog", { name: shellWords.search.label });
}

describe("the search in the top bar", () => {
  it("opens on Ctrl K with the cursor in it and the latest posts below", async () => {
    signedIn();
    blog();
    renderAt("/media");
    const search = await openWithKeys();
    expect(within(search).getByLabelText(shellWords.search.label)).toHaveFocus();
    const latest = await within(search).findByRole("list", { name: shellWords.search.latest });
    const post = within(latest).getByRole("link", { name: "Why internal links matter" });
    expect(post).toHaveAttribute("href", "/admin/posts/01JA");
    expect(await axeViolations()).toEqual([]);
  });

  it("finds a screen by its name and opens the first match on Enter", async () => {
    signedIn();
    blog();
    renderAt("/posts");
    const search = await openWithKeys();
    await userEvent.type(within(search).getByLabelText(shellWords.search.label), "red");
    const screens = within(search).getByRole("list", { name: shellWords.search.screens });
    expect(within(screens).getByRole("link", { name: en.nav.redirects })).toBeVisible();
    await userEvent.keyboard("{Enter}");
    const heading = await screen.findByRole("heading", { level: 1, name: en.redirects.title });
    expect(heading).toBeVisible();
    expect(screen.queryByRole("dialog", { name: shellWords.search.label })).toBeNull();
  });

  it("closes on Escape and gives the focus back to its button", async () => {
    signedIn();
    blog();
    renderAt("/media");
    await userEvent.click(await screen.findByRole("button", { name: shellWords.search.label }));
    expect(screen.getByRole("dialog", { name: shellWords.search.label })).toBeVisible();
    await userEvent.keyboard("{Escape}");
    expect(screen.queryByRole("dialog", { name: shellWords.search.label })).toBeNull();
    expect(screen.getByRole("button", { name: shellWords.search.label })).toHaveFocus();
  });

  it("leaves Ctrl K to the editor's text, where it links words", async () => {
    signedIn();
    blog();
    renderAt("/media");
    await screen.findByRole("button", { name: shellWords.search.label });
    const text = document.createElement("div");
    text.setAttribute("contenteditable", "true");
    document.body.append(text);
    fireEvent.keyDown(text, { key: "k", ctrlKey: true });
    expect(screen.queryByRole("dialog", { name: shellWords.search.label })).toBeNull();
    text.remove();
  });
});
