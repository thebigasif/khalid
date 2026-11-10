import type { AdminPost, PostEdit } from "../api/posts";

/** What quick edit shows of a post, as its fields hold it; an empty text means none. */
export type Basics = {
  title: string;
  description: string;
  /** The tags, separated by commas. */
  tags: string;
  /** A category's slug. */
  category: string;
  /** An author's slug. */
  author: string;
};

/** The basics of `post`, as quick edit first shows them. */
export function basicsOf(post: AdminPost): Basics {
  return {
    title: post.title,
    description: post.settings.excerpt ?? "",
    tags: post.settings.tags.join(", "),
    category: post.settings.category ?? "",
    author: post.author ?? "",
  };
}

/** The tags written in `text`, separated by commas, each once whatever its capitals. */
export function tagsOf(text: string): string[] {
  const tags: string[] = [];
  for (const part of text.split(",")) {
    const tag = part.trim();
    if (tag !== "" && !tags.some((kept) => kept.toLowerCase() === tag.toLowerCase())) {
      tags.push(tag);
    }
  }
  return tags;
}

/** `text` with its outer spaces gone, or `null` when nothing is left. */
function orNull(text: string): string | null {
  const kept = text.trim();
  return kept === "" ? null : kept;
}

/**
 * The edit that turns `post` into `basics`, naming only what changed, or `null` when
 * nothing did.
 */
export function editFrom(post: AdminPost, basics: Basics): PostEdit | null {
  const before = basicsOf(post);
  const edit: PostEdit = { updated_at: post.updated_at };
  const settings: NonNullable<PostEdit["settings"]> = {};
  if (basics.title.trim() !== before.title) {
    edit.title = basics.title.trim();
  }
  if (basics.author !== before.author) {
    edit.author = orNull(basics.author);
  }
  if (orNull(basics.description) !== orNull(before.description)) {
    settings.excerpt = orNull(basics.description);
  }
  const tags = tagsOf(basics.tags);
  if (tags.join("\n") !== post.settings.tags.join("\n")) {
    settings.tags = tags;
  }
  if (basics.category !== before.category) {
    settings.category = orNull(basics.category);
  }
  if (Object.keys(settings).length > 0) {
    edit.settings = settings;
  }
  return Object.keys(edit).length > 1 ? edit : null;
}
