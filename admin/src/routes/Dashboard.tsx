import { useQuery } from "@tanstack/react-query";
import { readVisitors } from "../api/analytics";
import { isData } from "../api/call";
import { listComments } from "../api/comments";
import { problemMessage } from "../api/posts";
import { isFailure, isPage, useRows } from "../api/rows";
import { blogName } from "../auth/blog";
import { PageHeader } from "../components/PageHeader";
import { Problem } from "../components/Problem";
import { Metrics } from "../dashboard/Metrics";
import { NeedsAttention } from "../dashboard/NeedsAttention";
import { QuickActions } from "../dashboard/QuickActions";
import { RecentChanges } from "../dashboard/RecentChanges";
import { dashboardWords as words } from "../strings/dashboardWords";
import { en } from "../strings/en";

/** The first screen after signing in: the blog's numbers, what needs the writer, and the drafts. */
export function Dashboard() {
  // A list counts its whole length, so one row is enough to learn a total.
  const blocked = useRows({ filter: "gate:refuses", limit: 5 });
  const drafts = useRows({ filter: "status:draft", order: "updated_at desc", limit: 5 });
  const live = useRows({ filter: "status:published", limit: 1 });
  const scheduled = useRows({ filter: "status:scheduled", limit: 1 });
  // The same queries as the Comments and Analytics screens, so each keeps the other fresh.
  const waiting = useQuery({
    queryKey: ["comments", "pending"],
    queryFn: () => listComments("pending"),
  }).data;
  const visits = useQuery({ queryKey: ["visitors", 28], queryFn: () => readVisitors(28) }).data;

  const title = words.greeting(blogName());
  const failed = [blocked, drafts, live, scheduled].find(isFailure);
  if (failed !== undefined) {
    return (
      <>
        <PageHeader title={title} />
        <Problem message={problemMessage(failed.problem, en.problems.unreachable)} />
      </>
    );
  }
  if (!(isPage(blocked) && isPage(drafts) && isPage(live) && isPage(scheduled))) {
    return (
      <>
        <PageHeader title={title} />
        <p role="status">{en.dashboard.loading}</p>
      </>
    );
  }

  const stuck = blocked.pagination.total;
  return (
    <>
      <PageHeader
        title={title}
        lede={stuck === 0 ? en.dashboard.calm : en.dashboard.blockedLede(stuck)}
      />
      <Metrics
        live={live.pagination.total}
        drafts={drafts.pagination.total}
        failing={stuck}
        visits={isData(visits) ? visits.data : null}
      />
      <div className="board">
        <NeedsAttention
          blocked={blocked}
          scheduled={scheduled.pagination.total}
          waiting={isData(waiting) ? waiting.data.length : null}
        />
        <section className="panel" aria-labelledby="drafts-title">
          <h2 id="drafts-title">{words.recentDrafts}</h2>
          <RecentChanges rows={drafts.rows} empty={words.noDrafts} />
        </section>
      </div>
      <QuickActions />
    </>
  );
}
