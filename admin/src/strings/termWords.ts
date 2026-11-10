// The Tags and categories screen's words, kept beside en.ts so that file stays short.

/** "1 post" or "12 posts". */
function posts(count: number): string {
  return count === 1 ? "1 post" : `${count} posts`;
}

export const termWords = {
  title: "Tags and categories",
  lede: "A category is the one shelf a post sits on, such as Technical SEO. A tag is a loose label, and a post can have many. Each has its own page on your blog that lists its posts. Change them here and every post that uses them changes with them.",
  tags: {
    title: "Tags",
    lede: "Tags come from your posts: a tag exists while one post has it. Each language has its own tags.",
    loading: "Reading your tags",
    none: "No post has a tag yet. Add tags to a post in the editor's settings, and they show here.",
    name: "Name",
    language: "Language",
    posts: "Posts",
    count: posts,
  },
} as const;
