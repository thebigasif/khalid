import { type ChangeEvent, type FormEvent, useState } from "react";
import { type Days, PERIODS } from "../api/analytics";
import { en } from "../strings/en";

type Props = { days: Days; onDays: (days: Days) => void };

/** The days the select names, or the screen's default when it names none. */
function picked(value: string): Days {
  return PERIODS.find((days) => String(days) === value) ?? 28;
}

/** The choice of period a report covers, sent with its button. */
export function PeriodForm({ days, onDays }: Props) {
  const words = en.analytics;
  const [choice, setChoice] = useState<Days>(days);
  const pick = (event: ChangeEvent<HTMLSelectElement>) => setChoice(picked(event.target.value));
  const submit = (event: FormEvent) => {
    event.preventDefault();
    onDays(choice);
  };
  return (
    <form className="period" onSubmit={submit}>
      <div className="field">
        <label htmlFor="days">{words.days}</label>
        <select id="days" value={choice} onChange={pick}>
          {PERIODS.map((option) => (
            <option key={option} value={option}>
              {words.period(option)}
            </option>
          ))}
        </select>
      </div>
      <button type="submit" className="btn primary">
        {words.show}
      </button>
    </form>
  );
}
