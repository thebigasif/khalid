// The words of the SEO tools: the bulk SEO editor and the workouts.
export const seoToolWords = {
  bulk: {
    nav: "Bulk SEO",
    title: "Edit SEO in bulk",
    lede: "Fix the search titles, descriptions and focus keywords of many posts on one screen. Each row saves on its own.",
    terms: [
      {
        term: "SEO title",
        meaning: "The title Google shows. Left empty, the post's own title is used.",
      },
      {
        term: "Description",
        meaning: "The two lines under the title in Google. About 160 characters fit.",
      },
      {
        term: "Focus keyword",
        meaning: "The one search phrase the post is written to answer.",
      },
    ],
    show: "Show",
    every: "Every post",
    missing: {
      title: "Posts with no SEO title",
      description: "Posts with no description",
      keyword: "Posts with no focus keyword",
    },
    hidden: "Posts on this page that already have it are hidden.",
    loading: "Reading your posts",
    none: "No posts here. Pick another filter above.",
    post: "Post",
    seoTitle: "SEO title",
    description: "Description",
    keyword: "Focus keyword",
    actions: "Save",
    save: "Save",
    saved: "Saved.",
    labelFor: (field: string, post: string) => `${field} of ${post}`,
    saveFor: (post: string) => `Save ${post}`,
    length: (count: number, most: number) => `${count} of about ${most} characters`,
    page: (page: number, pages: number) => `Page ${page} of ${pages}`,
    previous: "Previous page",
    next: "Next page",
  },
} as const;
