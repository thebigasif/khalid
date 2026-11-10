import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { axeViolations, renderAt, signedIn } from "../../test/render";
import { SEO_DEMO, seoBlog } from "../../test/seo";
import { seoWords } from "../strings/seoWords";

const WORDS = seoWords.appearance;

describe("the SEO settings screen", () => {
  it("opens from the side menu on search appearance, with a live search result", async () => {
    signedIn();
    seoBlog();
    renderAt("/");
    await userEvent.click(await screen.findByRole("link", { name: seoWords.title }));
    expect(await screen.findByRole("heading", { name: seoWords.tabs.appearance })).toBeVisible();
    expect(screen.getByRole("radio", { name: "·" })).toBeChecked();
    const preview = screen.getByRole("region", { name: WORDS.preview });
    expect(preview).toHaveTextContent("Choosing anchor text · Field Notes on SEO");
    expect(await axeViolations()).toEqual([]);
  });

  it("fills the preview as a template changes, and adds a variable by its button", async () => {
    signedIn();
    seoBlog();
    renderAt("/seo");
    await userEvent.selectOptions(
      await screen.findByLabelText(WORDS.type),
      seoWords.types.category,
    );
    const title = screen.getByLabelText(WORDS.titleField);
    expect(title).toHaveValue("");
    expect(title).toHaveAccessibleDescription(WORDS.defaultIs("{title} {sep} {site}"));
    await userEvent.type(title, "All about");
    const variables = screen.getByRole("group", { name: WORDS.addTo("title template") });
    const add = variables.querySelector("button");
    expect(add).toHaveTextContent(seoWords.variables.title);
    if (add !== null) {
      await userEvent.click(add);
    }
    expect(title).toHaveValue("All about {title}");
    const preview = screen.getByRole("region", { name: WORDS.preview });
    expect(preview).toHaveTextContent("All about Technical SEO");
  });

  it("saves the whole titles part, and nothing when nothing changed", async () => {
    signedIn();
    const changes = seoBlog();
    renderAt("/seo");
    await userEvent.click(await screen.findByRole("button", { name: seoWords.save }));
    expect(await screen.findByText(seoWords.unchanged)).toBeVisible();
    await userEvent.click(screen.getByRole("radio", { name: "»" }));
    await userEvent.type(screen.getByLabelText(WORDS.tagline), "Plain notes on search");
    await userEvent.click(screen.getByRole("button", { name: seoWords.save }));
    expect(await screen.findByText(seoWords.live)).toBeVisible();
    expect(changes).toEqual([
      {
        updated_at: SEO_DEMO.updated_at,
        seo: {
          titles: {
            separator: "»",
            tagline: "Plain notes on search",
            post: { title: "{title} {sep} {site}" },
          },
        },
      },
    ]);
    expect(screen.queryByText(seoWords.restart)).toBeNull();
  });
});
