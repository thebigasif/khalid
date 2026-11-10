import { en } from "./en";

/** `count` of `thing`, with an s when there is not exactly one. */
const plural = (count: number, thing: string) =>
  count === 1 ? `1 ${thing}` : `${count} ${thing}s`;

// The words of the dashboard, the first screen after signing in.
export const dashboardWords = {
  greeting: (name: string | null) =>
    name === null ? en.dashboard.title : `${en.dashboard.title} to ${name}`,
  numbers: "Your blog in numbers",
  live: "Live posts",
  liveLink: "See live posts",
  drafts: "Drafts",
  draftsLink: "See every post",
  failing: "Failing checks",
  failingLink: "Fix them",
  allPass: "Every post passes its checks.",
  visits: "Visits, last 28 days",
  visitsLink: "See analytics",
  searchClicks: "Search clicks",
  connectWhy: "Connect Search Console to see how people find your posts on Google.",
  connect: "Connect Search Console",
  health: "Site health",
  healthScore: (score: number) => `${score} / 100`,
  healthWhat: "The share of your pages with no error, from the daily site check.",
  healthNone:
    "No site check has run yet. Your blog checks every page once a day, the first time an hour after it starts.",
  healthFound: (errors: number, warnings: number, pages: number) =>
    `${plural(errors, "error")} and ${plural(warnings, "warning")} on ${plural(pages, "page")}.`,
  healthSince: (change: number, since: string) => {
    if (change === 0) {
      return `The same as on ${since}.`;
    }
    return change > 0 ? `Up ${change} since ${since}.` : `Down ${-change} since ${since}.`;
  },
  attention: "Needs attention",
  failsChecks: "Fails checks",
  scheduled: (count: number) =>
    count === 1 ? "1 post is scheduled to go live" : `${count} posts are scheduled to go live`,
  waiting: (count: number) => {
    if (count >= 100) {
      return "100 or more comments are waiting for your yes";
    }
    return count === 1
      ? "1 comment is waiting for your yes"
      : `${count} comments are waiting for your yes`;
  },
  waitingPill: "Waiting",
  allClear: "Nothing needs you right now.",
  startPost: "Start a new post",
  recentDrafts: "Recent drafts",
  noDrafts: "No drafts. Every post is live or scheduled.",
  quick: "Quick actions",
  quickWrite: "Write a post",
  quickMedia: "Add a picture or file",
  quickComments: "Answer comments",
  quickLook: "Change the look",
  issues: "Fix these first",
  issuesLede: "The checks that fail on most posts. Fixing one helps every post it names.",
  onPosts: (count: number) => (count === 1 ? "1 post" : `${count} posts`),
  start: "Start here",
  startOn: (what: string) => `Start on: ${what}`,
} as const;
