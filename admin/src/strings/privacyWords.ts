// The words of the Privacy screen, where the `[privacy]` table and the
// categories of `[code]` in site.toml are changed. They sit in their own
// file so en.ts stays short.

/** The four categories an owner may give a script, in the order the picker lists them. */
export const CATEGORIES = ["preferences", "analytics", "marketing", "no-tracking"] as const;

/** One of the four categories of a script. */
export type Category = (typeof CATEGORIES)[number];

/** The languages Khalid's own banner words come in, so owners can replace them. */
export const TEXT_LANGUAGES = ["en", "de"] as const;

export const privacyWords = {
  title: "Privacy",
  lede: "Khalid asks readers before anything tracks them or stores data on their device, and nothing optional runs until they agree. Here you choose how it asks. Each part saves to site.toml, and the blog uses it after Khalid restarts.",
  loading: "Reading the privacy settings",
  sections: "Parts of the privacy settings",
  saved: "Saved to site.toml. The blog uses it after Khalid restarts.",
  unchanged: "Nothing to save: this part is as site.toml has it.",
  restart:
    "Some saved settings wait for a restart. Until Khalid restarts, the blog keeps the ones it started with.",
  tabs: {
    settings: "Settings",
    scripts: "Scripts",
    text: "Text",
    contact: "Contact",
    proof: "Proof",
  },
  settings: {
    lede: "How long a reader's choice lasts, what a browser's privacy signal does, and whether another tool asks instead of Khalid.",
    months: "Months a choice lasts",
    monthsHint:
      "After this, readers are asked again. Use 1 to 13; 6 is what EU regulators suggest.",
    gpc: "Global Privacy Control",
    gpcHint:
      "A signal some browsers send to say: do not sell or share my data. Khalid always honours it.",
    gpcMarketing: "Keep Marketing off",
    gpcMarketingHint:
      "Ads and tracking stay off. Readers can still allow counting visits. This is how US law reads the signal.",
    gpcRejectAll: "Treat it as Reject all",
    gpcRejectAllHint: "Every optional category stays off. The strictest reading.",
    counts: "Count readers' choices each day",
    countsHint:
      "Daily totals per category, with no address and no reader ID, so you can show a regulator what readers chose.",
    countsStatic:
      "This blog is built as plain files, so Khalid never sees readers' choices and cannot count them.",
    tool: "I use a certified consent tool instead",
    toolHint:
      "A paid tool, such as Cookiebot, that ad networks require. Khalid then runs your scripts as they are and shows no banner, so your tool must ask first. Videos and other embeds still ask for themselves.",
    toolName: "Name of the tool",
    toolNameHint: "Named on your privacy page, such as Cookiebot.",
  },
  scripts: {
    lede: "Your own code from [code] in site.toml. Each script waits until a reader allows its category. A script with no category counts as Marketing, the strictest.",
    list: "Your scripts",
    script: (n: number) => `Script ${n}`,
    untagged: "No category: counts as Marketing",
    category: "Category",
    vendor: "Who runs it",
    vendorHint: "The company readers see in the banner and on the privacy page.",
    policy: "Their privacy policy",
    policyHint: "The address of the company's own privacy page.",
    noScripts: "No scripts of your own yet.",
    noScriptsWhy:
      "Readers see no banner, because nothing waits for their yes. Videos and other embeds still ask for themselves.",
    noScriptsFix: "Add code to [code] in site.toml, then tag it here.",
    staticTitle: "This blog is built as plain files.",
    staticWhy:
      "A small script in each page asks readers and keeps their choice. Readers with scripts off get nothing optional, and choices are not counted.",
  },
  categories: {
    preferences: "Preferences: remembers settings",
    analytics: "Analytics: counts visits",
    marketing: "Marketing: ads and tracking",
    "no-tracking": "None: sets no cookie, tracks no one",
  } satisfies Record<Category, string>,
  text: {
    lede: "Your own words for the banner and each category, per language. Leave a field empty to keep Khalid's own words.",
    language: "Language",
    languageHint: "Each language has its own words.",
    languages: { en: "English", de: "German" } satisfies Record<
      (typeof TEXT_LANGUAGES)[number],
      string
    >,
    fieldHint: "Empty keeps Khalid's own words.",
    fields: {
      title: "Banner heading",
      banner: "Banner sentence",
      preferences: "Preferences",
      analytics: "Analytics",
      marketing: "Marketing",
      external_media: "External media",
    },
  },
  contact: {
    lede: "Who runs the blog, for the privacy page readers can open from every page.",
    missing: "Your privacy page does not say who to contact yet.",
    missingWhy:
      "The law asks every site to say who runs it and how readers reach you. Add a name and an email.",
    name: "Person or company",
    nameHint: "Who is responsible for readers' data.",
    email: "Email for privacy questions",
    emailHint: "Readers write here to see or delete their data.",
    address: "Postal address",
    addressHint: "Where letters about readers' data can be sent.",
  },
  proof: {
    lede: "What readers were shown and what they chose, to show a regulator. Khalid keeps daily totals only, with no address and no reader ID.",
    loading: "Reading the consent proof",
    counts: "Readers' choices",
    countsLede:
      "Each number is how many answers allowed that category. The rest refused it. Necessary is always on.",
    week: "Week",
    weekOf: (day: string) => `Week of ${day}`,
    answers: "Answers",
    total: "Total",
    none: "No choices recorded yet",
    noneWhy: "Totals appear here once readers answer the banner.",
    download: "Download as CSV",
    history: "What readers were shown",
    historyLede:
      "A new version starts when the banner text, the categories, the vendors or the lifetime change.",
    version: "Version",
    from: "First shown",
    until: "Last shown",
    shown: "What it showed",
    show: "Show",
    noHistory: "No reader has seen the banner yet.",
  },
};
