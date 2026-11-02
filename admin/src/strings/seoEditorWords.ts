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
} as const;
