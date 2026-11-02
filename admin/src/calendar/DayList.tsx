import type { CalendarDay } from "../api/calendar";
import { calendarWords } from "../strings/calendarWords";
import { calendarDay } from "../strings/dates";
import { inMonth, type Month } from "./month";
import { PostChip } from "./PostChip";

/** The month as a list: each day with a post, earliest first, and its posts. */
export function DayList({ days, month }: { days: CalendarDay[]; month: Month }) {
  const shown = days.filter((day) => inMonth(day.date, month));
  if (shown.length === 0) {
    return <p role="status">{calendarWords.none}</p>;
  }
  return (
    <ol className="agenda" aria-labelledby="month-title">
      {shown.map((day) => (
        <li key={day.date}>
          <h3>
            <time dateTime={day.date}>{calendarDay(day.date)}</time>
          </h3>
          <ul>
            {day.posts.map((post) => (
              <li key={post.id}>
                <PostChip post={post} />
              </li>
            ))}
          </ul>
        </li>
      ))}
    </ol>
  );
}
