import { useQuery } from "@tanstack/react-query";
import { problemMessage } from "../api/posts";
import { isData } from "../api/redirects";
import { readReport } from "../api/searchConsole";
import { Problem } from "../components/Problem";
import { en } from "../strings/en";
import { searchConsoleWords } from "../strings/searchConsoleWords";
import { PostMonth } from "./PostMonth";
import { ThisWeek } from "./ThisWeek";
import { ToFix } from "./ToFix";

/** The rest of what Search Console says: the week, what to fix, and each post's month. */
export function Report() {
  // Google is asked once per visit, as for the follow-ups.
  const reply = useQuery({
    queryKey: ["search-console-report"],
    queryFn: readReport,
    staleTime: 5 * 60 * 1000,
  }).data;
  if (reply === undefined) {
    return <p role="status">{searchConsoleWords.loading}</p>;
  }
  if (!isData(reply)) {
    return <Problem message={problemMessage(reply.problem, en.problems.unreachable)} />;
  }
  const report = reply.data;
  return (
    <>
      <ThisWeek report={report} />
      <ToFix report={report} />
      {report.posts.length > 0 && <PostMonth posts={report.posts} />}
    </>
  );
}
