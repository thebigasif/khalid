import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useState } from "react";
import { type Calendar as Days, readCalendar, scheduleOn } from "../api/calendar";
import { isData, type Reply } from "../api/call";
import { problemMessage } from "../api/posts";
import { DayList } from "../calendar/DayList";
import { DraftList } from "../calendar/DraftList";
import {
  byDay,
  canSchedule,
  draftsToSchedule,
  localToday,
  type Month,
  monthOf,
  shift,
  span,
} from "../calendar/month";
import { MonthGrid } from "../calendar/MonthGrid";
import { PageHeader } from "../components/PageHeader";
import { Problem } from "../components/Problem";
import { calendarWords } from "../strings/calendarWords";
import { calendarDay } from "../strings/dates";
import { en, type UserMessage } from "../strings/en";

/** The query key the screen reads again after a draft is given its day. */
export const CALENDAR = ["calendar"];

const MONTH = new Intl.DateTimeFormat(en.locale, {
  month: "long",
  year: "numeric",
  timeZone: "UTC",
});

/** The title of the post `id` on the calendar, or its identity when it is not there. */
function titleOf(calendar: Reply<Days> | undefined, id: string): string {
  if (!isData(calendar)) {
    return id;
  }
  const all = [...calendar.data.undated, ...calendar.data.days.flatMap((day) => day.posts)];
  return all.find((post) => post.id === id)?.title ?? id;
}

/** The content calendar: what is planned, scheduled and live, a month at a time. */
export function Calendar() {
  const words = calendarWords;
  const today = localToday();
  const [month, setMonth] = useState<Month>(monthOf(today));
  const [view, setView] = useState<"month" | "list">("month");
  const [done, setDone] = useState<string | null>(null);
  const [failed, setFailed] = useState<UserMessage | null>(null);
  const queries = useQueryClient();
  const { from, to } = span(month);
  const calendar = useQuery({
    queryKey: [...CALENDAR, from, to],
    queryFn: () => readCalendar(from, to),
  }).data;

  const schedule = async (id: string, day: string) => {
    const title = titleOf(calendar, id);
    if (!canSchedule(day, today)) {
      setDone(null);
      setFailed(words.past);
      return;
    }
    const answer = await scheduleOn(id, day);
    if ("post" in answer) {
      setFailed(null);
      setDone(words.scheduled(title, calendarDay(day)));
    } else {
      setDone(null);
      setFailed(problemMessage(answer.problem, en.problems.unreachable));
    }
    await queries.invalidateQueries({ queryKey: CALENDAR });
  };

  let shown = <p role="status">{words.loading}</p>;
  if (calendar !== undefined && !isData(calendar)) {
    shown = <Problem message={problemMessage(calendar.problem, en.problems.unreachable)} />;
  } else if (isData(calendar) && view === "list") {
    shown = <DayList days={calendar.data.days} month={month} />;
  } else if (isData(calendar)) {
    shown = (
      <div className="calendar-layout">
        <MonthGrid
          month={month}
          today={today}
          posts={byDay(calendar.data.days)}
          onDrop={schedule}
        />
        <DraftList
          drafts={draftsToSchedule(calendar.data.days, calendar.data.undated)}
          today={today}
          onSchedule={schedule}
        />
      </div>
    );
  }
  return (
    <>
      <PageHeader title={words.title} lede={words.lede}>
        <div className="switch" role="group" aria-label={words.views}>
          {(["month", "list"] as const).map((each) => (
            <button
              key={each}
              type="button"
              aria-pressed={view === each}
              onClick={() => setView(each)}
            >
              {words[each]}
            </button>
          ))}
        </div>
      </PageHeader>
      <div className="bar">
        <h2 id="month-title" className="month-title">
          {MONTH.format(new Date(Date.UTC(month.year, month.month - 1, 1)))}
        </h2>
        <div className="flex flex-wrap gap-2">
          <button type="button" className="btn" onClick={() => setMonth(shift(month, -1))}>
            {words.previous}
          </button>
          <button type="button" className="btn" onClick={() => setMonth(monthOf(today))}>
            {words.thisMonth}
          </button>
          <button type="button" className="btn" onClick={() => setMonth(shift(month, 1))}>
            {words.next}
          </button>
        </div>
      </div>
      {failed === null ? null : <Problem message={failed} />}
      <p role="status" className={done === null ? "sr-only" : "done"}>
        {done}
      </p>
      {shown}
    </>
  );
}
