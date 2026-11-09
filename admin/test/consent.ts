// The consent proof the Proof tab's tests read: two versions of the banner,
// and answers over two weeks, as the Admin API sends them.
import type { ConsentCounts, ConsentVersions } from "../src/api/analytics";
import type { SiteSettings } from "../src/api/site";
import type { Answers } from "./blog";
import { PRIVACY_DEMO, privacyBlog } from "./privacy";
import { json } from "./render";

/** The first wording, then a new one from 1 November. */
export const VERSIONS: ConsentVersions = {
  versions: [
    {
      version: "0123456789abcdef",
      first_day: "2026-10-01",
      last_day: "2026-10-31",
      shown: { lifetime_months: 6 },
    },
    {
      version: "fedcba9876543210",
      first_day: "2026-11-01",
      last_day: "2026-11-08",
      shown: { lifetime_months: 6, text: { en: { title: "Your choices" } } },
    },
  ],
};

const tally = (choices: number, [preferences, analytics, marketing, external_media]: number[]) => ({
  choices,
  preferences: preferences ?? 0,
  analytics: analytics ?? 0,
  marketing: marketing ?? 0,
  external_media: external_media ?? 0,
});

/** Answers in the weeks of 26 October and 2 November. */
export const COUNTS: ConsentCounts = {
  first: "2026-10-12",
  last: "2026-11-08",
  days: 28,
  totals: tally(11, [3, 8, 1, 6]),
  per_day: [
    { day: "2026-10-26", version: "0123456789abcdef", counts: tally(4, [1, 3, 0, 2]) },
    { day: "2026-10-29", version: "0123456789abcdef", counts: tally(2, [0, 1, 0, 1]) },
    { day: "2026-11-03", version: "fedcba9876543210", counts: tally(5, [2, 4, 1, 3]) },
  ],
};

/** A blog with `site`'s settings and this proof; gives the days each counts request asked for. */
export function proofBlog(site: SiteSettings = PRIVACY_DEMO, counts: ConsentCounts = COUNTS) {
  const asked: (string | null)[] = [];
  const proof: Answers = {
    "GET /consent/versions/": () => json(200, VERSIONS),
    "GET /consent/counts/": (request) => {
      asked.push(new URL(request.url).searchParams.get("days"));
      return json(200, counts);
    },
  };
  privacyBlog(site, proof);
  return asked;
}
