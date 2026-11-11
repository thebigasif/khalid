import type { SiteHealth } from "../api/seo";
import { Icon } from "../components/Icon";
import { dashboardWords as words } from "../strings/dashboardWords";
import { day } from "../strings/dates";

/** The site health card: the last audit's score out of 100, what it found and how it moved. */
export function HealthCard({ health }: { health: SiteHealth }) {
  const last = health.last ?? null;
  const [first] = health.trend;
  const moved = first !== undefined && last !== null && health.trend.length > 1;
  return (
    <li className={last !== null && last.errors > 0 ? "metric failing" : "metric"}>
      <span className="metric-label">
        <Icon name="shield" />
        {words.health}
      </span>
      {last === null ? (
        <p>{words.healthNone}</p>
      ) : (
        <>
          <strong className="metric-value">{words.healthScore(last.score)}</strong>
          <p>
            {words.healthWhat} {words.healthFound(last.errors, last.warnings, last.pages)}
            {moved ? ` ${words.healthSince(last.score - first.score, day(first.at))}` : null}
          </p>
        </>
      )}
    </li>
  );
}
