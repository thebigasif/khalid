import { createApi } from "./client";
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
