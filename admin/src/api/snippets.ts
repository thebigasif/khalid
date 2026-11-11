// The snippet routes of the Admin API (G24), typed from the OpenAPI
// document. The calls go through `call`, which keeps the answer's problem.
import { useQuery } from "@tanstack/react-query";
import { call, nothing, type Reply, sending } from "./call";
import type { components } from "./schema";

type Schemas = components["schemas"];

/** A snippet in one language, as the Admin API sends it. */
export type Snippet = Schemas["AdminSnippet"];

/** A new snippet, in the site's default language. */
export type NewSnippet = Pick<Schemas["NewSnippet"], "slug" | "title" | "markdown">;

async function snippets(response: Response): Promise<Snippet[]> {
  const body: Schemas["AdminSnippets"] = await response.json();
  return body.snippets;
}

/** Every snippet in every language. */
export function listSnippets(): Promise<Reply<Snippet[]>> {
  return call("/snippets/", {}, snippets);
}

/** Saves a new snippet; gives back the one made. */
export function createSnippet(snippet: NewSnippet): Promise<Reply<Snippet[]>> {
  const body: Schemas["NewSnippets"] = { snippets: [snippet] };
  return call("/snippets/", sending("POST", body), snippets);
}

/** A change to a snippet's name or blocks; only the fields sent change. */
export type SnippetEdit = Omit<Schemas["SnippetEdit"], "updated_at">;

/** Saves `edit` on the copy of `snippet` it was made on; a newer copy answers 409. */
export function editSnippet(snippet: Snippet, edit: SnippetEdit): Promise<Reply<Snippet[]>> {
  const body: Schemas["SnippetEdits"] = { snippets: [{ ...edit, updated_at: snippet.updated_at }] };
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
