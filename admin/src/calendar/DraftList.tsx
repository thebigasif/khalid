import { useEffect, useId, useRef, useState } from "react";
import type { CalendarPost } from "../api/calendar";
import { calendarWords } from "../strings/calendarWords";
import { PostChip } from "./PostChip";

type Props = {
  drafts: CalendarPost[];
  today: string;
  onSchedule: (id: string, day: string) => void;
};

/**
 * The drafts a writer can give a day, each draggable onto the month and
 * each with a Schedule button that asks for the day: the way with a
 * keyboard, and the way on a phone.
 */
export function DraftList({ drafts, today, onSchedule }: Props) {
  const words = calendarWords;
  const [asking, setAsking] = useState<string | null>(null);
  return (
    <section className="drafts" aria-labelledby="drafts-title">
      <h2 id="drafts-title">{words.drafts}</h2>
      <p className="hint">{words.draftsHint}</p>
      {drafts.length === 0 ? (
        <p>{words.noDrafts}</p>
      ) : (
        <ul>
          {drafts.map((post) => (
            <li key={post.id}>
              <PostChip post={post} />
              {asking === post.id ? (
                <DayForm
                  post={post}
                  today={today}
                  onSend={(day) => {
                    setAsking(null);
                    onSchedule(post.id, day);
                  }}
                  onCancel={() => setAsking(null)}
                />
              ) : (
                <button
                  type="button"
                  className="btn"
                  aria-label={words.scheduleFor(post.title)}
                  onClick={() => setAsking(post.id)}
                >
                  {words.schedule}
                </button>
              )}
            </li>
          ))}
        </ul>
      )}
    </section>
  );
}

type FormProps = {
  post: CalendarPost;
  today: string;
  onSend: (day: string) => void;
  onCancel: () => void;
};

/** Asks for the day one draft goes live. */
function DayForm({ post, today, onSend, onCancel }: FormProps) {
  const words = calendarWords;
  const [day, setDay] = useState(today);
  const field = useId();
  const hint = useId();
  const input = useRef<HTMLInputElement>(null);
  // The day is the one thing this form asks, so it opens ready to type.
  useEffect(() => {
    input.current?.focus();
  }, []);
  return (
    <form
      className="day-form"
      aria-label={words.scheduleFor(post.title)}
      onSubmit={(event) => {
        event.preventDefault();
        onSend(day);
      }}
    >
      <label htmlFor={field}>{words.day}</label>
      <span className="hint" id={hint}>
        {words.dayHint}
      </span>
      <input
        id={field}
        type="date"
        min={today}
        required
        value={day}
        aria-describedby={hint}
        ref={input}
        onChange={(event) => setDay(event.target.value)}
      />
      <div className="flex flex-wrap gap-3">
        <button type="submit" className="btn primary">
          {words.send}
        </button>
        <button type="button" className="btn" onClick={onCancel}>
          {words.cancel}
        </button>
      </div>
    </form>
  );
}
