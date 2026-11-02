import { Link } from "@tanstack/react-router";
import type { PostRow } from "../api/rows";
import { lightOf } from "../editor/LintPanel";
import { day } from "../strings/dates";
import { en } from "../strings/en";
import { seoEditorWords } from "../strings/seoEditorWords";
import { CheckPills } from "./CheckPills";
import { stageOf } from "./stages";

type Props = {
  rows: PostRow[];
  /** The identities of the ticked posts. */
  ticked: string[];
  onTick: (id: string, on: boolean) => void;
};

/** A score as its light, its number, and what the light means in words. */
function ScoreCell({ value }: { value: number | undefined }) {
  if (value === undefined) {
    return <span className="muted">{seoEditorWords.notCounted}</span>;
  }
  const light = lightOf(value);
  return (
    <span className="score-cell">
      <span className={`light ${light}`} aria-hidden="true" />
      {value} <span className="muted">{seoEditorWords.state[light]}</span>
    </span>
  );
}

/** One page of posts: tick box, title, author, checks, stage, index, scores, links, change. */
export function PostTable({ rows, ticked, onTick }: Props) {
  return (
    <div className="scroll">
      <table className="wide">
        <thead>
          <tr>
            <th scope="col">
              <span className="sr-only">{en.bulk.tickColumn}</span>
            </th>
            <th scope="col">{en.posts.titleColumn}</th>
            <th scope="col">{en.posts.author}</th>
            <th scope="col">{en.posts.checks}</th>
            <th scope="col">{en.posts.review}</th>
            <th scope="col">{en.posts.index}</th>
            <th scope="col">{seoEditorWords.seo}</th>
            <th scope="col">{seoEditorWords.readability}</th>
            <th scope="col">{seoEditorWords.linksColumn}</th>
            <th scope="col">{en.posts.changed}</th>
          </tr>
        </thead>
        <tbody>
          {rows.map((row) => {
            const stage = stageOf(row);
            return (
              <tr key={row.id}>
                <td>
                  <input
                    type="checkbox"
                    aria-label={en.bulk.tick(row.title)}
                    checked={ticked.includes(row.id)}
                    onChange={(event) => onTick(row.id, event.target.checked)}
                  />
                </td>
                <td>
                  <Link to="/posts/$id" params={{ id: row.id }}>
                    {row.title}
                  </Link>
                  <br />
                  <span className="muted">{`/${row.slug}/`}</span>
                </td>
                <td data-label={en.posts.author}>{row.author ?? en.posts.noAuthor}</td>
                <td data-label={en.posts.checks}>
                  <CheckPills lint={row.lint} />
                </td>
                <td data-label={en.posts.review}>
                  <span className={stage === "live" ? "pill ok" : "pill quiet"}>
                    {en.stage[stage]}
                  </span>
                </td>
                <td data-label={en.posts.index}>
                  <span className={row.index_status === "indexable" ? "pill ok" : "pill quiet"}>
                    {en.index[row.index_status]}
                  </span>
                </td>
                <td data-label={seoEditorWords.seo}>
                  <ScoreCell value={row.scores?.seo} />
                </td>
                <td data-label={seoEditorWords.readability}>
                  <ScoreCell value={row.scores?.readability} />
                </td>
                <td className="link-count" data-label={seoEditorWords.linksColumn}>
                  {seoEditorWords.linksInOut(row.links.inbound, row.links.outbound)}
                </td>
                <td data-label={en.posts.changed}>
                  <time dateTime={row.updated_at}>{day(row.updated_at)}</time>
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}
