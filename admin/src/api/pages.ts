// The page routes of the Admin API: the site's pages, such as About.
import { call, nothing, type Reply, sending } from "./call";
import type { components } from "./schema";

/** A page in one language, as its file in the site folder holds it. */
export type AdminPage = components["schemas"]["AdminPage"];

/** A new page; the language is the site's default when left out. */
export type NewPage = components["schemas"]["NewPage"];

/** A change to a page: only the fields sent change; an empty description removes it. */
export type PageEdit = components["schemas"]["PageEdit"];

async function pages(response: Response): Promise<AdminPage[]> {
  const body: components["schemas"]["AdminPages"] = await response.json();
  return body.pages;
}

/** Every page in every language, ordered by language and slug. */
export function listPages(): Promise<Reply<AdminPage[]>> {
  return call("/pages/", {}, pages);
}

/** One page, by language and slug, such as `en.about`. */
export function readPage(id: string): Promise<Reply<AdminPage[]>> {
  return call(`/pages/${encodeURIComponent(id)}/`, {}, pages);
}

/** Makes a page's file; the page is live at once. */
export function createPage(page: NewPage): Promise<Reply<AdminPage[]>> {
  const body: components["schemas"]["NewPages"] = { pages: [page] };
  return call("/pages/", sending("POST", body), pages);
}

/** Saves an edit made on the copy that `edit.updated_at` names. */
export function editPage(id: string, edit: PageEdit): Promise<Reply<AdminPage[]>> {
  const body: components["schemas"]["PageEdits"] = { pages: [edit] };
  return call(`/pages/${encodeURIComponent(id)}/`, sending("PUT", body), pages);
}

/** Deletes a page's file, unless it changed after the copy `updatedAt` names. */
export function deletePage(id: string, updatedAt: string): Promise<Reply<null>> {
  const query = new URLSearchParams({ updated_at: updatedAt });
  return call(`/pages/${encodeURIComponent(id)}/?${query}`, { method: "DELETE" }, nothing);
}
