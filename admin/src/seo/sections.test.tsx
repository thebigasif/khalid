import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { axeViolations, renderAt, signedIn } from "../../test/render";
import { SEO_DEMO, seoBlog } from "../../test/seo";
import { seoWords } from "../strings/seoWords";

/** Opens the SEO settings on the tab named `tab`. */
async function openTab(tab: string) {
  renderAt("/seo");
  await userEvent.click(await screen.findByRole("button", { name: tab }));
}

const save = () => userEvent.click(screen.getByRole("button", { name: seoWords.save }));

describe("the parts of the SEO settings", () => {
  it("writes only archives that differ from the defaults, and the blog's switch", async () => {
    signedIn();
    const changes = seoBlog();
    await openTab(seoWords.tabs.index);
    expect(screen.getByLabelText(seoWords.index.archive.tags)).toBeChecked();
    expect(screen.getByLabelText(seoWords.index.archive.dates)).not.toBeChecked();
    expect(await axeViolations()).toEqual([]);
    await userEvent.click(screen.getByLabelText(seoWords.index.archive.dates));
    await userEvent.click(screen.getByLabelText(seoWords.index.site));
    await save();
    expect(await screen.findByText(seoWords.saved)).toBeVisible();
    expect(changes[0]?.seo).toEqual({ index: { dates: "index", site: "noindex" } });
  });

  it("saves a person with one profile per line, and keeps the logo's size", async () => {
    signedIn();
    const publisher = { logo: { url: "https://www.example.com/me.png", alt: "Asif", width: 400 } };
    const changes = seoBlog({ ...SEO_DEMO, seo: { publisher } });
    await openTab(seoWords.tabs.representation);
    await userEvent.click(screen.getByLabelText(seoWords.representation.person));
    expect(screen.queryByLabelText(seoWords.representation.founded)).toBeNull();
    await userEvent.type(
      screen.getByLabelText(seoWords.representation.profiles),
      "https://www.linkedin.com/in/asif\n\nhttps://www.youtube.com/@serpdad",
    );
    await save();
    expect(await screen.findByText(seoWords.saved)).toBeVisible();
    expect(changes[0]?.seo).toEqual({
      publisher: {
        type: "person",
        logo: { url: "https://www.example.com/me.png", alt: "Asif", width: 400 },
        same_as: ["https://www.linkedin.com/in/asif", "https://www.youtube.com/@serpdad"],
      },
    });
  });
});
