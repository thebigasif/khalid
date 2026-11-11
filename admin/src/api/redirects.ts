// The redirect routes of the Admin API (KH-SEO-8), typed from the OpenAPI
// document. The calls go through `call`, which keeps the answer's problem.
import { call, nothing, type Reply, sending } from "./call";
import type { components } from "./schema";

export { type Failure, isData, type Reply } from "./call";

type Schemas = components["schemas"];

/** A redirect Khalid keeps in its database, as the Admin API sends it. */
export type StoredRedirect = Schemas["AdminRedirect"];

/** An address on the blog that answered 404, with the page nearest to it. */
export type Miss = Schemas["AdminMiss"];

async function redirects(response: Response): Promise<StoredRedirect[]> {
  const body: Schemas["AdminRedirects"] = await response.json();
  return body.redirects;
}

/** Every redirect Khalid keeps, oldest first; aliases in post files are not among them. */
export function listRedirects(): Promise<Reply<StoredRedirect[]>> {
  return call("/redirects/", {}, redirects);
}

/** Removes one kept redirect. */
export function removeRedirect(id: number): Promise<Reply<null>> {
  return call(`/redirects/${id}/`, { method: "DELETE" }, nothing);
}

/** Stores one redirect from `from`: a 301 to `to`, or a 410 when `to` is `null`. */
export function addRedirect(from: string, to: string | null): Promise<Reply<StoredRedirect[]>> {
  const redirect = to === null ? { from, status: 410 } : { from, to, status: 301 };
  const body: Schemas["NewRedirects"] = { redirects: [redirect] };
  return call("/redirects/", sending("POST", body), redirects);
}

/** Stores every redirect in a CSV list, or none; gives back those stored. */
export function importRedirects(csv: string): Promise<Reply<StoredRedirect[]>> {
  const init = { method: "POST", headers: { "Content-Type": "text/csv" }, body: csv };
  return call("/redirects/import/", init, redirects);
}

/** Every kept redirect as CSV text, header first. */
export function exportRedirects(): Promise<Reply<string>> {
  return call("/redirects/export/", {}, (response) => response.text());
}

/** The addresses that answered 404, most visited first. */
export function listMisses(): Promise<Reply<Miss[]>> {
  return call("/not-found/", {}, async (response) => {
    const body: Schemas["AdminMisses"] = await response.json();
    return body.misses;
  });
}

/** Sends an address that answered 404 to `to` for good, and drops it from the log. */
export function redirectMiss(path: string, to: string): Promise<Reply<StoredRedirect[]>> {
  const body: Schemas["RedirectMiss"] = { path, to };
  return call("/not-found/redirect/", sending("POST", body), redirects);
}
