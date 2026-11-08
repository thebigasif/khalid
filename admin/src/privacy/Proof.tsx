import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { useState } from "react";
import { PeriodForm } from "../analytics/PeriodForm";
import {
  type ConsentCounts,
  type Days,
  readConsentCounts,
  readConsentVersions,
} from "../api/analytics";
import { problemMessage } from "../api/posts";
import { isData } from "../api/redirects";
import { EmptyState } from "../components/EmptyState";
import { Problem } from "../components/Problem";
import { save } from "../redirects/CsvRedirects";
import type { PrivacyProps } from "../routes/Privacy";
import { calendarDay, count } from "../strings/dates";
import { en } from "../strings/en";
import { privacyWords } from "../strings/privacyWords";

type Tally = ConsentCounts["totals"];

/** The counts of a tally, in the order of the table's and the file's columns. */
const COLUMNS = ["choices", "preferences", "analytics", "marketing", "external_media"] as const;

/** The Monday that starts the week of `day`, both as the API writes days. */
function monday(day: string): string {
  const date = new Date(`${day}T00:00:00Z`);
  date.setUTCDate(date.getUTCDate() - ((date.getUTCDay() + 6) % 7));
  return date.toISOString().slice(0, 10);
}

/** The answers of each week, by the Monday that starts it, first week first. */
function weeks(counts: ConsentCounts): Map<string, Tally> {
  const weeks = new Map<string, Tally>();
  for (const { day, counts: tally } of counts.per_day) {
    const sum = weeks.get(monday(day));
    const added = COLUMNS.map((key) => [key, tally[key] + (sum?.[key] ?? 0)]);
    weeks.set(monday(day), Object.fromEntries(added) as Tally);
  }
  return weeks;
}

/** Every day's answers as CSV text, header first and the period's totals last. */
function csv({ per_day, totals }: ConsentCounts): string {
  const line = (first: string, version: string, tally: Tally) =>
    [first, version, ...COLUMNS.map((key) => tally[key])].join(",");
  const days = per_day.map((row) => line(row.day, row.version, row.counts));
  return [`day,version,${COLUMNS.join(",")}`, ...days, line("total", "", totals), ""].join("\n");
}

/** The heading of each column of counts. */
function label(key: (typeof COLUMNS)[number]): string {
  return key === "choices" ? privacyWords.proof.answers : privacyWords.text.fields[key];
}

/** One row of the weekly table: its heading, then a count per column. */
function Row({ name, tally }: { name: string; tally: Tally }) {
  return (
    <tr>
      <th scope="row">{name}</th>
      {COLUMNS.map((key) => (
        <td key={key} data-label={label(key)}>
          {count(tally[key])}
        </td>
      ))}
    </tr>
  );
}

/** What readers were shown and chose, from the consent proof Khalid keeps (R18). */
export function Proof({ site }: PrivacyProps) {
  const words = privacyWords.proof;
  const [days, setDays] = useState<Days>(28);
  const counts = useQuery({
    queryKey: ["consent-counts", days],
    queryFn: () => readConsentCounts(days),
    placeholderData: keepPreviousData,
  }).data;
  const versions = useQuery({ queryKey: ["consent-versions"], queryFn: readConsentVersions }).data;

  if (!isData(counts) || !isData(versions)) {
    const failed = [counts, versions].find((reply) => reply !== undefined && !("data" in reply));
    if (failed !== undefined && "status" in failed) {
      return <Problem message={problemMessage(failed.problem, en.problems.unreachable)} />;
    }
    return <p role="status">{words.loading}</p>;
  }
  const shown = versions.data.versions;
  return (
    <div className="proof">
      <p>{words.lede}</p>
      <h3 id="proof-counts">{words.counts}</h3>
      <p className="muted">{words.countsLede}</p>
      <PeriodForm days={days} onDays={setDays} />
      {counts.data.per_day.length === 0 ? (
        <EmptyState icon="table" title={words.none}>
          <p>{site.mode === "static" ? privacyWords.settings.countsStatic : words.noneWhy}</p>
        </EmptyState>
      ) : (
        <>
          <div className="scroll">
            <table aria-labelledby="proof-counts">
              <thead>
                <tr>
                  <th scope="col">{words.week}</th>
                  {COLUMNS.map((key) => (
                    <th key={key} scope="col">
                      {label(key)}
                    </th>
                  ))}
                </tr>
              </thead>
              <tbody>
                {[...weeks(counts.data)].map(([start, tally]) => (
                  <Row key={start} name={words.weekOf(calendarDay(start))} tally={tally} />
                ))}
              </tbody>
              <tfoot>
                <Row name={words.total} tally={counts.data.totals} />
              </tfoot>
            </table>
          </div>
          <button
            type="button"
            className="btn"
            onClick={() => save(csv(counts.data), "consent.csv")}
          >
            {words.download}
          </button>
        </>
      )}
      <h3 id="proof-history">{words.history}</h3>
      <p className="muted">{shown.length === 0 ? words.noHistory : words.historyLede}</p>
      <div className="scroll">
        <table aria-labelledby="proof-history">
          <thead>
            <tr>
              <th scope="col">{words.version}</th>
              <th scope="col">{words.from}</th>
              <th scope="col">{words.until}</th>
              <th scope="col">{words.shown}</th>
            </tr>
          </thead>
          <tbody>
            {shown.map((each) => (
              <tr key={each.version}>
                <th scope="row">
                  <code>{each.version}</code>
                </th>
                <td data-label={words.from}>{calendarDay(each.first_day)}</td>
                <td data-label={words.until}>{calendarDay(each.last_day)}</td>
                <td data-label={words.shown}>
                  <details>
                    <summary>{words.show}</summary>
                    <pre>{JSON.stringify(each.shown, null, 1)}</pre>
                  </details>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
