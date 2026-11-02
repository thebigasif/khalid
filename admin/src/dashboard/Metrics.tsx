import { Link } from "@tanstack/react-router";
import type { Visitors } from "../api/analytics";
import { Icon } from "../components/Icon";
import { dashboardWords as words } from "../strings/dashboardWords";
import { count } from "../strings/dates";

type Props = {
  live: number;
  drafts: number;
  failing: number;
  /** The last 28 days of visits; `null` when the blog cannot count them yet. */
  visits: Visitors | null;
};

/** The row of cards at the top of the dashboard: the blog's numbers, each with where to go next. */
export function Metrics({ live, drafts, failing, visits }: Props) {
  return (
    <section aria-labelledby="numbers-title">
      <h2 id="numbers-title" className="sr-only">
        {words.numbers}
      </h2>
      <ul className="metrics">
        <li className="metric">
          <span className="metric-label">
            <Icon name="posts" />
            {words.live}
          </span>
          <strong className="metric-value">{count(live)}</strong>
          <Link to="/posts" search={{ stage: "live" }}>
            {words.liveLink}
          </Link>
        </li>
        <li className="metric">
          <span className="metric-label">
            <Icon name="clock" />
            {words.drafts}
          </span>
          <strong className="metric-value">{count(drafts)}</strong>
          <Link to="/posts">{words.draftsLink}</Link>
        </li>
        <li className={failing === 0 ? "metric" : "metric failing"}>
          <span className="metric-label">
            <Icon name="alert" />
            {words.failing}
          </span>
          <strong className="metric-value">{count(failing)}</strong>
          {failing === 0 ? (
            <p>{words.allPass}</p>
          ) : (
            <Link to="/posts" search={{ checks: "blocked" }}>
              {words.failingLink}
            </Link>
          )}
        </li>
        {visits === null ? (
          <li className="metric connect">
            <span className="metric-label">
              <Icon name="search" />
              {words.searchClicks}
            </span>
            <p>{words.connectWhy}</p>
            <Link to="/search-console">{words.connect}</Link>
          </li>
        ) : (
          <li className="metric">
            <span className="metric-label">
              <Icon name="analytics" />
              {words.visits}
            </span>
            <strong className="metric-value">{count(visits.views)}</strong>
            <Link to="/analytics">{words.visitsLink}</Link>
          </li>
        )}
      </ul>
    </section>
  );
}
