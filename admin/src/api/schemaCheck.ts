import { createApi } from "./client";
import { isProblem, type Problem } from "./posts";
import type { components } from "./schema";

/** The two reports on a piece of JSON-LD: correct schema.org, and what Google needs. */
export type SchemaChecks = components["schemas"]["SchemaChecks"];
/** One finding of a schema report. */
export type CheckFinding = components["schemas"]["CheckFinding"];

const api = createApi();

/**
 * Checks `jsonLd` twice, saving nothing: against the schema.org vocabulary, and against what Google
 * needs for a rich result. `visibleText` is what a reader sees, so values the page never shows are
 * flagged. Null when the blog cannot check it, such as for text that is not JSON.
 */
export async function checkSchema(
  jsonLd: string,
  visibleText?: string,
): Promise<SchemaChecks | null> {
  const body =
    visibleText === undefined
      ? { json_ld: jsonLd }
      : { json_ld: jsonLd, visible_text: visibleText };
  const { data } = await api
    .POST("/api/admin/v1/schema/checks/", { body })
    .catch(() => ({ data: undefined }));
  return data ?? null;
}

/** What the validator shows: both reports, or why the check could not be made. */
export type Checked = { checks: SchemaChecks } | { problem: Problem | null };

/**
 * Checks pasted JSON-LD, or every JSON-LD script on the page at `url`, which the blog fetches.
 * Says why when it cannot, such as a page with no schema or an address inside the server's network.
 */
export async function checkMarkup(source: { json_ld: string } | { url: string }): Promise<Checked> {
  try {
    const { data, error } = await api.POST("/api/admin/v1/schema/checks/", { body: source });
    if (data !== undefined) {
      return { checks: data };
    }
    return { problem: isProblem(error) ? error : null };
  } catch {
    return { problem: null };
  }
}
