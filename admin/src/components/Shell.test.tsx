import { screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { adminBlog } from "../../test/blog";
import { axeViolations, renderAt, signedIn } from "../../test/render";
import { en } from "../strings/en";
import { shellWords } from "../strings/shellWords";

describe("the frame around every screen", () => {
  it("groups the side menu and marks the screen you are on", async () => {
    signedIn();
    adminBlog({});
    renderAt("/posts");
    const menu = await screen.findByRole("navigation", { name: en.nav.label });
    for (const group of Object.values(shellWords.groups)) {
      expect(within(menu).getByRole("list", { name: group })).toBeVisible();
    }
    const write = within(menu).getByRole("list", { name: shellWords.groups.write });
    const posts = within(write).getByRole("link", { name: en.nav.posts });
    expect(posts).toHaveAttribute("aria-current", "page");
    expect(within(menu).getByRole("link", { name: en.nav.dashboard })).not.toHaveAttribute(
      "aria-current",
    );
    expect(await axeViolations()).toEqual([]);
  });

  it("names where you are in the top bar", async () => {
    signedIn();
    adminBlog({});
    renderAt("/team/owner");
    const bar = await screen.findByRole("banner");
    expect(within(bar).getByText(shellWords.groups.settings)).toBeInTheDocument();
    expect(within(bar).getByText(en.nav.team)).toBeInTheDocument();
  });

  it("keeps the menu behind a button on a small screen, and gives the focus back", async () => {
    signedIn();
    adminBlog({});
    renderAt("/posts");
    const opener = await screen.findByRole("button", { name: shellWords.openMenu });
    const menu = screen.getByRole("navigation", { name: en.nav.label });
    expect(opener).toHaveAttribute("aria-expanded", "false");
    expect(opener).toHaveAttribute("aria-controls", menu.id);
    expect(menu).toHaveAttribute("data-open", "false");
    await userEvent.click(opener);
    expect(opener).toHaveAttribute("aria-expanded", "true");
    expect(menu).toHaveAttribute("data-open", "true");
    expect(screen.getByRole("button", { name: shellWords.closeMenu })).toHaveFocus();
    await userEvent.keyboard("{Escape}");
    expect(opener).toHaveAttribute("aria-expanded", "false");
    expect(opener).toHaveFocus();
  });

  it("closes the menu once a screen is chosen", async () => {
    signedIn();
    adminBlog({});
    renderAt("/posts");
    await userEvent.click(await screen.findByRole("button", { name: shellWords.openMenu }));
    const menu = screen.getByRole("navigation", { name: en.nav.label });
    await userEvent.click(within(menu).getByRole("link", { name: en.nav.media }));
    expect(menu).toHaveAttribute("data-open", "false");
    expect(await screen.findByRole("heading", { level: 1, name: en.media.title })).toBeVisible();
  });
});
