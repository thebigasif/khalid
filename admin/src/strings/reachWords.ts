// The editor's fields for where a post is filed and who may read it: tags,
// category, featured, access, password, template and head code.

export const reachWords = {
  tags: "Tags",
  tagsHint:
    "Words readers can browse by, separated by commas, such as internal links, anchor text.",
  category: "Category",
  categoryHint: "The one section of the blog this post belongs to.",
  noCategory: "No category",
  featured: "Feature this post",
  featuredHint: "Featured posts come first in lists and on the home page's featured section.",
  title: "Who reads it, and how it looks",
  access: "Who can read it",
  accessHint:
    "Members are readers who signed in. Everyone else sees the title, the excerpt and a note.",
  accessChoices: { public: "Everyone", members: "Members only" },
  passwordOn: "Ask readers for a password",
  passwordOnHint:
    "Readers must type the password before they see the text. Turn it off to open the post again.",
  password: "New password",
  passwordHint:
    "8 to 128 characters. Leave it empty to keep the password already set. Khalid keeps only a scrambled copy.",
  template: "Template",
  templateHint:
    "The theme file this post is shown with, such as wide for post-wide.html. Leave it empty for the usual look.",
  headCode: "Code in the page head",
  headCodeHint:
    "HTML added to this post's page head, such as a verification tag. Only owners and admins can change it.",
} as const;
