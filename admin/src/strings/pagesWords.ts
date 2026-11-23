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
  address: {
    label: "Address",
    hint: "The last part of the page's address, such as about. If you change it, the old address sends readers and search engines to the new one with a 301 redirect, the signal that a page moved for good.",
  },
  robots: {
    label: "Search results",
    hint: "Whether search engines may list this page. Hide a page such as a thank-you page.",
  },
  canonical: {
    label: "Original address",
    hint: "Only when the page first appeared somewhere else, such as https://example.com/about/. Search engines then credit that address.",
  },
  imageSrc: {
    label: "Feature image",
    hint: "Shown under the title and in link previews. A media path such as /media/team.jpg, or a full address.",
  },
  imageAlt: {
    label: "Image description",
    hint: "What the image shows, for readers who cannot see it. Needed when there is an image.",
  },
  facebookTitle: {
    label: "Facebook title",
    hint: "The title Facebook and LinkedIn show when someone shares the page.",
  },
  facebookDescription: { label: "Facebook description", hint: "The line under that title." },
  xTitle: { label: "X title", hint: "The title X shows when someone shares the page." },
  xDescription: { label: "X description", hint: "The line under that title." },
};

const seo = {
  summary: "Search and sharing",
  lede: "How search engines and social sites show this page. A link preview is the card a social site shows when someone shares the page. Leave a field empty to use the page's own title, description and image.",
  robots: [
    { value: "", label: "Show this page in search results" },
    { value: "noindex", label: "Keep it out of search results" },
    { value: "noindex, nofollow", label: "Keep it out, and do not follow its links" },
  ],
};

export const pagesWords = {
  nav: "Pages",
  title: "Pages",
  lede: "A page is part of your site that is not a post, such as About or Contact. It has no date and no author, and stays out of the post lists and feeds. Each page is a file in your site folder, so git keeps its history.",
  fields,
  seo,
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
        `Deleting moves ${title} to the trash, where you can put it back. Links to it from posts, the menu and other sites then get the answer you choose below, so you do not have to change them by hand.`,
      submit: (title: string) => `Delete ${title}`,
      farewell: "What should the page's old address do?",
      farewellHint:
        "Readers and search engines may still follow links to this page. Tell them what happened.",
      gone: "Say the page is gone for good",
      goneHint: "Search engines drop the address quickly. This is a 410 answer.",
      moved: "Send readers to another page or post",
      movedHint:
        "Search engines move the old address's standing to the new one. This is a 301 redirect.",
      target: "Page or post to send readers to",
      targetHint: "Pick the one closest to what this page covered.",
      noTarget: "There is no other live page or post to send readers to.",
      noFarewell: (title: string, from: string, why: string) => ({
        what: `${title} is in the trash, but ${from} was not redirected.`,
        why,
        fix: "Add the redirect under Redirects, or put the page back from the trash.",
      }),
    },
  },
};
