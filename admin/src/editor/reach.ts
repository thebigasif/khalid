import type { components } from "../api/schema";

type Schemas = components["schemas"];
type Settings = Schemas["PostSettings"];
type Edit = Schemas["SettingsEdit"];

/** Who may read a post's body. */
export type Access = "public" | "members";

/**
 * Where a post is filed and who may read it, as the fields hold them. Tags
 * are comma-separated, text is empty for none, and `newPassword` is empty
 * until someone types one: the saved password is never sent back.
 */
export type ReachDraft = {
  tags: string;
  category: string;
  featured: boolean;
  access: Access;
  passwordOn: boolean;
  newPassword: string;
  template: string;
  headCode: string;
};

/** The settings in a {@link ReachDraft} whose field is not free text. */
export type ReachChoice = "featured" | "access" | "passwordOn";

/** The text fields of a {@link ReachDraft}. */
export const REACH_TEXT = ["tags", "category", "newPassword", "template", "headCode"] as const;

const KEYS = [...REACH_TEXT, "featured", "access", "passwordOn"] as const;

/** The fields for a post's `settings`. */
export function reachDraft(settings: Settings): ReachDraft {
  return {
    tags: settings.tags.join(", "),
    category: settings.category ?? "",
    featured: settings.featured,
    access: settings.access === "members" ? "members" : "public",
    passwordOn: settings.password_protected,
    newPassword: "",
    template: settings.template ?? "",
    headCode: settings.head_code ?? "",
  };
}

/** Whether two drafts hold the same filing and access. */
export function sameReach(a: ReachDraft, b: ReachDraft): boolean {
  return KEYS.every((key) => a[key] === b[key]);
}

function orNull(text: string): string | null {
  const trimmed = text.trim();
  return trimmed === "" ? null : trimmed;
}

/** The tags in `text`, each trimmed, empty ones and repeats left out. */
export function tagList(text: string): string[] {
  const tags: string[] = [];
  for (const each of text.split(",")) {
    const tag = each.trim();
    if (tag !== "" && !tags.some((kept) => kept.toLowerCase() === tag.toLowerCase())) {
      tags.push(tag);
    }
  }
  return tags;
}

/** The new password to send, `null` to remove it, or nothing to keep it. */
function password(saved: ReachDraft, now: ReachDraft): Pick<Edit, "password"> {
  if (!now.passwordOn) {
    return saved.passwordOn ? { password: null } : {};
  }
  // A password is sent as typed: spaces may be part of it.
  return now.newPassword === "" ? {} : { password: now.newPassword };
}

/** The settings of an edit that change filing and access: only those that differ from `saved`. */
export function reachEdit(saved: ReachDraft, now: ReachDraft): Edit {
  const edit: Edit = { ...password(saved, now) };
  if (now.tags !== saved.tags) {
    edit.tags = tagList(now.tags);
  }
  if (now.category !== saved.category) {
    edit.category = orNull(now.category);
  }
  if (now.featured !== saved.featured) {
    edit.featured = now.featured;
  }
  if (now.access !== saved.access) {
    edit.access = now.access;
  }
  if (now.template !== saved.template) {
    edit.template = orNull(now.template);
  }
  if (now.headCode !== saved.headCode) {
    edit.head_code = orNull(now.headCode);
  }
  return edit;
}
