import type { AdminPage, PageEdit } from "../api/pages";
import type { components } from "../api/schema";

type PageSeo = AdminPage["seo"];
type ShareCard = components["schemas"]["ShareCard"];
type PageSeoEdit = components["schemas"]["PageSeoEdit"];

/** A page's search and sharing settings as the form holds them: plain text. */
export type SeoFields = {
  robots: string;
  canonical: string;
  imageSrc: string;
  imageAlt: string;
  facebookTitle: string;
  facebookDescription: string;
  xTitle: string;
  xDescription: string;
};

/** A page's fields as the form holds them: plain text. */
export type PageFields = { title: string; description: string; markdown: string } & SeoFields;

/** The search choice the form shows for `robots`: listed, hidden, or hidden with its links unfollowed. */
export function robotsChoice(robots: string | null | undefined): string {
  const said = robots ?? "";
  if (!said.startsWith("noindex")) {
    return "";
  }
  return said.includes("nofollow") ? "noindex, nofollow" : "noindex";
}

function seoFieldsOf(seo: PageSeo): SeoFields {
  const cards = seo.share_cards ?? {};
  return {
    robots: robotsChoice(seo.robots),
    canonical: seo.canonical ?? "",
    imageSrc: seo.feature_image?.src ?? "",
    imageAlt: seo.feature_image?.alt ?? "",
    facebookTitle: cards.facebook?.title ?? "",
    facebookDescription: cards.facebook?.description ?? "",
    xTitle: cards.x?.title ?? "",
    xDescription: cards.x?.description ?? "",
  };
}

/** The form's starting text for `page`. */
export function fieldsOf(page: AdminPage): PageFields {
  return {
    title: page.title,
    description: page.description ?? "",
    markdown: page.markdown,
    ...seoFieldsOf(page.seo),
  };
}

/** One share card from its title and description, keeping the image it had; `null` when empty. */
function card(title: string, description: string, before: ShareCard | null | undefined) {
  const made: ShareCard = {
    title: title.trim() || null,
    description: description.trim() || null,
    image: before?.image ?? null,
  };
  return made.title === null && made.description === null && made.image === null ? null : made;
}

/** The settings that changed between `seo` and `fields`, or `null` when none did. */
function seoChanges(seo: PageSeo, fields: SeoFields): PageSeoEdit | null {
  const before = seoFieldsOf(seo);
  const edit: PageSeoEdit = {};
  const differs = (key: keyof SeoFields) => fields[key].trim() !== before[key].trim();
  if (differs("robots")) {
    edit.robots = fields.robots || null;
  }
  if (differs("canonical")) {
    edit.canonical = fields.canonical.trim() || null;
  }
  if (differs("imageSrc") || differs("imageAlt")) {
    const src = fields.imageSrc.trim();
    const caption = seo.feature_image?.caption ?? null;
    edit.feature_image = src === "" ? null : { src, alt: fields.imageAlt.trim(), caption };
  }
  const cardKeys = ["facebookTitle", "facebookDescription", "xTitle", "xDescription"] as const;
  if (cardKeys.some(differs)) {
    const cards = seo.share_cards ?? {};
    edit.share_cards = {
      facebook: card(fields.facebookTitle, fields.facebookDescription, cards.facebook),
      x: card(fields.xTitle, fields.xDescription, cards.x),
    };
  }
  return Object.keys(edit).length === 0 ? null : edit;
}

/**
 * The edit that turns `page` into `fields`, with only the fields that
 * changed; `null` when nothing did. An empty description removes it, as
 * the API reads it. The text is sent as typed, since spaces can matter in
 * markdown; only a change at its end alone does not count.
 */
export function changes(page: AdminPage, fields: PageFields): PageEdit | null {
  const before = fieldsOf(page);
  const edit: PageEdit = { updated_at: page.updated_at };
  let changed = false;
  for (const key of ["title", "description"] as const) {
    if (fields[key].trim() !== before[key].trim()) {
      edit[key] = fields[key].trim();
      changed = true;
    }
  }
  if (fields.markdown.trimEnd() !== before.markdown.trimEnd()) {
    edit.markdown = fields.markdown;
    changed = true;
  }
  const seo = seoChanges(page.seo, fields);
  if (seo !== null) {
    edit.seo = seo;
    changed = true;
  }
  return changed ? edit : null;
}
