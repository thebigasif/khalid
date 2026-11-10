import { Link } from "@tanstack/react-router";
import { Fragment, useState } from "react";
import type { PostRow } from "../api/rows";
import { lightOf } from "../editor/LintPanel";
import { day } from "../strings/dates";
import { en } from "../strings/en";
import { quickEditWords } from "../strings/quickEditWords";
import { seoEditorWords } from "../strings/seoEditorWords";
import { CheckPills } from "./CheckPills";
import { QuickEdit } from "./QuickEdit";
import { stageOf } from "./stages";

type Props = {
  rows: PostRow[];
  /** The identities of the ticked posts. */
  ticked: string[];
  onTick: (id: string, on: boolean) => void;
  /** The authors of the posts on the page, which quick edit offers. */
  authors: string[];
  /** Called with what to tell once a quick edit is saved or found nothing to save. */
  onSaid: (said: string) => void;
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
export function PostTable({ rows, ticked, onTick, authors, onSaid }: Props) {
  const [open, setOpen] = useState<string | null>(null);
  const close = (said: string | null) => {
    setOpen(null);
    if (said !== null) {
      onSaid(said);
    }
  };
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
              <Fragment key={row.id}>
                <tr>
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
                    <br />
                    <button
                      type="button"
                      className="quiet-link"
                      aria-label={quickEditWords.open(row.title)}
                      aria-expanded={open === row.id}
                      onClick={() => setOpen(open === row.id ? null : row.id)}
                    >
                      {quickEditWords.openShort}
                    </button>
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
                {open === row.id ? (
                  <tr>
                    <td colSpan={10}>
                      <QuickEdit id={row.id} authors={authors} onClose={close} />
                    </td>
                  </tr>
                ) : null}
              </Fragment>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}
