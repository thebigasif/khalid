import { type DragEvent, useState } from "react";
import type { CalendarPost, PlanSlot } from "../api/calendar";
import { calendarWords } from "../strings/calendarWords";
import { calendarDay } from "../strings/dates";
import { en } from "../strings/en";
import { canSchedule, inMonth, type Month, weeks } from "./month";
import { SuggestedChip } from "./PlanPanel";
import { DRAGGED, PostChip } from "./PostChip";

// Monday 4 January 2027 starts the week the names are read from.
const WEEKDAYS = Array.from({ length: 7 }, (_, offset) => new Date(Date.UTC(2027, 0, 4 + offset)));
const SHORT = new Intl.DateTimeFormat(en.locale, { weekday: "short", timeZone: "UTC" });
const LONG = new Intl.DateTimeFormat(en.locale, { weekday: "long", timeZone: "UTC" });

type Props = {
  month: Month;
  today: string;
  posts: Map<string, CalendarPost[]>;
  /** The drafts the planner suggests, by day. */
  suggested: Map<string, PlanSlot[]>;
  /** A draft was dropped on a day it may go live. */
  onDrop: (id: string, day: string) => void;
};

/** The month as a grid of weeks, each day with its posts; a draft dropped on a day is scheduled. */
export function MonthGrid({ month, today, posts, suggested, onDrop }: Props) {
  const [over, setOver] = useState<string | null>(null);
  const dropOn = (day: string) => (event: DragEvent) => {
    event.preventDefault();
    setOver(null);
    const id = event.dataTransfer.getData(DRAGGED);
    if (id !== "") {
      onDrop(id, day);
    }
  };
  return (
    <div className="scroll">
      <table className="month" aria-labelledby="month-title">
        <thead>
          <tr>
            {WEEKDAYS.map((weekday) => (
              <th scope="col" key={weekday.toISOString()}>
                <abbr title={LONG.format(weekday)}>{SHORT.format(weekday)}</abbr>
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {weeks(month).map((week) => (
            <tr key={week[0]}>
              {week.map((day) => {
                const open = canSchedule(day, today);
                const own = posts.get(day) ?? [];
                const ideas = suggested.get(day) ?? [];
                const count = own.length + ideas.length;
                return (
                  <td
                    key={day}
                    data-day={day}
                    data-label={calendarDay(day)}
                    data-outside={inMonth(day, month) ? undefined : "true"}
                    data-today={day === today ? "true" : undefined}
                    data-over={over === day ? "true" : undefined}
                    onDragOver={
                      open
                        ? (event) => {
                            event.preventDefault();
                            setOver(day);
                          }
                        : undefined
                    }
                    onDragLeave={() => setOver(null)}
                    onDrop={open ? dropOn(day) : undefined}
                  >
                    <span className="month-date" aria-hidden="true">
                      {Number(day.slice(8))}
                    </span>
                    <span className="sr-only">
                      {calendarDay(day)}
                      {day === today ? `, ${calendarWords.today}` : ""}
                      {count > 0 ? `, ${calendarWords.count(count)}` : ""}
                    </span>
                    {count === 0 ? null : (
                      <ul>
                        {own.map((post) => (
                          <li key={post.id}>
                            <PostChip post={post} />
                          </li>
                        ))}
                        {ideas.map((slot) => (
                          <li key={`suggested-${slot.id}`}>
                            <SuggestedChip slot={slot} />
                          </li>
                        ))}
                      </ul>
                    )}
                  </td>
                );
              })}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
