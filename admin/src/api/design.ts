// The Design route of the Admin API, `/design/`, typed from the OpenAPI
// document. The answer wraps one object, `{ "design": ... }`.
import { call, type Reply, sending } from "./call";
import type { components } from "./schema";

type Schemas = components["schemas"];

/** One font a body or heading may use. */
export type FontChoice = Schemas["FontChoice"];

/** One colour on one ground, with its WCAG contrast. */
export type ContrastPair = Schemas["ContrastPair"];

/** The colours, fonts, shape and logo the blog's pages use. */
export type Design = Schemas["AdminDesign"];

/** A change: only the tokens sent change. */
export type DesignChange = Schemas["DesignChange"];

async function design(response: Response): Promise<Design> {
  const body: Schemas["SiteDesign"] = await response.json();
  return body.design;
}

/** The design as the blog's pages use it now. */
export function readDesign(): Promise<Reply<Design>> {
  return call("/design/", {}, design);
}

/** Saves a change made on the copy that `change.updated_at` names. */
export function changeDesign(change: DesignChange): Promise<Reply<Design>> {
  const body: Schemas["DesignChanges"] = { design: change };
  return call("/design/", sending("PUT", body), design);
}
