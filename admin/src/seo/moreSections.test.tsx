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

describe("the breadcrumbs and site search parts", () => {
  it("saves a breadcrumb trail through tags, in bold, with only what differs", async () => {
    signedIn();
    const changes = seoBlog();
    await openTab(seoWords.tabs.breadcrumbs);
    const words = seoWords.breadcrumbs;
    expect(screen.getByLabelText(words.category)).toBeChecked();
    expect(await axeViolations()).toEqual([]);
    await userEvent.type(screen.getByLabelText(words.home), "Home");
    await userEvent.click(screen.getByLabelText(words.tag));
    await userEvent.click(screen.getByLabelText(words.bold));
    await save();
    expect(await screen.findByText(seoWords.live)).toBeVisible();
    expect(changes[0]?.seo).toEqual({
      breadcrumbs: { home: "Home", taxonomy: "tag", bold_last: true },
    });
  });

  it("removes the breadcrumbs part when the separator goes back to its default", async () => {
    signedIn();
    const changes = seoBlog({ ...SEO_DEMO, seo: { breadcrumbs: { separator: "»" } } });
    await openTab(seoWords.tabs.breadcrumbs);
    const separator = screen.getByLabelText(seoWords.breadcrumbs.separator);
    expect(separator).toHaveValue("»");
    await userEvent.clear(separator);
    await save();
    expect(await screen.findByText(seoWords.live)).toBeVisible();
    expect(changes[0]?.seo).toEqual({ breadcrumbs: null });
  });

  it("turns the spam filter off and raises the longest search", async () => {
    signedIn();
    const changes = seoBlog();
    await openTab(seoWords.tabs.search);
    const words = seoWords.search;
    expect(screen.getByLabelText(words.symbols)).toBeChecked();
    expect(screen.getByLabelText(words.length)).toHaveValue(50);
    expect(await axeViolations()).toEqual([]);
    await userEvent.clear(screen.getByLabelText(words.length));
    await userEvent.type(screen.getByLabelText(words.length), "80");
    await userEvent.click(screen.getByLabelText(words.spam));
    await save();
    expect(await screen.findByText(seoWords.live)).toBeVisible();
    expect(changes[0]?.seo).toEqual({ search_filter: { max_length: 80, spam: false } });
  });

  it("keeps no limit on the length when site.toml says 0", async () => {
    signedIn();
    const changes = seoBlog({ ...SEO_DEMO, seo: { search_filter: { max_length: 0 } } });
    await openTab(seoWords.tabs.search);
    expect(screen.getByLabelText(seoWords.search.length)).toHaveValue(0);
    await userEvent.click(screen.getByLabelText(seoWords.search.symbols));
    await save();
    expect(await screen.findByText(seoWords.live)).toBeVisible();
    expect(changes[0]?.seo).toEqual({ search_filter: { max_length: 0, symbols: false } });
  });
});

describe("the files for AI tools and local business parts", () => {
  it("saves llms.txt's key pages one per line and the robots.txt line", async () => {
    signedIn();
    const changes = seoBlog();
    await openTab(seoWords.tabs.machines);
    const words = seoWords.machines;
    expect(screen.getByLabelText(words.llms)).toBeChecked();
    expect(screen.getByLabelText(words.robots)).not.toBeChecked();
    expect(await axeViolations()).toEqual([]);
    await userEvent.type(screen.getByLabelText(words.pages), "/about/\n\n/guides/");
    await userEvent.click(screen.getByLabelText(words.robots));
    await save();
    expect(await screen.findByText(seoWords.live)).toBeVisible();
    expect(changes[0]?.seo).toEqual({
      llms_txt: { pages: ["/about/", "/guides/"] },
      schema_map: { robots_txt: true },
    });
  });

  it("turns llms.txt off and keeps its other settings", async () => {
    signedIn();
    const llms_txt = { intro: "Written by the SEO team.", exclude: ["/tag/"] };
    const changes = seoBlog({ ...SEO_DEMO, seo: { llms_txt } });
    await openTab(seoWords.tabs.machines);
    expect(screen.getByLabelText(seoWords.machines.exclude)).toHaveValue("/tag/");
    await userEvent.click(screen.getByLabelText(seoWords.machines.llms));
    await save();
    expect(await screen.findByText(seoWords.live)).toBeVisible();
    expect(changes[0]?.seo).toEqual({ llms_txt: { ...llms_txt, off: true } });
  });

  it("names the locations and turns the KML file off", async () => {
    signedIn();
    const locations = [{ name: "Mill Lane Dental" }, { name: "Kirkgate Dental" }];
    const changes = seoBlog({ ...SEO_DEMO, seo: { locations } });
    await openTab(seoWords.tabs.local);
    const words = seoWords.local;
    expect(screen.getByText(words.listed(["Mill Lane Dental", "Kirkgate Dental"]))).toBeVisible();
    expect(await axeViolations()).toEqual([]);
    await userEvent.click(screen.getByLabelText(words.kml));
    await save();
    expect(await screen.findByText(seoWords.live)).toBeVisible();
    expect(changes[0]?.seo).toEqual({ local: { kml: false } });
  });

  it("says when site.toml lists no locations", async () => {
    signedIn();
    seoBlog();
    await openTab(seoWords.tabs.local);
    expect(screen.getByText(seoWords.local.none)).toBeVisible();
  });
});
