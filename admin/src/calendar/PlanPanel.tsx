import { Link } from "@tanstack/react-router";
import type { Plan, PlanSlot } from "../api/calendar";
import { calendarWords } from "../strings/calendarWords";
import { calendarDay } from "../strings/dates";

type Props = {
  plan: Plan;
  /** Whether the suggestions are being scheduled now. */
  busy: boolean;
  /** Schedules each suggested draft on its day. */
  onApply: (slots: PlanSlot[]) => void;
};

/** The drafts the planner suggests a day for, earliest first. */
export function suggestions(plan: Plan): PlanSlot[] {
  return plan.calendar.filter((slot) => slot.kind === "suggested");
}

/** The suggested drafts by day, `YYYY-MM-DD`. */
export function suggestedByDay(slots: PlanSlot[]): Map<string, PlanSlot[]> {
  const days = new Map<string, PlanSlot[]>();
  for (const slot of slots) {
    days.set(slot.day, [...(days.get(slot.day) ?? []), slot]);
  }
  return days;
}

/** A draft on the day the planner suggests: a dashed chip, its title linking to the editor. */
export function SuggestedChip({ slot, dated }: { slot: PlanSlot; dated?: boolean }) {
  return (
    <span className="chip chip-suggested">
      <span className="sr-only">{calendarWords.suggested}: </span>
      <Link to="/posts/$id" params={{ id: slot.id }}>
        {slot.title}
      </Link>
      {dated === true ? <time dateTime={slot.day}>{calendarDay(slot.day)}</time> : null}
    </span>
  );
}

/**
 * The planner's suggested days: what it suggests and why, what to watch,
 * and one button that schedules them all. Nothing changes before it.
 */
export function PlanPanel({ plan, busy, onApply }: Props) {
  const words = calendarWords;
  const suggested = suggestions(plan);
  return (
    <section className="plan" aria-labelledby="plan-title">
      <h2 id="plan-title">{words.plan}</h2>
      <p className="hint">{words.planHint(plan.per_week)}</p>
      {suggested.length === 0 ? (
        <p>{words.noSuggestions}</p>
      ) : (
        <ul>
          {suggested.map((slot) => (
            <li key={slot.id}>
              <SuggestedChip slot={slot} dated />
            </li>
          ))}
        </ul>
      )}
      {plan.warnings.map((note) => (
        <div className="plan-note" key={note.what}>
          <p>
            <strong>{note.what}</strong>
          </p>
          <p>{note.why}</p>
          <p>{note.fix}</p>
        </div>
      ))}
      {suggested.length === 0 ? null : (
        <>
          <p className="hint" id="plan-hint">
            {words.applyHint}
          </p>
          <button
            type="button"
            className="btn primary"
            aria-describedby="plan-hint"
            disabled={busy}
            onClick={() => onApply(suggested)}
          >
            {words.apply(suggested.length)}
          </button>
        </>
      )}
    </section>
  );
}
