import type { AiBots as Report } from "../api/analytics";
import { count } from "../strings/dates";
import { en } from "../strings/en";

type Props = { report: Report };

/** How often each AI assistant's bots read the blog, beside the readers the assistant sent. */
export function AiBots({ report }: Props) {
  const words = en.analytics.bots;
  return (
    <section aria-labelledby="bots-title">
      <h2 id="bots-title">{words.title}</h2>
      <p className="muted">{words.lede}</p>
      {report.tools.length === 0 ? (
        <p>{words.none}</p>
      ) : (
        <div className="scroll">
          <table aria-labelledby="bots-title">
            <thead>
              <tr>
                <th scope="col">{en.analytics.tools.tool}</th>
                <th scope="col">{words.reads}</th>
                <th scope="col">{words.pages}</th>
                <th scope="col">{words.sent}</th>
                <th scope="col">{words.bots}</th>
              </tr>
            </thead>
            <tbody>
              {report.tools.map((row) => (
                <tr key={row.tool}>
                  <th scope="row">{row.name}</th>
                  <td data-label={words.reads}>{count(row.reads)}</td>
                  <td data-label={words.pages}>{count(row.pages)}</td>
                  <td data-label={words.sent}>{count(row.sent)}</td>
                  <td data-label={words.bots}>
                    {row.bots.map((bot) => `${bot.bot} ${count(bot.reads)}`).join(", ")}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </section>
  );
}
