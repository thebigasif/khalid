// The words of the Design screen, where the colours, fonts, shape and logo
// in theme/tokens.toml are changed. They sit in their own file so en.ts
// stays short.
import { LIVE } from "./restartWords";

/** The colour tokens, in the order of the file and the screen. */
export const COLOR_TOKENS = [
  "colors.background",
  "colors.surface",
  "colors.text",
  "colors.muted",
  "colors.link",
  "colors.accent",
  "colors.border",
] as const;

/** One colour token. */
export type ColorToken = (typeof COLOR_TOKENS)[number];

export const designWords = {
  title: "Design",
  lede: "Your blog's colours, fonts and logo. Unless you change them here, they come from your template or from your main site through theme sync. A change is saved to theme/tokens.toml in your site folder, so git keeps a record of it like any other file.",
  loading: "Reading the design",
  live: LIVE,
  unchanged: "Nothing to save: the design is as theme/tokens.toml has it.",
  colors: "Colours",
  allReadable: "Every text colour is easy to read.",
  someFaint: (count: number) =>
    count === 1
      ? "One colour is too faint for some readers."
      : `${count} colours are too faint for some readers.`,
  standard:
    "WCAG AA, the web's standard for readable pages, asks for a contrast of 4.5 to 1 between text and its background, and 3 to 1 for buttons.",
  columns: {
    usedFor: "Used for",
    swatch: "Swatch",
    color: "Colour",
    readable: "Readable?",
    keep: "Keep when syncing",
  },
  usedFor: {
    "colors.background": "Page background",
    "colors.surface": "Panels and code",
    "colors.text": "Text",
    "colors.muted": "Quiet text, such as dates",
    "colors.link": "Links",
    "colors.accent": "Buttons and highlights",
    "colors.border": "Lines between parts",
  } satisfies Record<ColorToken, string>,
  colorLabel: (usedFor: string) => `${usedFor} colour`,
  keepLabel: (usedFor: string) => `Keep ${usedFor.toLowerCase()} when syncing`,
  checkedAgainst: "Text is checked against it",
  notText: "Not used for text",
  notColor: "Not a colour yet. Write it as #rrggbb, such as #1f2328.",
  ratio: (ratio: number) => `${ratio.toFixed(1)} to 1`,
  textOn: (ratio: number) => `Text on it: ${ratio.toFixed(1)} to 1`,
  tooFaint: (ratio: number) => `${ratio.toFixed(1)} to 1, too faint`,
  faintOnPanels: (ratio: number) => `${ratio.toFixed(1)} to 1 on panels, too faint`,
  useNearest: (shade: string) => `Use ${shade}, the closest shade that is easy to read`,
  fonts: "Fonts",
  body: "Text",
  bodyHint: "The font of every paragraph. A system font shows at once, with nothing to download.",
  heading: "Headings",
  shape: "Shape",
  radius: "Corner roundness",
  radiusHint: "How round the corners of buttons, images and boxes are, such as 0.375rem.",
  measure: "Line length",
  measureHint:
    "The widest a line of text may get, such as 40rem. About 70 letters is easy to read.",
  logo: "Logo",
  logoHint:
    "The picture at the top of every page. Pick one you uploaded to Media. Without one, the blog shows its name.",
  noLogo: "No logo. The blog shows its name.",
  logoAlt: "Your logo",
  chooseLogo: "Choose from Media",
  removeLogo: "Show the name instead",
  pickTitle: "Pictures in Media",
  noPictures: "Media has no pictures yet. Upload one on the Media screen first.",
  pick: (name: string) => `Use ${name} as the logo`,
  preview: "How a post looks",
  previewDate: "14 September 2026",
  previewTitle: "Why internal links matter",
  previewText:
    "Internal links help readers and search engines find the rest of your site. Here is how to add them without overthinking it.",
  save: "Save changes",
  resync:
    "After your main site changes its look, run khalid theme sync on the server. It replaces every colour and font you have not ticked to keep.",
};
