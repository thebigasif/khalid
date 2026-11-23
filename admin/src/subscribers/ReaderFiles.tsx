import { useQueryClient } from "@tanstack/react-query";
import { type FormEvent, useState } from "react";
import { failureMessage, isData } from "../api/call";
import {
  type AdminNewsletter,
  exportReaders,
  importReaders,
  type ReaderImport,
} from "../api/subscribers";
import { Problem } from "../components/Problem";
import { save } from "../redirects/CsvRedirects";
import type { UserMessage } from "../strings/en";
import { subscriberWords } from "../strings/subscriberWords";

type Props = { newsletters: AdminNewsletter[] };

/** Every reader out as a CSV file, and a reader file in: counted first, added only on a yes. */
export function ReaderFiles({ newsletters }: Props) {
  const words = subscriberWords.files;
  const queries = useQueryClient();
  const [text, setText] = useState<string | null>(null);
  const [list, setList] = useState(newsletters[0]?.name ?? "main");
  const [counted, setCounted] = useState<ReaderImport | null>(null);
  const [busy, setBusy] = useState<string | null>(null);
  const [done, setDone] = useState<string | null>(null);
  const [failed, setFailed] = useState<UserMessage | null>(null);

  const download = async () => {
    const reply = await exportReaders();
    if (isData(reply)) {
      setFailed(null);
      save(reply.data, "subscribers.csv");
    } else {
      setFailed(failureMessage(reply));
    }
  };

  // Counts what the file would add, so the number shows before anything is saved.
  const check = async (csv: string | null, into: string) => {
    setCounted(null);
    setDone(null);
    setFailed(null);
    if (csv === null) {
      return;
    }
    setBusy(words.checking);
    const reply = await importReaders(csv, into, false);
    setBusy(null);
    if (isData(reply)) {
      setCounted(reply.data);
    } else {
      setFailed(failureMessage(reply));
    }
  };

  const add = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    if (text === null || counted === null || busy !== null) {
      return;
    }
    setBusy(words.adding);
    const reply = await importReaders(text, list, true);
    setBusy(null);
    if (!isData(reply)) {
      setFailed(failureMessage(reply));
      return;
    }
    setCounted(null);
    setDone(words.added(reply.data.added));
    await queries.invalidateQueries({ queryKey: ["readers"] });
    await queries.invalidateQueries({ queryKey: ["newsletters"] });
  };

  return (
    <section aria-labelledby="files-title">
      <h2 id="files-title">{words.title}</h2>
      <p className="muted">{words.lede}</p>
      <p>
        <button type="button" className="btn" onClick={download}>
          {words.download}
        </button>
      </p>
      <form onSubmit={add}>
        <div className="field">
          <label htmlFor="readers-file">{words.file}</label>
          <span className="hint" id="readers-file-hint">
            {words.fileHint}
          </span>
          <input
            id="readers-file"
            type="file"
            accept=".csv,text/csv"
            aria-describedby="readers-file-hint"
            onChange={async (event) => {
              const csv = (await event.target.files?.[0]?.text()) ?? null;
              setText(csv);
              await check(csv, list);
            }}
          />
        </div>
        {newsletters.length > 1 ? (
          <div className="field">
            <label htmlFor="readers-into">{words.into}</label>
            <select
              id="readers-into"
              value={list}
              onChange={async (event) => {
                setList(event.target.value);
                await check(text, event.target.value);
              }}
            >
              {newsletters.map((each) => (
                <option key={each.name} value={each.name}>
                  {each.name}
                </option>
              ))}
            </select>
          </div>
        ) : null}
        {failed === null ? null : <Problem message={failed} />}
        <p role="status">{busy ?? done}</p>
        {counted === null ? null : <Counted counted={counted} />}
      </form>
    </section>
  );
}

/** What the checked file would do, and the yes that adds its readers. */
function Counted({ counted }: { counted: ReaderImport }) {
  const words = subscriberWords.files;
  const { source, added, already, skipped } = counted;
  return (
    <div className="focus">
      <p>
        <strong>{words.counted(words.sources[source], added)}</strong>{" "}
        {already > 0 ? `${words.already(already)} ` : null}
        {added > 0 ? words.notYet : words.nothingNew}
      </p>
      {skipped.length > 0 ? (
        <details>
          <summary>{words.skipped(skipped.length)}</summary>
          <ul>
            {skipped.map((row) => (
              <li key={row.line}>{words.skippedRow(row.line, row.why)}</li>
            ))}
          </ul>
        </details>
      ) : null}
      {added > 0 ? (
        <button type="submit" className="btn primary">
          {words.add(added)}
        </button>
      ) : null}
    </div>
  );
}
