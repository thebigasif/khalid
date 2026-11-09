import { useQuery } from "@tanstack/react-query";
import { Link } from "@tanstack/react-router";
import { useState } from "react";
import { isData } from "../api/call";
import { problemMessage } from "../api/posts";
import { isFailure, isPage, type PostRow, useRows } from "../api/rows";
import { readLinkSuggestions } from "../api/seo";
import { Problem } from "../components/Problem";
import { en } from "../strings/en";
import { seoToolWords } from "../strings/seoToolWords";
import { addLink, relatedTo } from "./workouts";

const words = seoToolWords.workouts;

/** The orphaned content workout: link each live post nothing links to from a post near it. */
export function OrphanWorkout() {
  const rows = useRows({ filter: "orphan:true+status:published", limit: 20 });
  const [done, setDone] = useState<string[]>([]);
  if (isFailure(rows)) {
    return <Problem message={problemMessage(rows.problem, en.problems.unreachable)} />;
  }
  if (!isPage(rows)) {
    return <p role="status">{words.loading}</p>;
  }
  return (
    <>
      <p>{words.orphans.what}</p>
      <p className="hint">{words.orphans.why}</p>
      {rows.rows.length === 0 ? (
        <p role="status">{words.orphans.none}</p>
      ) : (
        <>
          <p>
            <strong>{words.orphans.step}</strong> {words.how}
          </p>
          <p role="status">{words.progress(done.length, rows.rows.length)}</p>
          <ol className="rows">
            {rows.rows.map((row) => (
              <OrphanStep
                key={row.id}
                row={row}
                onDone={() => setDone((now) => [...now, row.id])}
              />
            ))}
          </ol>
        </>
      )}
    </>
  );
}

/** One orphan, with the posts closest to it, each with a button that links it from there. */
function OrphanStep({ row, onDone }: { row: PostRow; onDone: () => void }) {
  const read = useQuery({
    queryKey: ["link-suggestions", row.id, row.updated_at],
    queryFn: () => readLinkSuggestions(row.id),
  }).data;
  const [said, setSaid] = useState<string | null>(null);
  const [linked, setLinked] = useState(false);
  const related = isData(read) ? relatedTo(read.data) : [];
  const path = row.path;

  const link = async (from: string, title: string) => {
    if (path === undefined || path === null) {
      return;
    }
    const why = await addLink(from, { title: row.title, path }, null);
    setSaid(why ?? words.linked(title));
    if (why === null) {
      setLinked(true);
      onDone();
    }
  };

  return (
    <li>
      <div>
        <strong>{row.title}</strong>
        {isData(read) && related.length === 0 ? (
          <span className="hint">{words.noRelated}</span>
        ) : null}
        <span role="status" className="hint">
          {said}
        </span>
      </div>
      {linked
        ? null
        : related.map((each) => (
            <button
              key={each.id}
              type="button"
              className="btn"
              aria-label={words.linkFromFor(each.title, row.title)}
              onClick={() => link(each.id, each.title)}
            >
              {words.linkFrom(each.title)}
            </button>
          ))}
      <Link to="/posts/$id" params={{ id: row.id }} aria-label={words.openFor(row.title)}>
        {words.open}
      </Link>
    </li>
  );
}
