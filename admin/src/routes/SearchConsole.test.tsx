import { screen, within } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { axeViolations, json, renderAt, signedIn } from "../../test/render";
import { followUps, NOT_SET_UP, searchConsoleBlog } from "../../test/searchConsole";
import { searchConsoleWords } from "../strings/searchConsoleWords";

const words = searchConsoleWords;
const ROUTE = "GET /search-console/follow-ups/";
const REPORT = "GET /search-console/report/";

describe("the Search Console screen", () => {
  it("leads with the searches no heading answers, then each post's searches", async () => {
    signedIn();
    searchConsoleBlog();
    renderAt("/");
    await userEvent.click(await screen.findByRole("link", { name: words.title }));
    expect(await screen.findByRole("heading", { level: 1, name: words.title })).toBeVisible();
    const focus = await screen.findByRole("region", { name: words.open.title });
    expect(focus).toHaveTextContent(words.open.count(2, 3));
    expect(focus).toHaveTextContent(words.open.sentence("1 October 2026", "28 October 2026"));
    const links = screen.getByRole("table", { name: "Why internal links matter" });
    const open = within(links).getByRole("row", { name: /how many internal links/ });
    expect(open).toHaveTextContent(words.posts.noHeading);
    expect(open).toHaveTextContent("60");
    expect(open).toHaveTextContent("8.2");
    const answered = within(links).getByRole("row", { name: /how do search engines/ });
    expect(answered).toHaveTextContent('"Search engines use them to find and rank pages"');
    const post = screen.getByRole("region", { name: "Why internal links matter" });
    expect(within(post).getByRole("link", { name: "Why internal links matter" })).toHaveAttribute(
      "href",
      "https://www.example.com/blog/why-internal-links-matter/",
    );
    const edit = within(post).getByRole("link", { name: words.edit("Why internal links matter") });
    expect(edit).toHaveAttribute("href", "/admin/posts/en.why-internal-links-matter");
    expect(screen.getByRole("link", { name: words.title })).toHaveAttribute("aria-current", "page");
    expect(screen.getAllByRole("heading", { level: 1 })).toHaveLength(1);
    expect(await axeViolations()).toEqual([]);
  });

  it("shows the week, the posts to fix and each post's month, linked to the editor", async () => {
    signedIn();
    const sent = searchConsoleBlog();
    renderAt("/search-console");
    const week = await screen.findByRole("region", { name: words.week.title });
    expect(week).toHaveTextContent(words.week.clicks("84"));
    expect(week).toHaveTextContent(words.week.sentence("28 October 2026", "96"));
    expect(within(week).queryByRole("heading", { name: words.week.gained })).toBeNull();
    const losers = within(week).getByRole("region", { name: words.week.lost });
    expect(losers).toHaveTextContent(
      "What a sitemap does, and what it does not: 9 clicks, from 30.",
    );
    expect(within(losers).getByRole("link", { name: words.goDeeper })).toHaveAttribute(
      "href",
      "https://app.serpdad.com/go-deeper?question=why-it-dropped",
    );

    const falling = screen.getByRole("region", { name: words.falling.title });
    expect(falling).toHaveTextContent(": 90, 70, 50, then 40 clicks a month.");
    const refresh = within(falling).getByRole("link", { name: /What a sitemap does/ });
    expect(refresh).toHaveAttribute("href", "/admin/posts/en.what-a-sitemap-does");

    const lost = screen.getByRole("region", { name: words.lost.title });
    expect(lost).toHaveTextContent(words.lost.missing("31"));
    expect(within(lost).getByRole("link", { name: words.lost.redirect })).toHaveAttribute(
      "href",
      "/admin/redirects",
    );
    expect(lost).toHaveTextContent(words.lost.noindex);

    const month = screen.getByRole("table", { name: words.month.title });
    const links = within(month).getByRole("row", { name: /Why internal links matter/ });
    expect(links).toHaveTextContent("2,400");
    expect(links).toHaveTextContent("5.0%");
    expect(links).toHaveTextContent("3.1");
    const editor = within(links).getByRole("link", { name: "Why internal links matter" });
    expect(editor).toHaveAttribute("href", "/admin/posts/en.why-internal-links-matter");
    const unseen = within(month).getByRole("row", { name: /What a sitemap does/ });
    expect(unseen).toHaveTextContent(words.month.notShown);
    const asked = sent.map((request) => new URL(request.url).pathname);
    expect(asked.filter((path) => path.endsWith("/search-console/report/"))).toHaveLength(1);
    expect(await axeViolations()).toEqual([]);
  });

  it("shows the report's own problem below the questions when only the report fails", async () => {
    signedIn();
    const busy = { ...NOT_SET_UP, title: "Google is busy", status: 502, fix: "Wait a minute." };
    searchConsoleBlog({ [REPORT]: () => json(502, busy) });
    renderAt("/search-console");
    const alert = await screen.findByRole("alert");
    expect(alert).toHaveTextContent("Wait a minute.");
    expect(screen.getByRole("region", { name: words.open.title })).toBeVisible();
    expect(screen.queryByRole("region", { name: words.week.title })).toBeNull();
  });

  it("says so when every long search has a heading, and when there are none", async () => {
    signedIn();
    const answered = followUps();
    for (const post of answered.posts) {
      for (const search of post.searches) {
        search.answered_by = "A heading";
      }
    }
    searchConsoleBlog({ [ROUTE]: () => json(200, answered) });
    const { unmount } = renderAt("/search-console");
    expect(await screen.findByText(words.open.allAnswered(3))).toBeVisible();
    expect(screen.queryByText(words.posts.noHeading)).toBeNull();
    unmount();

    searchConsoleBlog({ [ROUTE]: () => json(200, followUps({ posts: [] })) });
    renderAt("/search-console");
    const none = words.open.none(8, "1 October 2026", "28 October 2026");
    expect(await screen.findByText(none)).toBeVisible();
    expect(screen.queryByRole("heading", { name: words.posts.title })).toBeNull();
  });

  it("shows the API's words once when the blog is not connected", async () => {
    signedIn();
    searchConsoleBlog({ [ROUTE]: () => json(409, NOT_SET_UP) });
    renderAt("/search-console");
    const alert = await screen.findByRole("alert");
    expect(alert).toHaveTextContent(NOT_SET_UP.title);
    expect(alert).toHaveTextContent(NOT_SET_UP.fix);
    expect(screen.getAllByRole("alert")).toHaveLength(1);
    expect(screen.queryByRole("table")).toBeNull();
  });
});
