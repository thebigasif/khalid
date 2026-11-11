// The snippet routes of the Admin API (G24). They are typed by hand, field
// for field as `AdminSnippet` and `NewSnippet`, because the OpenAPI document
// gains them only when the G24 stack lands. Then these types become
// `components["schemas"]` and the calls go through `createApi`.
import { useQuery } from "@tanstack/react-query";
import { call, nothing, type Reply, sending } from "./call";

/** A snippet in one language, as the Admin API sends it. */
export type Snippet = {
  /** The language and the slug, such as `en.affiliate-disclosure`. */
  id: string;
  /** The slug a post places it by. */
  slug: string;
  language: string;
  /** The name shown in the editor. */
  title: string;
  /** The blocks, in markdown. */
  markdown: string;
  updated_at: string;
};

/** A new snippet, in the site's default language. */
export type NewSnippet = { slug: string; title: string; markdown: string };

async function snippets(response: Response): Promise<Snippet[]> {
  const body: { snippets: Snippet[] } = await response.json();
  return body.snippets;
}

/** Every snippet in every language. */
export function listSnippets(): Promise<Reply<Snippet[]>> {
  return call("/snippets/", {}, snippets);
}

/** Saves a new snippet; gives back the one made. */
export function createSnippet(snippet: NewSnippet): Promise<Reply<Snippet[]>> {
  const init = {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ snippets: [snippet] }),
  };
  return call("/snippets/", init, snippets);
}

/** A change to a snippet's name or blocks; only the fields sent change. */
export type SnippetEdit = { title?: string; markdown?: string };

/** Saves `edit` on the copy of `snippet` it was made on; a newer copy answers 409. */
export function editSnippet(snippet: Snippet, edit: SnippetEdit): Promise<Reply<Snippet[]>> {
  const body = { snippets: [{ ...edit, updated_at: snippet.updated_at }] };
  return call(`/snippets/${encodeURIComponent(snippet.id)}/`, sending("PUT", body), snippets);
}

/** Deletes the copy of `snippet` it was read as; a newer copy answers 409. */
export function deleteSnippet(snippet: Snippet): Promise<Reply<null>> {
  const seen = new URLSearchParams({ updated_at: snippet.updated_at });
  const path = `/snippets/${encodeURIComponent(snippet.id)}/?${seen}`;
  return call(path, { method: "DELETE" }, nothing);
}

/** The site's snippets, once loaded. */
export function useSnippets(): Reply<Snippet[]> | undefined {
  return useQuery({ queryKey: ["snippets"], queryFn: listSnippets }).data;
}

/** The slug a snippet called `name` gets: small letters, digits and hyphens. */
export function snippetSlug(name: string): string {
  return name
    .normalize("NFKD")
    .replace(/\p{M}/gu, "")
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "");
}
