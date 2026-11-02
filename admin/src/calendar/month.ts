import type { CalendarDay, CalendarPost } from "../api/calendar";

/** A month: its year, and its number from 1 for January to 12. */
export type Month = { year: number; month: number };

function pad(value: number): string {
  return String(value).padStart(2, "0");
}

/** The day `day` of `month` as the API writes it, `YYYY-MM-DD`; day 0 is the day before the 1st. */
export function isoDay(month: Month, day: number): string {
  const at = new Date(Date.UTC(month.year, month.month - 1, day));
  return `${at.getUTCFullYear()}-${pad(at.getUTCMonth() + 1)}-${pad(at.getUTCDate())}`;
}

/** Today in the writer's time zone, `YYYY-MM-DD`. */
export function localToday(now: Date = new Date()): string {
  return `${now.getFullYear()}-${pad(now.getMonth() + 1)}-${pad(now.getDate())}`;
}

/** The month the day `day`, `YYYY-MM-DD`, falls in. */
export function monthOf(day: string): Month {
  const [year = 1970, month = 1] = day.split("-").map(Number);
  return { year, month };
}

/** The month `steps` months after `month`, or before it when `steps` is below zero. */
export function shift(month: Month, steps: number): Month {
  const at = new Date(Date.UTC(month.year, month.month - 1 + steps, 1));
  return { year: at.getUTCFullYear(), month: at.getUTCMonth() + 1 };
}

/**
 * The weeks a month shows, Monday first: whole weeks from the one holding
 * the 1st to the one holding the last day, so four to six rows of seven.
 */
export function weeks(month: Month): string[][] {
  const first = new Date(Date.UTC(month.year, month.month - 1, 1));
  const before = (first.getUTCDay() + 6) % 7;
  const days = new Date(Date.UTC(month.year, month.month, 0)).getUTCDate();
  const rows: string[][] = [];
  for (let start = 1 - before; start <= days; start += 7) {
    rows.push(Array.from({ length: 7 }, (_, offset) => isoDay(month, start + offset)));
  }
  return rows;
}

/** The first and the last day a month shows, for the calendar request. */
export function span(month: Month): { from: string; to: string } {
  const shown = weeks(month).flat();
  return { from: shown[0] ?? isoDay(month, 1), to: shown.at(-1) ?? isoDay(month, 1) };
}

/** Whether `day`, `YYYY-MM-DD`, is in `month`. */
export function inMonth(day: string, month: Month): boolean {
  return day.startsWith(`${month.year}-${pad(month.month)}-`);
}

/** Whether a post may be scheduled for `day`: today or a day to come. */
export function canSchedule(day: string, today: string): boolean {
  return /^\d{4}-\d{2}-\d{2}$/.test(day) && day >= today;
}

/** The posts of each day the calendar holds, by day. */
export function byDay(days: CalendarDay[]): Map<string, CalendarPost[]> {
  return new Map(days.map((day) => [day.date, day.posts]));
}

/** The drafts a writer can give a day: those with none, then those on a day this month. */
export function draftsToSchedule(days: CalendarDay[], undated: CalendarPost[]): CalendarPost[] {
  const dated = days.flatMap((day) => day.posts).filter((post) => post.status === "draft");
  return [...undated, ...dated];
}
