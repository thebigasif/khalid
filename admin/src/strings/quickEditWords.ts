// The words of quick edit in the posts table (brief G52, plan 4.6). The admin's
// other words are in en.ts.

export const quickEditWords = {
  open: (title: string) => `Quick edit ${title}`,
  openShort: "Quick edit",
  label: (title: string) => `Quick edit of ${title}`,
  intro:
    "Change the basics here without opening the editor. Only what you change is saved, and the post stays live or draft as it is.",
  reading: "Reading the post",
  title: "Title",
  description: "Description",
  descriptionHint: "The summary search results, feeds and cards show. One or two sentences.",
  tags: "Tags",
  tagsHint:
    "A tag is a short topic that lists posts together on one page. Separate tags with commas.",
  category: "Category",
  categoryHint: "The one section of the blog the post is listed in.",
  noCategory: "No category",
  author: "Author",
  noAuthor: "No author",
  save: "Save",
  saving: "Saving",
  close: "Close without saving",
  saved: (title: string) => `Saved ${title}.`,
  nothing: "Nothing changed, so nothing was saved.",
} as const;
