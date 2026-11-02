import { useQuery } from "@tanstack/react-query";
import { Link } from "@tanstack/react-router";
import { isData } from "../api/call";
import { readSiteIssues, type SiteIssue } from "../api/seo";
import { dashboardWords as words } from "../strings/dashboardWords";

/** The most issues the dashboard lists; the posts list holds the rest. */
const MOST = 5;

/** The pill for each weight: red stops a post, amber warns, blue is a tip. */
const PILL: Record<SiteIssue["weight"], string> = {
  block: "pill block",
  warn: "pill warn",
  tip: "pill info",
};

/** The site's most pressing lint problems as a task list, each with its fix and a post to start on. */
export function SiteIssues() {
  const issues = useQuery({ queryKey: ["site-issues"], queryFn: readSiteIssues }).data;
  // A blog that cannot send them, such as an older one, leaves the panel out.
  if (!isData(issues) || issues.data.length === 0) {
    return null;
  }
  return (
    <section className="panel" aria-labelledby="issues-title">
      <h2 id="issues-title">{words.issues}</h2>
      <p className="hint">{words.issuesLede}</p>
      <ol className="rows">
        {issues.data.slice(0, MOST).map((issue) => (
          <li key={issue.rule}>
            <div>
              <strong>{issue.what}</strong>
              <span className="hint">
                {issue.why} {issue.fix}
              </span>
            </div>
            <span className={PILL[issue.weight]}>{words.onPosts(issue.posts)}</span>
            <Link
              to="/posts/$id"
              params={{ id: issue.example }}
              aria-label={words.startOn(issue.what)}
            >
              {words.start}
            </Link>
          </li>
        ))}
      </ol>
    </section>
  );
}
