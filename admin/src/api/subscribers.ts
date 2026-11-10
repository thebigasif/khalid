import { call, nothing, type Reply, sending } from "./call";
import type { components } from "./schema";

/** A reader as the Admin API sends them. */
export type AdminSubscriber = components["schemas"]["AdminSubscriber"];
/** Where a reader stands: pending, confirmed or unsubscribed. */
export type SubscriberStatus = components["schemas"]["SubscriberStatus"];
/** One newsletter with its readers and sign-up form. */
export type AdminNewsletter = components["schemas"]["AdminNewsletter"];
/** The totals of one email that went out. */
export type AdminSend = components["schemas"]["AdminSend"];
/** Whether the site can send now, and if not, what it lacks. */
export type SendReadiness = components["schemas"]["SendReadiness"];
/** The newsletters, whether the site can send, and what went out. */
export type Newsletters = components["schemas"]["AdminNewsletters"];
/** Where a page of readers sits in the whole list. */
export type ReaderPagination = components["schemas"]["Pagination"];

/** Which readers to list; every part left out matches everyone. */
export type ReaderQuery = {
  status?: SubscriberStatus;
  list?: string;
  q?: string;
  page: number;
};

/** One page of readers and where it sits. */
export type ReaderPage = { readers: AdminSubscriber[]; pagination: ReaderPagination };

/** How many readers one page holds. */
export const READERS_PER_PAGE = 50;

// An answer with no list counts as a failure, so the screen shows a problem instead of breaking.
async function one(response: Response): Promise<AdminSubscriber> {
  const body: Partial<components["schemas"]["AdminSubscribers"]> = await response.json();
  const [reader] = body.subscribers ?? [];
  if (reader === undefined) {
    throw new TypeError("The answer has no subscriber.");
  }
  return reader;
}

/** One page of readers, newest first. */
export function listReaders(query: ReaderQuery): Promise<Reply<ReaderPage>> {
  const params = new URLSearchParams({ page: String(query.page), limit: String(READERS_PER_PAGE) });
  for (const [name, value] of [
    ["status", query.status],
    ["list", query.list],
    ["q", query.q?.trim()],
  ] as const) {
    if (value !== undefined && value !== "") {
      params.set(name, value);
    }
  }
  return call(`/subscribers/?${params}`, {}, async (response) => {
    const body: Partial<components["schemas"]["SubscriberList"]> = await response.json();
    if (!Array.isArray(body.subscribers) || body.meta === undefined) {
      throw new TypeError("The answer has no list of subscribers.");
    }
    return { readers: body.subscribers, pagination: body.meta.pagination };
  });
}

/** Gives a reader a label, or takes it away. */
export function changeLabel(
  id: number,
  label: string,
  change: "add" | "remove",
): Promise<Reply<AdminSubscriber>> {
  const method = change === "add" ? "PUT" : "DELETE";
  return call(`/subscribers/${id}/labels/${encodeURIComponent(label)}/`, { method }, one);
}

/** Forgets a reader: their address, newsletters and labels go for good. */
export function forgetReader(id: number): Promise<Reply<null>> {
  return call(`/subscribers/${id}/`, { method: "DELETE" }, nothing);
}

/** The newsletters, whether the site can send, and the emails that went out. */
export function listNewsletters(): Promise<Reply<Newsletters>> {
  return call("/newsletters/", {}, async (response) => {
    const body: Partial<Newsletters> = await response.json();
    if (!Array.isArray(body.newsletters) || !Array.isArray(body.sends) || !body.sending) {
      throw new TypeError("The answer has no list of newsletters.");
    }
    return { newsletters: body.newsletters, sends: body.sends, sending: body.sending };
  });
}

/** Sends a published post to a newsletter's readers who carry every one of `labels`, once. */
export function sendPost(list: string, post: string, labels: string[]): Promise<Reply<string>> {
  const body: components["schemas"]["NewsletterSend"] = { post, labels };
  return call(
    `/newsletters/${encodeURIComponent(list)}/sends/`,
    sending("POST", body),
    async (response) => {
      const started: Partial<components["schemas"]["SendStarted"]> = await response.json();
      if (typeof started.key !== "string") {
        throw new TypeError("The answer has no send key.");
      }
      return started.key;
    },
  );
}
