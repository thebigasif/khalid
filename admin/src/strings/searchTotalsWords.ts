// Google search's numbers on the Analytics screen, kept beside en.ts so that file stays short.
export const searchTotalsWords = {
  title: "From Google search",
  lede: "What Google Search Console, Google's report of how your blog does in search, counted for all your pages together. Khalid asks Google with your own Google account, so these numbers can take a few seconds.",
  loading: "Asking Google",
  measure: "Number",
  period: (first: string, last: string) => `${first} to ${last}`,
  before: (days: number) => `The ${days} days before`,
  clicks: "Clicks",
  shown: "Shown",
  clicked: "Clicked",
  position: "Average position",
  notShown: "Not shown",
  terms: [
    {
      term: "Clicks",
      meaning: "Times someone clicked from Google's results to one of your pages.",
    },
    { term: "Shown", meaning: "Times Google showed one of your pages in its results." },
    { term: "Clicked", meaning: "The share of those times someone clicked through." },
    {
      term: "Average position",
      meaning: "Where your pages stood in Google's results, 1 being the top.",
    },
  ],
  late: (days: number) =>
    `Google needs about three days to make a day's numbers final, so these ${days} days end three days before the page views above.`,
  notConnected:
    "Google search numbers show here once the blog is connected to Search Console. Until then, Khalid asks Google nothing.",
  connect: "See how to connect Search Console",
};
