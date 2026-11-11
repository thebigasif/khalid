// The words of the Look screen, where the blog's template is chosen.
// en.ts carries them as en.look; they sit in their own file so en.ts
// stays short.
import { LIVE, WAITS } from "./restartWords";

export const lookWords = {
  title: "Look",
  lede: "A template sets how your blog's pages are laid out: what the home page leads with, and what sits around a post. Your colours, fonts and logo stay the same in every template. Your choice is saved to site.toml, and the blog uses it within a few seconds.",
  loading: "Reading the templates",
  choose: "Choose a template",
  inUse: "In use",
  turnsOn: "Turns on",
  turnsOff: "Turns off",
  optionsFor: (name: string) => `Options for ${name}`,
  optionsHint:
    "Each option starts at the template's own choice. Change only what you want different.",
  noOptions: "This template has no options. It looks the same on every blog, in your colours.",
  linesHint: (most: number) => `One per line, up to ${most}.`,
  cta: {
    title: "Your call to action",
    about:
      "A call to action is the one thing you ask readers to do, such as start a trial. Write it once here and every page that shows it uses the same button. Empty all three lines to remove it.",
    shows:
      "This template shows it as a band after posts or on the front page, and as a button in the header. The options above say where.",
    blockOnly:
      "This template shows it only where a post or page has a ::cta line. Pick a template with a call to action band to show it on every post.",
    label: "Words on the button",
    labelHint: "A few words, such as Start a free trial.",
    href: "Where the button goes",
    hrefHint:
      "A full address, such as https://example.com/signup, or a path on your blog, such as /blog/start/.",
    note: "Line beside the button",
    noteHint: "Optional. One short line, such as No card needed.",
  },
  save: "Save the look",
  saved: "Saved to site.toml.",
  live: LIVE,
  unchanged: "Nothing to save: this is the look your blog has.",
  restart: WAITS,
  // What each reader feature a template turns on or off is, in a few words.
  // A key not listed here shows as its own words.
  reader: {
    byline: "Each author's photo, name and job above a post",
    reviewed_by: "Who reviewed a post, in its byline",
    reading_time: "How long a post takes to read",
    table_of_contents: "The contents of a long post",
    heading_links: "A link to each heading of a post",
    enlarge_images: "A link from each picture to its largest size",
    author_box: "The author box at the end of a post",
    related_posts: "Related posts under a post",
    rich_cards: "The author, date and category on each post card",
    topic_links: "A row of category links on the home page",
    authors_page: "The page that lists every author",
    archive: "The archive page of every post",
    search_shortcut: "The search link in the header",
    linked_mentions: "The posts that link to a post, under it",
    link_previews: "A linked post's title when a reader points at the link",
  } as Record<string, string>,
} as const;
