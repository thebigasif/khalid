import { type FormEvent, useState } from "react";
import type { AdminSubscriber } from "../api/subscribers";
import { day } from "../strings/dates";
import { subscriberWords } from "../strings/subscriberWords";

type Props = {
  reader: AdminSubscriber;
  onLabel: (reader: AdminSubscriber, label: string, change: "add" | "remove") => Promise<boolean>;
  onForget: (reader: AdminSubscriber) => void;
};

const PILLS = { confirmed: "pill ok", pending: "pill warn", unsubscribed: "pill quiet" } as const;

/** One reader: where they stand, what they get, their labels, and forgetting them on request. */
export function ReaderRow({ reader, onLabel, onForget }: Props) {
  const words = subscriberWords.readers;
  const [label, setLabel] = useState("");
  const [asking, setAsking] = useState(false);
  const field = `label-${reader.id}`;

  const add = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    if (await onLabel(reader, label.trim().toLowerCase(), "add")) {
      setLabel("");
    }
  };

  let forget = (
    <button
      type="button"
      className="btn"
      aria-label={words.forgetName(reader.email)}
      onClick={() => setAsking(true)}
    >
      {words.forget}
    </button>
  );
  if (asking) {
    forget = (
      <div className="flex flex-wrap items-center gap-3">
        <span>{words.forgetSure(reader.email)}</span>
        <button type="button" className="btn" onClick={() => onForget(reader)}>
          {words.forgetYes}
        </button>
        <button type="button" className="btn" onClick={() => setAsking(false)}>
          {words.keep}
        </button>
      </div>
    );
  }
  return (
    <tr>
      <th scope="row">{reader.email}</th>
      <td data-label={words.stands}>
        <span className={PILLS[reader.status]}>{words.statuses[reader.status]}</span>
      </td>
      <td data-label={words.newsletters}>{reader.lists.join(", ")}</td>
      <td data-label={words.labels}>
        <ul className="m-0 flex list-none flex-wrap gap-2 p-0">
          {reader.labels.map((each) => (
            <li key={each} className="flex items-center gap-1">
              <span className="pill quiet">{each}</span>
              <button
                type="button"
                className="btn"
                aria-label={words.removeLabelName(each, reader.email)}
                onClick={() => onLabel(reader, each, "remove")}
              >
                {words.removeLabel}
              </button>
            </li>
          ))}
        </ul>
        {reader.status === "unsubscribed" ? null : (
          <details>
            <summary>{words.addLabel}</summary>
            <form className="flex flex-wrap items-end gap-3" onSubmit={add}>
              <div className="field">
                <label htmlFor={field}>{words.label}</label>
                <span className="hint">{words.labelHint}</span>
                <input
                  id={field}
                  value={label}
                  required
                  pattern="[a-z0-9]+(-[a-z0-9]+)*"
                  onChange={(event) => setLabel(event.target.value)}
                />
              </div>
              <button type="submit" className="btn">
                {words.add}
              </button>
            </form>
          </details>
        )}
      </td>
      <td data-label={words.joined}>{day(reader.created_at)}</td>
      <td data-label={words.actions}>{forget}</td>
    </tr>
  );
}
