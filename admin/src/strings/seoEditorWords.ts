// The words of the editor's SEO parts: the two scores, the search result
// on desktop and mobile, the share cards, the robots choices, secondary
// keywords and link suggestions. They sit in their own file so en.ts
// stays short.
export const seoEditorWords = {
  lights: "SEO and readability",
  seo: "SEO",
  readability: "Readability",
  outOf: (value: number) => `${value} of 100`,
  // What each light means, in words, so colour is never the only sign.
  state: { good: "Good", ok: "Could be better", bad: "Needs work" },
  words: (count: number) => `${count} ${count === 1 ? "word" : "words"}`,
  device: "Show the result as on",
  desktop: "Desktop",
  mobile: "Mobile",
  titleCut: "Google cuts this title here, so put the words that matter first.",
  descriptionCut: "Google cuts this description here, so put what matters first.",
  secondary: "Secondary keywords",
  secondaryHint:
    "Up to four related phrases, one per line, such as a synonym of the focus keyword. The checks look for each one in the text.",
  advanced: "More search engine choices",
  advancedHint:
    "Most posts need none of these. Each one stops search engines using this post in one more way.",
  restrictions: {
    noimageindex: "Keep this post's images out of image search",
    noarchive: "Do not offer a cached copy of this post",
    nosnippet: "Show no text from this post under its title in results",
  },
  maxSnippet: "Longest text shown in results",
  maxSnippetHint: "In characters. Leave it empty for any length.",
  maxImage: "Largest picture shown in results",
  maxImageHint: "Large pictures win more clicks, and Google Discover needs them.",
  imageSizes: { "": "Any size", large: "Large", standard: "Standard", none: "None" },
  share: "How it looks when shared",
  shareLede:
    "A link to this post on Facebook and X, made from the share settings, or else the post's own title, description and image.",
  facebook: "Facebook",
  x: "X",
  noImage: "No picture yet. Add a feature image, or one for each network in More settings.",
  links: "Internal links to add",
  linksLede:
    "Posts on your blog this one does not link to yet, the closest first. Links help readers and search engines find your other posts.",
  noLinks: "No suggestions now. Add a category or tags, and posts on the same topic show up here.",
  addLink: (title: string) => `Add a link to ${title}`,
  addLinkShort: "Add a link",
  added: "Added to the text. It is saved with your next change.",
  why: {
    category: "Same category",
    tags: (tags: string) => `Shares the tags ${tags}`,
    cluster: "Same topic cluster",
    pillar: "The main post of this topic cluster",
    keyword: (keyword: string) => `Your text names its focus keyword, "${keyword}"`,
  },
  copy: "Copy as a new draft",
  copyHint:
    "Makes a new draft with this post's text and settings, to start a similar post or rework this one safely.",
} as const;
