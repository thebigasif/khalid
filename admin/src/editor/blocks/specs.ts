/** One directive block: its name, its form and its attributes in print order. */
export type BlockSpec = {
  name: string;
  /** A leaf is one `::name{...}` line; every other block has a body. */
  leaf: boolean;
  /** The attributes it takes, in the order a printed block writes them. */
  attributes: readonly string[];
  /** The attribute a card shows to tell this block from the next one. */
  shows: string | null;
  /** The colons that open a new block: four for a block holding blocks. */
  fence: string;
};

const leaf = (name: string, attributes: string[], shows: string | null): BlockSpec => ({
  name,
  leaf: true,
  attributes,
  shows,
  fence: "::",
});

const box = (
  name: string,
  attributes: string[],
  shows: string | null,
  fence = ":::",
): BlockSpec => ({
  name,
  leaf: false,
  attributes,
  shows,
  fence,
});

const PRODUCT = [
  "name",
  "image",
  "price",
  "currency",
  "rating",
  "best",
  "average",
  "ratings",
  "href",
  "button",
];

const RECIPE = [
  "name",
  "description",
  "image",
  "servings",
  "prep",
  "cook",
  "category",
  "cuisine",
  "calories",
];

const EVENT = [
  "name",
  "start",
  "end",
  "venue",
  "street",
  "locality",
  "postcode",
  "country",
  "online",
  "status",
  "price",
  "currency",
  "image",
  "performer",
];

/**
 * Every directive of plan section 4.6, with the attributes `khalid-content`
 * takes for it (`crates/khalid-content/src/directive.rs` and the files in
 * `directive/`), in the order that crate lists them.
 */
export const BLOCK_SPECS: readonly BlockSpec[] = [
  leaf("audio", ["src", "title"], "title"),
  leaf("book", ["name", "author", "isbn", "href"], "name"),
  leaf("bookmark", ["href", "title", "description", "image"], "href"),
  leaf(
    "business",
    ["name", "type", "street", "locality", "postcode", "country", "phone", "email", "href"],
    "name",
  ),
  leaf("button", ["href", "label"], "label"),
  box("callout", ["kind"], "kind"),
  box("chart", ["type", "title", "unit", "src"], "title"),
  box("columns", [], null, "::::"),
  box("course", ["name", "href"], "name"),
  leaf("cta", [], null),
  box("dataset", ["name", "keywords", "license", "download"], "name"),
  leaf("embed", ["src", "title", "height", "uploaded"], "title"),
  box("event", EVENT, "name"),
  box("faq", [], null, "::::"),
  box("figure", ["src", "alt"], "alt"),
  leaf("file", ["src", "title"], "title"),
  box("form", ["id", "button"], "id"),
  box("gallery", [], null),
  box("gated", ["note", "label", "button"], "note"),
  box("header", ["image", "alt"], null),
  box("howto", ["name", "time"], "name", "::::"),
  box("html", ["show"], "show"),
  leaf("location", ["name"], "name"),
  box("maths", ["label"], "label"),
  leaf("newsletter", ["label", "button"], "label"),
  leaf("privacy-facts", [], null),
  box("product", PRODUCT, "name"),
  box("proscons", ["pros", "cons"], "pros"),
  box("pullquote", ["by"], "by"),
  box("recipe", RECIPE, "name", "::::"),
  box("review", ["item", "type", "rating", "best", "average", "ratings", "author"], "item"),
  leaf("snippet", ["slug"], "slug"),
  box("table", ["fold", "more"], "more"),
  box("takeaways", ["title"], "title"),
  leaf("toc", [], null),
  box("toggle", ["summary"], "summary"),
  leaf(
    "video",
    ["src", "title", "poster", "description", "duration", "thumbnail", "uploaded"],
    "title",
  ),
];

/** The names of blocks written inside another block, such as `:::question`. */
export const INNER_BLOCKS: readonly string[] = [
  "question",
  "step",
  "ingredients",
  "method",
  "column",
];
