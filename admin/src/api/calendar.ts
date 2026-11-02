// The calendar route of the Admin API (KH-EDT-4): posts by day. It only
// reads; giving a draft its day is an edit to the post.
import { call, type Reply } from "./call";
import { type Answer, readPost, savePost } from "./posts";
import type { components } from "./schema";

/** Drafts, scheduled posts and live posts by day, as the Admin API sends them. */
export type Calendar = components["schemas"]["Calendar"];
/** One post on the calendar. */
export type CalendarPost = components["schemas"]["CalendarPost"];
/** The posts on one day. */
export type CalendarDay = components["schemas"]["CalendarDay"];

/** The calendar from `from` to `to`, both `YYYY-MM-DD` and at most 92 days apart. */
export function readCalendar(from: string, to: string): Promise<Reply<Calendar>> {
  const query = new URLSearchParams({ from, to });
  return call(`/calendar/?${query}`, {}, async (response) => {
    const body: components["schemas"]["CalendarBody"] = await response.json();
    return body.calendar;
  });
}

/** The hour a scheduled day goes live, in the writer's time zone: noon keeps it on that day everywhere. */
export const GO_LIVE_HOUR = "12:00";

/**
 * Schedules the post `id` to go live on `day`, `YYYY-MM-DD`, at noon in the
 * writer's time zone. The post is read first, because an edit names the
 * copy it was made on.
 */
export async function scheduleOn(id: string, day: string): Promise<Answer> {
  const read = await readPost(id);
  if (!("post" in read)) {
    return read;
  }
  const at = new Date(`${day}T${GO_LIVE_HOUR}`);
  return savePost(id, {
    updated_at: read.post.updated_at,
    status: "scheduled",
    published_at: at.toISOString(),
  });
}
