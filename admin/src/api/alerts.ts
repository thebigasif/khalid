// The Admin API's alerts route: what Khalid raised for the site's owner,
// so the dashboard shows it to an owner with no email set up.
import { call, type Reply } from "./call";
import type { components } from "./schema";

/** The findings of one kind still raised, with what, why and fix. */
export type RaisedAlert = components["schemas"]["RaisedAlert"];

/** The alerts still raised, one per kind, in the order alerts go out. */
export function readAlerts(): Promise<Reply<RaisedAlert[]>> {
  return call("/alerts/", {}, async (response) => {
    const body: { alerts?: unknown } = await response.json();
    return Array.isArray(body.alerts) ? body.alerts : [];
  });
}
