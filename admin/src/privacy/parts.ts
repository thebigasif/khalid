// The `[privacy]` table and the `[code]` entries of site.toml as the
// settings API sends them, typed field for field as `PrivacySettings` and
// `CodeEntry` in khalid-core (privacy plan U2). Each part holds only what
// the file sets, so a missing key keeps Khalid's default.
import { CATEGORIES, type Category } from "../strings/privacyWords";

/** What Global Privacy Control turns off. */
export type GpcMode = "marketing" | "reject-all";

/** The owner's words for one language, each replacing Khalid's own. */
export type PrivacyText = {
  title?: string;
  banner?: string;
  preferences?: string;
  analytics?: string;
  marketing?: string;
  external_media?: string;
};

/** Who runs the site, for the privacy page. */
export type PrivacyContact = { name?: string; email?: string; address?: string };

/** The `[privacy]` table: every key may be left out. */
export type PrivacyTable = {
  /** 6 unless set, from 1 to 13. */
  lifetime_months?: number;
  gpc?: GpcMode;
  /** On for a server and off for a static host unless set. */
  counts?: boolean;
  /** The certified consent tool that asks instead of Khalid. */
  external_tool?: string;
  /** The owner's words, per language such as `en`. */
  text?: Record<string, PrivacyText>;
  contact?: PrivacyContact;
};

/** One part of `[privacy]` as a change sends it. */
export type PrivacyPart = number | string | boolean | Record<string, unknown>;

/** One entry of `[code]`, as written. */
export type CodeEntry = {
  head?: string;
  body?: string;
  /** Marketing when left out. */
  category?: Category;
  vendor?: string;
  policy?: string;
};

/** How long a choice lasts when the table does not say. */
export const DEFAULT_MONTHS = 6;

/** The category `entry` names, or `null` when it names none and counts as Marketing. */
export function categoryOf(entry: CodeEntry): Category | null {
  const named = CATEGORIES.find((each) => each === entry.category);
  return named ?? null;
}

/** Whether the privacy page lacks what the law asks for: who runs the site and an email. */
export function contactMissing(table: PrivacyTable | undefined): boolean {
  const contact = table?.contact;
  return !contact?.name?.trim() || !contact?.email?.trim();
}

/** `text` trimmed, or `undefined` when empty, so the file leaves the key out. */
export function given(text: string): string | undefined {
  const trimmed = text.trim();
  return trimmed === "" ? undefined : trimmed;
}
