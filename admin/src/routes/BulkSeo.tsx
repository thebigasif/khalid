import { useState } from "react";
import { problemMessage } from "../api/posts";
import { isFailure, isPage, useRows } from "../api/rows";
import { PageHeader } from "../components/PageHeader";
import { Problem } from "../components/Problem";
import { BulkSeoRow } from "../seo/BulkSeoRow";
import { MISSING, type Missing } from "../seo/bulkSeo";
import { en } from "../strings/en";
import { seoToolWords } from "../strings/seoToolWords";

/** How many posts one page of the editor holds. */
const PAGE_SIZE = 20;

/** The SEO title, description and focus keyword of many posts, edited in one table. */
export function BulkSeo() {
  const words = seoToolWords.bulk;
  const [missing, setMissing] = useState<Missing | null>(null);
  const [page, setPage] = useState(1);
  // The list can filter by a focus keyword itself; the other two are read from each post.
  const filter = missing === "keyword" ? "has_focus_keyword:false" : undefined;
  const rows = useRows({ filter, order: "updated_at desc", limit: PAGE_SIZE, page });

  let shown = <p role="status">{words.loading}</p>;
  if (isFailure(rows)) {
    shown = <Problem message={problemMessage(rows.problem, en.problems.unreachable)} />;
  } else if (isPage(rows) && rows.rows.length === 0) {
    shown = <p role="status">{words.none}</p>;
  } else if (isPage(rows)) {
    const { pages } = rows.pagination;
    shown = (
      <>
        <div className="scroll">
          <table className="wide bulk-seo">
            <thead>
              <tr>
                <th scope="col">{words.post}</th>
                <th scope="col">{words.seoTitle}</th>
                <th scope="col">{words.description}</th>
                <th scope="col">{words.keyword}</th>
                <th scope="col">{words.actions}</th>
              </tr>
            </thead>
            <tbody>
              {rows.rows.map((row) => (
                <BulkSeoRow key={row.id} row={row} missing={missing} />
              ))}
            </tbody>
          </table>
        </div>
        {pages > 1 ? (
          <nav className="paging" aria-label={words.page(page, pages)}>
            <button
              type="button"
              className="btn"
              disabled={page <= 1}
              onClick={() => setPage(page - 1)}
            >
              {words.previous}
            </button>
            <span>{words.page(page, pages)}</span>
            <button
              type="button"
              className="btn"
              disabled={page >= pages}
              onClick={() => setPage(page + 1)}
            >
              {words.next}
            </button>
          </nav>
        ) : null}
      </>
    );
  }

  return (
    <>
      <PageHeader title={words.title} lede={words.lede} />
      <dl className="terms">
        {words.terms.map(({ term, meaning }) => (
          <div key={term}>
            <dt>{term}</dt>
            <dd>{meaning}</dd>
          </div>
        ))}
      </dl>
      <div className="field">
        <label htmlFor="bulk-seo-show">{words.show}</label>
        <select
          id="bulk-seo-show"
          value={missing ?? ""}
          onChange={(event) => {
            setMissing(MISSING.find((each) => each === event.target.value) ?? null);
            setPage(1);
          }}
        >
          <option value="">{words.every}</option>
          {MISSING.map((each) => (
            <option key={each} value={each}>
              {words.missing[each]}
            </option>
          ))}
        </select>
        {missing === null || missing === "keyword" ? null : (
          <span className="hint">{words.hidden}</span>
        )}
      </div>
      {shown}
    </>
  );
}
