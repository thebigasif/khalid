import { describe, expect, it } from "vitest";
import type { CalendarPost } from "../api/calendar";
import {
  canSchedule,
  draftsToSchedule,
  inMonth,
  localToday,
  monthOf,
  shift,
  span,
  weeks,
} from "./month";

const NOVEMBER = { year: 2026, month: 11 };

describe("a month on the calendar", () => {
  it("shows whole weeks from Monday, the 1st in the first row and the last day in the last", () => {
    const rows = weeks(NOVEMBER);
    // 1 November 2026 is a Sunday, so the first row starts on Monday 26 October.
    expect(rows[0]).toEqual([
      "2026-10-26",
      "2026-10-27",
      "2026-10-28",
      "2026-10-29",
      "2026-10-30",
      "2026-10-31",
      "2026-11-01",
    ]);
    expect(rows.at(-1)?.at(-1)).toBe("2026-12-06");
    expect(rows).toHaveLength(6);
    expect(span(NOVEMBER)).toEqual({ from: "2026-10-26", to: "2026-12-06" });
  });

  it("starts a month whose 1st is a Monday on the 1st", () => {
    expect(weeks({ year: 2027, month: 2 })[0]?.[0]).toBe("2027-02-01");
    expect(weeks({ year: 2027, month: 2 })).toHaveLength(4);
  });

  it("moves across the turn of the year", () => {
    expect(shift({ year: 2026, month: 12 }, 1)).toEqual({ year: 2027, month: 1 });
    expect(shift({ year: 2027, month: 1 }, -1)).toEqual({ year: 2026, month: 12 });
    expect(monthOf("2026-11-10")).toEqual(NOVEMBER);
  });

  it("knows the days of the month and the days a post may still go live", () => {
    expect(inMonth("2026-11-30", NOVEMBER)).toBe(true);
    expect(inMonth("2026-12-01", NOVEMBER)).toBe(false);
    expect(canSchedule("2026-11-10", "2026-11-10")).toBe(true);
    expect(canSchedule("2026-11-09", "2026-11-10")).toBe(false);
    expect(canSchedule("", "2026-11-10")).toBe(false);
    expect(localToday(new Date(2026, 10, 2, 8, 30))).toBe("2026-11-02");
  });

  it("offers the drafts with no day first, then the drafts on a day", () => {
    const post = (id: string, status: CalendarPost["status"]): CalendarPost => ({
      id,
      title: id,
      status,
      review_state: "draft",
    });
    const posts = [post("live", "published"), post("dated", "draft")];
    const days = [{ date: "2026-11-12", posts }];
    expect(draftsToSchedule(days, [post("undated", "draft")]).map((each) => each.id)).toEqual([
      "undated",
      "dated",
    ]);
  });
});
