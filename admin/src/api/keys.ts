// The key routes of the Admin API, typed from the OpenAPI document. The
// calls go through `call`, which keeps the answer's problem.
import { call, nothing, type Reply, sending } from "./call";
import type { components } from "./schema";

type Schemas = components["schemas"];

/** A key that works. Its secret is in `key` only in the answer that made it. */
export type AdminKey = Schemas["AdminKey"];

/** The kinds of key the API makes; staff tokens come from signing in. */
export type NewKeyKind = Schemas["NewKeyKind"];

/** A new key: a content key takes no scopes. */
export type NewKey = Schemas["NewKey"];

async function keys(response: Response): Promise<AdminKey[]> {
  const body: Schemas["AdminKeys"] = await response.json();
  return body.keys;
}

/** Every key that works, oldest first, never with its secret. */
export function listKeys(): Promise<Reply<AdminKey[]>> {
  return call("/keys/", {}, keys);
}

/** Makes a key; the answer holds the key itself this once. */
export function createKey(key: NewKey): Promise<Reply<AdminKey[]>> {
  const body: Schemas["NewKeys"] = { keys: [key] };
  return call("/keys/", sending("POST", body), keys);
}

/** Switches a key off at once; the API refuses the key making the request. */
export function revokeKey(id: string): Promise<Reply<null>> {
  return call(`/keys/${encodeURIComponent(id)}/`, { method: "DELETE" }, nothing);
}
