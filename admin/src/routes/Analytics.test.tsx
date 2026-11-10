import { screen, waitFor, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { aiBots, aiVisitors, analyticsBlog, uncited, visitors } from "../../test/analytics";
import { axeViolations, json, renderAt, signedIn } from "../../test/render";
import { en } from "../strings/en";
import { searchTotalsWords as searchWords } from "../strings/searchTotalsWords";

const words = en.analytics;

describe("the Analytics screen", () => {
  it("shows the week's page views, their sources and what AI did with the blog", async () => {
    signedIn();
    analyticsBlog();
    renderAt("/");
    await userEvent.click(await screen.findByRole("link", { name: en.nav.analytics }));
    expect(await screen.findByRole("heading", { level: 1, name: words.title })).toBeVisible();
    const total = await screen.findByRole("region", { name: words.total(7) });
    expect(total).toHaveTextContent("321");
    expect(total).toHaveTextContent(words.rise(12, 7, "287"));
    expect(screen.getByText(words.chart.summary("0", "63", "27 October 2026"))).toBeVisible();
    const sources = screen.getByRole("table", { name: words.sources.title });
    const search = within(sources).getByRole("row", { name: /Search engines/ });
    expect(search).toHaveTextContent("125");
    expect(search).toHaveTextContent("108");
    const pages = screen.getByRole("table", { name: words.pages.title });
    const top = within(pages).getByRole("link", { name: "/blog/why-internal-links-matter/" });
    expect(top).toHaveAttribute("href", "/blog/why-internal-links-matter/");
    const tools = screen.getByRole("table", { name: words.tools.title });
    expect(within(tools).getByRole("row", { name: /ChatGPT/ })).toHaveTextContent("21");
    expect(within(tools).getByRole("row", { name: /Claude/ })).toHaveTextContent(
      words.tools.noPage,
    );
    const bots = screen.getByRole("table", { name: words.bots.title });
    const openai = within(bots).getByRole("row", { name: /ChatGPT/ });
    expect(openai).toHaveTextContent("412");
    expect(openai).toHaveTextContent("GPTBot 301, OAI-SearchBot 111");
    expect(within(bots).getAllByRole("row").at(-1)).toHaveTextContent("CCBot 92");
    const quiet = screen.getByRole("table", { name: words.uncited.title });
    expect(quiet).toHaveTextContent("/blog/what-a-sitemap-does/");
    expect(quiet).toHaveTextContent("ClaudeBot, GPTBot");
    const google = await screen.findByRole("table", { name: searchWords.title });
    expect(within(google).getByRole("row", { name: /^Clicks/ })).toHaveTextContent("412");
    expect(within(google).getByRole("row", { name: /^Clicked/ })).toHaveTextContent("4.2%");
    const position = within(google).getByRole("row", { name: /^Average position/ });
    expect(position).toHaveTextContent("7.4");
    expect(position).toHaveTextContent(searchWords.notShown);
    expect(screen.getByText(searchWords.late(7))).toBeVisible();
    expect(screen.getByRole("link", { name: en.nav.analytics })).toHaveAttribute(
      "aria-current",
      "page",
    );
    expect(screen.getAllByRole("heading", { level: 1 })).toHaveLength(1);
    expect(await axeViolations()).toEqual([]);
  });

  it("reads every report again for the days picked", async () => {
    signedIn();
    const sent = analyticsBlog();
    renderAt("/analytics");
    await userEvent.selectOptions(await screen.findByLabelText(words.days), words.period(90));
    await userEvent.click(screen.getByRole("button", { name: words.show }));
    await waitFor(() => {
      const asked = sent.map((request) => new URL(request.url).searchParams.get("days"));
      expect(asked.filter((days) => days === "90")).toHaveLength(4);
    });
  });

  it("tells a blog not connected to Search Console where to connect, calmly", async () => {
    signedIn();
    const notSetUp = {
      type: "https://khalid.dev/problems/not-set-up",
      title: "Search Console is not set up",
      status: 409,
      detail: "This site is not connected to Google Search Console.",
      fix: "Follow the steps in docs/deploy.md.",
    };
    analyticsBlog({ "GET /search-console/totals/": () => json(409, notSetUp) });
    renderAt("/analytics");
    const connect = await screen.findByRole("link", { name: searchWords.connect });
    expect(connect).toHaveAttribute("href", "/admin/search-console");
    expect(screen.queryByRole("alert")).toBeNull();
    expect(screen.queryByRole("table", { name: searchWords.title })).toBeNull();
  });

  it("keeps the page views on screen when Google gives no usable answer", async () => {
    signedIn();
    const quota = {
      type: "https://khalid.dev/problems/upstream",
      title: "Google gave no usable answer",
      status: 502,
      detail: "Google's quota for these requests is spent for now.",
      fix: "Wait a minute, or until tomorrow for the daily quota, then try again.",
    };
    analyticsBlog({ "GET /search-console/totals/": () => json(502, quota) });
    renderAt("/analytics");
    const google = await screen.findByRole("region", { name: searchWords.title });
    expect(await within(google).findByRole("alert")).toHaveTextContent(quota.title);
    expect(screen.getByRole("region", { name: words.total(7) })).toHaveTextContent("321");
  });

  it("says plainly when nothing was counted", async () => {
    signedIn();
    const none = { views: 0, before: 0, arrivals: 0, pages: [] };
    const quiet = visitors({ ...none, per_day: [{ day: "2026-10-31", views: 0 }] });
    analyticsBlog({
      "GET /analytics/visitors/": () => json(200, quiet),
      "GET /analytics/ai-visitors/": () => json(200, aiVisitors({ ai_views: 0, tools: [] })),
      "GET /analytics/uncited/": () => json(200, uncited({ pages: [] })),
      "GET /analytics/ai-bots/": () => json(200, aiBots({ tools: [] })),
    });
    renderAt("/analytics");
    expect(await screen.findByText(words.level(7))).toBeVisible();
    expect(screen.getByText(words.chart.none)).toBeVisible();
    expect(screen.getByText(words.pages.none)).toBeVisible();
    expect(screen.getByText(words.tools.none)).toBeVisible();
    expect(screen.getByText(words.bots.none)).toBeVisible();
    expect(screen.getByText(words.uncited.none)).toBeVisible();
    expect(screen.queryByText(words.chart.table)).toBeNull();
  });

  it("shows the API's words once when the key may not read visitor data", async () => {
    signedIn();
    const lacks = {
      type: "https://khalid.dev/problems/forbidden",
      title: "This key cannot read visitor data.",
      status: 403,
      detail: "Analytics need the settings:write scope.",
      fix: "Sign in with an owner or admin staff token.",
    };
    analyticsBlog({
      "GET /analytics/visitors/": () => json(403, lacks),
      "GET /analytics/ai-visitors/": () => json(403, lacks),
      "GET /analytics/uncited/": () => json(403, lacks),
      "GET /analytics/ai-bots/": () => json(403, lacks),
    });
    renderAt("/analytics");
    expect(await screen.findByRole("alert")).toHaveTextContent(lacks.title);
    expect(screen.getAllByRole("alert")).toHaveLength(1);
  });
});
