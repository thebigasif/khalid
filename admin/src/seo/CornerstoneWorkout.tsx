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
import { addLink, pillarOf } from "./workouts";

const words = seoToolWords.workouts;

/** The cornerstone workout: link every post of a topic cluster to the cluster's main post. */
export function CornerstoneWorkout() {
  const rows = useRows({ filter: "cluster:member", limit: 20 });
  if (isFailure(rows)) {
    return <Problem message={problemMessage(rows.problem, en.problems.unreachable)} />;
  }
  if (!isPage(rows)) {
    return <p role="status">{words.loading}</p>;
  }
  return (
    <>
      <p>{words.cornerstone.what}</p>
      <p className="hint">{words.cornerstone.why}</p>
      {rows.rows.length === 0 ? (
        <p role="status">{words.cornerstone.none}</p>
      ) : (
        <>
          <p>
            <strong>{words.cornerstone.step}</strong> {words.how}
          </p>
          <ol className="rows">
            {rows.rows.map((row) => (
              <MemberStep key={row.id} row={row} />
            ))}
          </ol>
        </>
      )}
    </>
  );
}

/** One post of a cluster: done when it links to its pillar, or a button that adds the link. */
function MemberStep({ row }: { row: PostRow }) {
  const read = useQuery({
    queryKey: ["link-suggestions", row.id, row.updated_at],
    queryFn: () => readLinkSuggestions(row.id),
  }).data;
  const [said, setSaid] = useState<string | null>(null);
  const [linked, setLinked] = useState(false);
  const pillar = isData(read) ? pillarOf(read.data) : null;

  const link = async () => {
    if (pillar === null) {
      return;
    }
    const why = await addLink(row.id, pillar, pillar.anchor ?? null);
    setSaid(why ?? words.linkedTo(pillar.title));
    setLinked(why === null);
  };

  return (
    <li>
      <div>
        <strong>{row.title}</strong>
        <span role="status" className="hint">
          {said}
        </span>
      </div>
      {isData(read) && pillar === null ? <span className="pill ok">{words.already}</span> : null}
      {pillar === null || linked ? null : (
        <button
          type="button"
          className="btn"
          aria-label={words.linkToFor(row.title, pillar.title)}
          onClick={link}
        >
          {words.linkTo(pillar.title)}
        </button>
      )}
      <Link to="/posts/$id" params={{ id: row.id }} aria-label={words.openFor(row.title)}>
        {words.open}
      </Link>
    </li>
  );
}
