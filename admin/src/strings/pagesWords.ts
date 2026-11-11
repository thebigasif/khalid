// The words of the Pages screen: the site's pages that stand apart from
// the posts, such as About, and adding, changing and deleting one.

const fields = {
  title: { label: "Title", hint: "As readers see it at the top of the page." },
  slug: {
    label: "Address",
    hint: "The last part of the page's address. Lowercase words joined by hyphens, such as about.",
  },
  language: {
    label: "Language",
    hint: "The page's language, such as de. Leave it empty for the blog's main one.",
  },
  description: {
    label: "Description",
    hint: "One or two sentences search engines show under the title. Optional.",
  },
  markdown: { label: "Text", hint: "What the page says, in markdown." },
};

export const pagesWords = {
  nav: "Pages",
  title: "Pages",
  lede: "A page is part of your site that is not a post, such as About or Contact. It has no date and no author, and stays out of the post lists and feeds. Each page is a file in your site folder, so git keeps its history.",
  fields,
  add: {
    title: "Add a page",
    live: "A new page is live as soon as you add it.",
    submit: "Add page",
    added: (title: string) => `Added. ${title} is live now.`,
  },
  list: {
    title: "Your pages",
    loading: "Reading your pages",
    none: "No pages yet. Add the first one above.",
    name: "Title",
    slug: "Address",
    language: "Language",
    action: "Action",
    edit: "Edit",
    editLabel: (title: string, language: string) => `Edit ${title}, ${language}`,
  },
  edit: {
    title: "Page",
    back: "All pages",
    loading: "Reading the page",
    lede: (slug: string, language: string) =>
      `This is the ${language} page at the address ${slug}. Saving puts your change live at once.`,
    save: "Save page",
    saved: "Saved. Readers see the change now.",
    unchanged: "Nothing changed, so nothing was saved.",
    remove: {
      title: "Delete this page",
      lede: (title: string) =>
        `Deleting moves ${title} to the trash, and its address stops working until you put it back. A link to it from a post or the menu leads nowhere meanwhile, so change those first.`,
      submit: (title: string) => `Delete ${title}`,
    },
  },
};
