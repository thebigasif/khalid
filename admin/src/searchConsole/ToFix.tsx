import { Link } from "@tanstack/react-router";
import type { SearchReport } from "../api/searchConsole";
import { count } from "../strings/dates";
import { searchConsoleWords } from "../strings/searchConsoleWords";
import { PageName } from "./PageName";

type Props = { report: SearchReport };

/** A number of clicks or requests, whole and grouped. */
const whole = (value: number) => count(Math.round(value));

/** The posts whose clicks keep falling, and the pages Google sends readers to who cannot read them. */
export function ToFix({ report }: Props) {
  const words = searchConsoleWords;
  return (
    <>
      {report.decaying.length > 0 && (
        <section aria-labelledby="falling-title">
          <h2 id="falling-title">{words.falling.title}</h2>
          <p className="muted">{words.falling.why}</p>
          <ul>
            {report.decaying.map((page) => (
              <li key={page.url}>
                <PageName id={page.id} title={page.title} />
                {words.falling.months(page.clicks.map(whole))}{" "}
                <a href={page.go_deeper}>{words.goDeeper}</a>
              </li>
            ))}
          </ul>
        </section>
      )}
      {report.lost.length > 0 && (
        <section aria-labelledby="lost-pages-title">
          <h2 id="lost-pages-title">{words.lost.title}</h2>
          <p className="muted">{words.lost.why}</p>
          <ul>
            {report.lost.map((page) => (
              <li key={page.url}>
                {page.url}
                {words.lost.clicks(whole(page.clicks))}{" "}
                {page.not_found_hits == null ? (
                  words.lost.noindex
                ) : (
                  <>
                    {words.lost.missing(whole(page.not_found_hits))}{" "}
                    <Link to="/redirects">{words.lost.redirect}</Link> {words.lost.replaced}
                  </>
                )}
              </li>
            ))}
          </ul>
        </section>
      )}
    </>
  );
}
