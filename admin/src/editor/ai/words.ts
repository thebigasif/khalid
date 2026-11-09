// The words of the editor's AI suggestions. Kept beside them, so this
// package adds to the admin without changing its shared strings file.

/** Every label, hint and status line the AI suggestions show. */
export const aiWords = {
  /** The mark on every AI button, so a writer always knows what writes it. */
  badge: "AI",
  suggest: "Suggest",
  suggestField: (field: string) => `Suggest ${field} with AI`,
  fix: "Suggest a fix",
  fixLabel: (what: string) => `Suggest a fix with AI for: ${what}`,
  summarize: "Summarize",
  summarizeLabel: "Summarize the post as key takeaways with AI",
  alt: "Suggest alt text",
  altLabel: "Suggest alt text for this picture with AI",
  asking: "Writing suggestions. This takes a few seconds.",
  intro: "Pick one to use it. Nothing in the post changes until you do.",
  fixIntro: "Copy the one that fits, then change the post. Nothing changes on its own.",
  takeawaysIntro:
    "These points go in a key takeaways box where you add it. Readers see it as a short summary.",
  use: "Use this",
  copy: "Copy",
  copied: "Copied.",
  add: "Add to the end of the post",
  added:
    "Added at the end of the post, in Markdown. Move it under the first paragraph if you like.",
  close: "Close",
  characters: (count: number) => (count === 1 ? "1 character" : `${count} characters`),
  fits: "a good length.",
  fields: {
    seoTitle: "an SEO title",
    excerpt: "a meta description",
    xTitle: "a title for X",
    xDescription: "a description for X",
    facebookTitle: "a title for Facebook",
    facebookDescription: "a description for Facebook",
  },
} as const;

/** A field the editor can suggest words for. */
export type SuggestField = keyof typeof aiWords.fields;
