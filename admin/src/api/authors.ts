// The author routes of the Admin API, typed from the OpenAPI document. The
// calls go through `call`, which keeps the answer's problem.
import { call, nothing, type Reply, sending } from "./call";
import type { components } from "./schema";

type Schemas = components["schemas"];

/** An author in one language, as their file in the site folder holds them. */
export type AdminAuthor = Schemas["AdminAuthor"];

/** A new author; the language is the site's default when left out. */
export type NewAuthor = Schemas["NewAuthor"];

/** A change to an author: only the fields sent change; an empty role removes it. */
export type AuthorEdit = Schemas["AuthorEdit"];

async function authors(response: Response): Promise<AdminAuthor[]> {
  const body: Schemas["AdminAuthors"] = await response.json();
  return body.authors;
}

/** The slug posts name an author by, from their identity such as `en.asif`. */
export function authorSlug(id: string | null | undefined): string | undefined {
  const dot = id?.indexOf(".") ?? -1;
  return id === null || id === undefined || dot < 0 ? undefined : id.slice(dot + 1);
}

/** Every author in every language. */
export function listAuthors(): Promise<Reply<AdminAuthor[]>> {
  return call("/authors/", {}, authors);
}

/** One author, by language and slug. */
export function readAuthor(id: string): Promise<Reply<AdminAuthor[]>> {
  return call(`/authors/${encodeURIComponent(id)}/`, {}, authors);
}

/** Makes an author's file. */
export function createAuthor(author: NewAuthor): Promise<Reply<AdminAuthor[]>> {
  const body: Schemas["NewAuthors"] = { authors: [author] };
  return call("/authors/", sending("POST", body), authors);
}

/** Saves an edit made on the copy that `edit.updated_at` names. */
export function editAuthor(id: string, edit: AuthorEdit): Promise<Reply<AdminAuthor[]>> {
  const body: Schemas["AuthorEdits"] = { authors: [edit] };
  return call(`/authors/${encodeURIComponent(id)}/`, sending("PUT", body), authors);
}

/** Deletes an author's file, unless it changed after the copy `updatedAt` names. */
export function deleteAuthor(id: string, updatedAt: string): Promise<Reply<null>> {
  const query = new URLSearchParams({ updated_at: updatedAt });
  return call(`/authors/${encodeURIComponent(id)}/?${query}`, { method: "DELETE" }, nothing);
}
