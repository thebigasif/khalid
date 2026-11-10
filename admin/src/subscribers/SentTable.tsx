import type { AdminSend } from "../api/subscribers";
import { day } from "../strings/dates";
import { subscriberWords } from "../strings/subscriberWords";

type Props = { sends: AdminSend[] };

/** The emails that went out, newest first, with their totals: never who got them. */
export function SentTable({ sends }: Props) {
  const words = subscriberWords.sent;
  let table = <p>{words.none}</p>;
  if (sends.length > 0) {
    table = (
      <div className="scroll">
        <table>
          <thead>
            <tr>
              <th scope="col">{words.post}</th>
              <th scope="col">{words.list}</th>
              <th scope="col">{words.when}</th>
              <th scope="col">{words.copies}</th>
              <th scope="col">{words.refused}</th>
              <th scope="col">{words.left}</th>
            </tr>
          </thead>
          <tbody>
            {sends.map((sent) => (
              <tr key={sent.key}>
                <th scope="row">{sent.key.replace(/^post:/, "")}</th>
                <td data-label={words.list}>{sent.list}</td>
                <td data-label={words.when}>{day(sent.started_at)}</td>
                <td data-label={words.copies}>{sent.sent.toLocaleString("en")}</td>
                <td data-label={words.refused}>{sent.failed.toLocaleString("en")}</td>
                <td data-label={words.left}>{sent.unsubscribed.toLocaleString("en")}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    );
  }
  return (
    <section aria-labelledby="sent-title">
      <h2 id="sent-title">{words.title}</h2>
      {table}
      <p className="muted">{words.totals}</p>
    </section>
  );
}
