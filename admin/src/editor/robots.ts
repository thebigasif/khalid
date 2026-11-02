// A post's robots directives, the words of its `robots` setting such as
// `noindex, nofollow, max-snippet:50`, split into the choices the editor
// shows, and joined again in the order Khalid writes them.

/** The extra uses of a page a writer may forbid. */
export const RESTRICTIONS = ["noimageindex", "noarchive", "nosnippet"] as const;
/** One of them. */
export type Restriction = (typeof RESTRICTIONS)[number];
/** The largest image preview search engines may show; empty for any. */
export const IMAGE_PREVIEWS = ["", "large", "standard", "none"] as const;
/** One of them. */
export type ImagePreview = (typeof IMAGE_PREVIEWS)[number];

/** A post's robots directives as the editor's fields hold them. */
export type Robots = {
  noindex: boolean;
  nofollow: boolean;
  restrictions: Restriction[];
  /** The longest snippet in characters; empty for any length. */
  maxSnippet: string;
  maxImagePreview: ImagePreview;
  /** Words the editor has no field for, such as `notranslate`, kept as written. */
  others: string[];
};

/** The words of `text` as the editor's fields. */
export function parseRobots(text: string): Robots {
  const robots: Robots = {
    noindex: false,
    nofollow: false,
    restrictions: [],
    maxSnippet: "",
    maxImagePreview: "",
    others: [],
  };
  for (const word of text.split(",").map((each) => each.trim().toLowerCase())) {
    const [name = "", value = ""] = word.split(":").map((part) => part.trim());
    const restriction = RESTRICTIONS.find((each) => each === name);
    const preview = IMAGE_PREVIEWS.find((each) => each !== "" && each === value);
    if (name === "noindex" || name === "nofollow") {
      robots[name] = true;
    } else if (restriction !== undefined) {
      robots.restrictions.push(restriction);
    } else if (name === "max-snippet" && value !== "-1") {
      robots.maxSnippet = value;
    } else if (name === "max-image-preview" && preview !== undefined && preview !== "large") {
      robots.maxImagePreview = preview;
    } else if (!["", "index", "follow", "all", "max-snippet", "max-image-preview"].includes(name)) {
      robots.others.push(word);
    }
  }
  return robots;
}

/**
 * The setting's words for `robots`, leaving out `index` and `follow`, which
 * search engines assume; empty when every choice is the default.
 */
export function robotsText(robots: Robots): string {
  const words = [
    ...(robots.noindex ? ["noindex"] : []),
    ...(robots.nofollow ? ["nofollow"] : []),
    ...RESTRICTIONS.filter((each) => robots.restrictions.includes(each)),
    ...robots.others,
  ];
  const snippet = robots.maxSnippet.trim();
  if (snippet !== "") {
    words.push(`max-snippet:${snippet}`);
  }
  if (robots.maxImagePreview !== "") {
    words.push(`max-image-preview:${robots.maxImagePreview}`);
  }
  return words.join(", ");
}
