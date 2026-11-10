import type { FollowUp, FollowUps, SearchReport } from "../src/api/searchConsole";
import type { Answers } from "./redirects";
import { json, serve } from "./render";

/** A long search Google shared, answered under `heading` or by no heading. */
function search(query: string, impressions: number, heading: string | null): FollowUp {
  return { query, clicks: 2, impressions, position: 8.2, answered_by: heading };
}

/** The follow-ups of the month ending 28 October 2026, as the Admin API sends them. */
export function followUps(fields: Partial<FollowUps> = {}): FollowUps {
  return {
    property: "sc-domain:example.com",
    first: "2026-10-01",
    last: "2026-10-28",
    words: 8,
    posts: [
      {
        id: "en.why-internal-links-matter",
        title: "Why internal links matter",
        url: "https://www.example.com/blog/why-internal-links-matter/",
        searches: [
          search("how many internal links should a blog post have", 60, null),
          search(
            "how do search engines find and rank pages on a site",
            40,
            "Search engines use them to find and rank pages",
          ),
        ],
      },
      {
        id: "en.what-a-sitemap-does",
        title: "What a sitemap does, and what it does not",
        url: "https://www.example.com/blog/what-a-sitemap-does/",
        searches: [search("do i need a sitemap if my blog has only ten posts", 35, null)],
      },
    ],
    ...fields,
  };
}

const SITEMAP = "https://www.example.com/blog/what-a-sitemap-does/";
const GO_DEEPER = "https://app.serpdad.com/go-deeper?question=why-it-dropped";

/** The report of the same month: two posts, one falling, two lost pages and one loser. */
export function report(fields: Partial<SearchReport> = {}): SearchReport {
  return {
    property: "sc-domain:example.com",
    first: "2026-10-01",
    last: "2026-10-28",
    posts: [
      {
        id: "en.why-internal-links-matter",
        title: "Why internal links matter",
        url: "https://www.example.com/blog/why-internal-links-matter/",
        clicks: 120,
        impressions: 2400,
        ctr: 0.05,
        position: 3.1,
      },
      {
        id: "en.what-a-sitemap-does",
        title: "What a sitemap does, and what it does not",
        url: SITEMAP,
        clicks: 0,
        impressions: 0,
        ctr: 0,
        position: null,
      },
    ],
    decaying: [
      {
        id: "en.what-a-sitemap-does",
        title: "What a sitemap does, and what it does not",
        url: SITEMAP,
        clicks: [90, 70, 50, 40],
        action: "Refresh this post",
        go_deeper: GO_DEEPER,
      },
    ],
    lost: [
      {
        url: "https://www.example.com/blog/old-sitemap-guide/",
        clicks: 12,
        impressions: 300,
        why: "missing",
        not_found_hits: 31,
      },
      {
        url: "https://www.example.com/blog/draft-checklist/",
        clicks: 3,
        impressions: 90,
        why: "noindex",
        not_found_hits: null,
      },
    ],
    week_clicks: 84,
    last_week_clicks: 96,
    winners: [],
    losers: [
      {
        id: "en.what-a-sitemap-does",
        title: "What a sitemap does, and what it does not",
        url: SITEMAP,
        before: 30,
        now: 9,
        go_deeper: GO_DEEPER,
      },
    ],
    ...fields,
  };
}

/** What a blog that did not opt in to Search Console answers. */
export const NOT_SET_UP = {
  type: "https://khalid.dev/problems/not-set-up",
  title: "This is not set up yet",
  status: 409,
  detail:
    "This site is not connected to Google Search Console, Google's report of how a site does in search. Khalid asks Google nothing until site.toml opts in under [search_console] and the server has the owner's Google sign-in.",
  fix: "Follow the steps in docs/deploy.md under Google Search Console, then restart khalid serve.",
  rule: "search_console",
};

// What the Search Console screen reads when it opens, the month above unless a test says otherwise.
const MONTH: Answers = {
  "GET /search-console/follow-ups/": () => json(200, followUps()),
  "GET /search-console/report/": () => json(200, report()),
};

/** A blog that answers the Search Console route from `answers`, and the requests it was sent. */
export function searchConsoleBlog(answers: Answers = {}) {
  const sent: Request[] = [];
  const all = { ...MONTH, ...answers };
  serve((request) => {
    sent.push(request);
    const route = new URL(request.url).pathname.replace("/api/admin/v1", "");
    return all[`${request.method} ${route}`]?.(request) ?? json(404, {});
  });
  return sent;
}
