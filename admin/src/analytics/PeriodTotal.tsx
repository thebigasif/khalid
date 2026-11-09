import type { Days, Visitors } from "../api/analytics";
import { count } from "../strings/dates";
import { en } from "../strings/en";
import { PeriodForm } from "./PeriodForm";

type Props = { counts: Visitors; days: Days; onDays: (days: Days) => void };

/** How the period compares with the one before, in one sentence. */
function change({ views, before, days }: Visitors): string {
  const words = en.analytics;
  if (before === 0) {
    return views === 0 ? words.level(days) : words.fresh(days);
  }
  const percent = Math.round((Math.abs(views - before) / before) * 100);
  if (percent === 0) {
    return words.level(days);
  }
  const say = views > before ? words.rise : words.fall;
  return say(percent, days, count(before));
}

/** The focus of the screen: the page views of the period, and the choice of period. */
export function PeriodTotal({ counts, days, onDays }: Props) {
  const words = en.analytics;
  return (
    <section className="focus" aria-labelledby="total-title">
      <h2 id="total-title">{words.total(counts.days)}</h2>
      <p className="big">{count(counts.views)}</p>
      <p>{change(counts)}</p>
      <p className="muted">{words.viewMeans}</p>
      <PeriodForm days={days} onDays={onDays} />
    </section>
  );
}
