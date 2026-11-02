import { Link } from "@tanstack/react-router";
import type { RowPage } from "../api/rows";
import { EmptyState } from "../components/EmptyState";
import { StatusPill } from "../components/StatusPill";
import { dashboardWords as words } from "../strings/dashboardWords";
import { en } from "../strings/en";

type Props = {
  /** The first posts the checks stop from going live, and how many there are. */
  blocked: RowPage;
  scheduled: number;
  /** Comments waiting for a yes; `null` when the blog cannot list them. */
  waiting: number | null;
};

/** What wants the writer now: posts that fail their checks, posts about to go live, comments. */
export function NeedsAttention({ blocked, scheduled, waiting }: Props) {
  const failing = blocked.pagination.total;
  const comments = waiting ?? 0;
  const calm = failing === 0 && scheduled === 0 && comments === 0;
  return (
    <section className="panel" aria-labelledby="attention-title">
      <h2 id="attention-title">{words.attention}</h2>
      {calm ? (
        <EmptyState icon="check" title={words.allClear}>
          <Link to="/new">{words.startPost}</Link>
        </EmptyState>
      ) : (
        <ul className="rows">
          {blocked.rows.map((row) => (
            <li key={row.id}>
              <Link to="/posts/$id" params={{ id: row.id }}>
                {row.title}
              </Link>
              <span className="pill block">{words.failsChecks}</span>
            </li>
          ))}
          {failing === 0 ? null : (
            <li>
              <Link to="/posts" search={{ checks: "blocked" }}>
                {en.dashboard.showBlocked}
              </Link>
            </li>
          )}
          {scheduled === 0 ? null : (
            <li>
              <Link to="/posts" search={{ stage: "scheduled" }}>
                {words.scheduled(scheduled)}
              </Link>
              <StatusPill status="scheduled" />
            </li>
          )}
          {comments === 0 ? null : (
            <li>
              <Link to="/comments">{words.waiting(comments)}</Link>
              <span className="pill warn">{words.waitingPill}</span>
            </li>
          )}
        </ul>
      )}
    </section>
  );
}
