// The Tags and categories screen's words, kept beside en.ts so that file stays short.

/** "1 post" or "12 posts". */
function posts(count: number): string {
  return count === 1 ? "1 post" : `${count} posts`;
}

export const termWords = {
  title: "Tags and categories",
  lede: "A category is the one shelf a post sits on, such as Technical SEO. A tag is a loose label, and a post can have many. Each has its own page on your blog that lists its posts. Change them here and every post that uses them changes with them.",
  back: "Back to the list",
  categories: {
    title: "Categories",
    lede: "Each category has a name in every language of your site. Your categories live in categories.toml in your site folder.",
    loading: "Reading your categories",
    none: "Your site has no categories yet. Add the first one below.",
    name: "Name",
    slug: "Short name",
    posts: "Posts",
    count: (count: number) => (count === 0 ? "No posts" : posts(count)),
    other: (language: string, name: string) => `${language}: ${name}`,
  },
  addCategory: {
    title: "Add a category",
    slug: "Short name",
    slugHint:
      "Posts name the category by it in their category field. Lowercase words joined by hyphens. It cannot change later.",
    name: (language: string) => `Name in ${language}`,
    submit: "Add category",
    added: (name: string) => `Added the category "${name}".`,
  },
  tags: {
    title: "Tags",
    lede: "Tags come from your posts: a tag exists while one post has it. Each language has its own tags.",
    loading: "Reading your tags",
    none: "No post has a tag yet. Add tags to a post in the editor's settings, and they show here.",
    name: "Name",
    language: "Language",
    posts: "Posts",
    action: "Action",
    count: posts,
    rename: "Rename",
    renameLabel: (name: string, language: string) => `Rename ${name}, ${language}`,
    remove: "Remove",
    removeLabel: (name: string, language: string) => `Remove ${name}, ${language}`,
  },
  renameTag: {
    title: (name: string) => `Rename "${name}"`,
    lede: (count: number, language: string) =>
      `${count === 1 ? "1 post has" : `${count} posts have`} this tag, drafts included. The new name replaces it in each of them, and live posts change at once. If another ${language} tag already has the new name, the two become one tag.`,
    label: "New name",
    hint: "As readers see it on posts and on the tag's page.",
    submit: (count: number) => `Rename in ${posts(count)}`,
    done: (from: string, to: string) => `"${from}" is now "${to}".`,
  },
  removeTag: {
    title: (name: string) => `Remove "${name}"`,
    lede: (count: number) =>
      `The tag comes off ${posts(count)}, drafts included. The posts stay, and live posts change at once. The tag's own page goes with it.`,
    submit: (count: number) => `Remove from ${posts(count)}`,
    done: (name: string) => `"${name}" is gone from every post.`,
  },
} as const;
