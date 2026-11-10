import type { Mover, SearchReport } from "../api/searchConsole";
import { calendarDay, count } from "../strings/dates";
import { searchConsoleWords } from "../strings/searchConsoleWords";
import { PageName } from "./PageName";

type Props = { report: SearchReport };

/** This week's clicks against last week's, and the pages that gained and lost the most. */
export function ThisWeek({ report }: Props) {
  const words = searchConsoleWords.week;
  const lists: [string, string, Mover[]][] = [
    ["gained-title", words.gained, report.winners],
    ["lost-title", words.lost, report.losers],
  ];
  return (
    <section aria-labelledby="week-title">
      <h2 id="week-title">{words.title}</h2>
      <p>
        <strong>{words.clicks(count(Math.round(report.week_clicks)))}</strong>{" "}
        {words.sentence(calendarDay(report.last), count(Math.round(report.last_week_clicks)))}
      </p>
      {lists
        .filter(([, , movers]) => movers.length > 0)
        .map(([id, title, movers]) => (
          <section key={id} aria-labelledby={id}>
            <h3 id={id}>{title}</h3>
            <ul>
              {movers.map((mover) => (
                <li key={mover.url}>
                  <PageName id={mover.id} title={mover.title} />
                  {words.change(count(Math.round(mover.now)), count(Math.round(mover.before)))}
                  {mover.go_deeper != null && (
                    <>
                      {" "}
                      <a href={mover.go_deeper}>{searchConsoleWords.goDeeper}</a>
                    </>
                  )}
                </li>
              ))}
            </ul>
          </section>
        ))}
    </section>
  );
}
