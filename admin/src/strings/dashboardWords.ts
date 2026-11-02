import { en } from "./en";

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
} as const;
