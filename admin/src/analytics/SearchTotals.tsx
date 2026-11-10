import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { Link } from "@tanstack/react-router";
import { type Days, readSearchTotals, type SearchTotals as Totals } from "../api/analytics";
import { problemMessage } from "../api/posts";
import { isData } from "../api/redirects";
import { Problem } from "../components/Problem";
import { calendarDay, count } from "../strings/dates";
import { en } from "../strings/en";
import { searchTotalsWords } from "../strings/searchTotalsWords";

type Props = { days: Days };

const POSITION = new Intl.NumberFormat(en.locale, {
  minimumFractionDigits: 1,
  maximumFractionDigits: 1,
});

const SHARE = new Intl.NumberFormat(en.locale, {
  style: "percent",
  minimumFractionDigits: 1,
  maximumFractionDigits: 1,
});

/** Each number's row: its name and how to write it for one period. */
function rows(): [string, (numbers: Totals["now"]) => string][] {
  const words = searchTotalsWords;
  return [
    [words.clicks, (numbers) => count(Math.round(numbers.clicks))],
    [words.shown, (numbers) => count(Math.round(numbers.impressions))],
    [words.clicked, (numbers) => SHARE.format(numbers.ctr)],
    [
      words.position,
      (numbers) => (numbers.position == null ? words.notShown : POSITION.format(numbers.position)),
    ],
  ];
}

/** Google search's numbers for the period, against the period before, with what each means. */
function Numbers({ totals }: { totals: Totals }) {
  const words = searchTotalsWords;
  const now = words.period(calendarDay(totals.first), calendarDay(totals.last));
  const before = words.before(totals.days);
  return (
    <>
      <div className="scroll">
        <table aria-labelledby="search-title">
          <thead>
            <tr>
              <th scope="col">{words.measure}</th>
              <th scope="col">{now}</th>
              <th scope="col">{before}</th>
            </tr>
          </thead>
          <tbody>
            {rows().map(([name, write]) => (
              <tr key={name}>
                <th scope="row">{name}</th>
                <td data-label={now}>{write(totals.now)}</td>
                <td data-label={before}>{write(totals.before)}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      <dl className="terms">
        {words.terms.map(({ term, meaning }) => (
          <div key={term}>
            <dt>{term}</dt>
            <dd>{meaning}</dd>
          </div>
        ))}
      </dl>
      <p className="muted">{words.late(totals.days)}</p>
    </>
  );
}

/**
 * What Google search did for the blog in the days picked. Google is asked on its own,
 * so a slow Google or a blog that is not connected leaves the rest of the screen as it is.
 */
export function SearchTotals({ days }: Props) {
  const words = searchTotalsWords;
  const reply = useQuery({
    queryKey: ["search-totals", days],
    queryFn: () => readSearchTotals(days),
    placeholderData: keepPreviousData,
    // Google's numbers for a day settle once; asking again within minutes adds nothing.
    staleTime: 5 * 60 * 1000,
  }).data;

  let body = <p role="status">{words.loading}</p>;
  if (isData(reply)) {
    body = <Numbers totals={reply.data} />;
  } else if (reply?.status === 409) {
    body = (
      <p>
        {words.notConnected} <Link to="/search-console">{words.connect}</Link>
      </p>
    );
  } else if (reply !== undefined) {
    body = <Problem message={problemMessage(reply.problem, en.problems.unreachable)} />;
  }
  return (
    <section aria-labelledby="search-title">
      <h2 id="search-title">{words.title}</h2>
      <p className="muted">{words.lede}</p>
      {body}
    </section>
  );
}
