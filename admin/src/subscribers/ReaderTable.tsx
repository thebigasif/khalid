import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useState } from "react";
import { failureMessage, isData, type Reply } from "../api/call";
import {
  type AdminNewsletter,
  type AdminSubscriber,
  changeLabel,
  forgetReader,
  listReaders,
  READERS_PER_PAGE,
  type SubscriberStatus,
} from "../api/subscribers";
import { Problem } from "../components/Problem";
import type { UserMessage } from "../strings/en";
import { subscriberWords } from "../strings/subscriberWords";
import { ReaderRow } from "./ReaderRow";

type Props = { newsletters: AdminNewsletter[] };

type Which = SubscriberStatus | "all";

const WHICH: Which[] = ["all", "confirmed", "pending", "unsubscribed"];

/** The readers, newest first, 50 to a page, narrowed by where they stand, newsletter and address. */
export function ReaderTable({ newsletters }: Props) {
  const words = subscriberWords.readers;
  const queries = useQueryClient();
  const [which, setWhich] = useState<Which>("all");
  const [list, setList] = useState("");
  const [q, setQ] = useState("");
  const [page, setPage] = useState(1);
  const [done, setDone] = useState<string | null>(null);
  const [failed, setFailed] = useState<UserMessage | null>(null);
  const query = { status: which === "all" ? undefined : which, list, q, page };
  const listed = useQuery({ queryKey: ["readers", query], queryFn: () => listReaders(query) });

  // Says how a change went and reads the list again; true when it worked.
  const settle = async <T,>(answer: Reply<T>, said: string): Promise<boolean> => {
    if (!isData(answer)) {
      setDone(null);
      setFailed(failureMessage(answer));
      return false;
    }
    setFailed(null);
    setDone(said);
    await queries.invalidateQueries({ queryKey: ["readers"] });
    await queries.invalidateQueries({ queryKey: ["newsletters"] });
    return true;
  };
  const label = async (reader: AdminSubscriber, name: string, change: "add" | "remove") =>
    settle(await changeLabel(reader.id, name, change), words.labelled(reader.email));
  const forget = async (reader: AdminSubscriber) => {
    await settle(await forgetReader(reader.id), words.forgotten(reader.email));
  };

  const answer = listed.data;
  let table = <p role="status">{subscriberWords.loading}</p>;
  if (answer !== undefined && !isData(answer)) {
    table = <Problem message={failureMessage(answer)} />;
  } else if (isData(answer) && answer.data.readers.length === 0) {
    table = <p role="status">{words.none}</p>;
  } else if (isData(answer)) {
    const { readers, pagination } = answer.data;
    const from = (pagination.page - 1) * READERS_PER_PAGE + 1;
    table = (
      <>
        <div className="scroll">
          <table className="wide">
            <thead>
              <tr>
                <th scope="col">{words.address}</th>
                <th scope="col">{words.stands}</th>
                <th scope="col">{words.newsletters}</th>
                <th scope="col">{words.labels}</th>
                <th scope="col">{words.joined}</th>
                <th scope="col">{words.actions}</th>
              </tr>
            </thead>
            <tbody>
              {readers.map((reader) => (
                <ReaderRow key={reader.id} reader={reader} onLabel={label} onForget={forget} />
              ))}
            </tbody>
          </table>
        </div>
        <p className="flex flex-wrap items-center gap-3">
          <span className="muted">
            {words.shown(from, from + readers.length - 1, pagination.total)}
          </span>
          {pagination.prev ? (
            <button type="button" className="btn" onClick={() => setPage(page - 1)}>
              {words.previous}
            </button>
          ) : null}
          {pagination.next ? (
            <button type="button" className="btn" onClick={() => setPage(page + 1)}>
              {words.next}
            </button>
          ) : null}
        </p>
      </>
    );
  }
  return (
    <section aria-labelledby="readers-title">
      <h2 id="readers-title">{words.title}</h2>
      <fieldset className="switch spaced" aria-label={words.which}>
        {WHICH.map((each) => (
          <button
            key={each}
            type="button"
            aria-pressed={each === which}
            onClick={() => {
              setWhich(each);
              setPage(1);
            }}
          >
            {words.statuses[each]}
          </button>
        ))}
      </fieldset>
      <div className="pair">
        <div className="field">
          <label htmlFor="readers-q">{words.find}</label>
          <input
            id="readers-q"
            type="search"
            value={q}
            onChange={(event) => {
              setQ(event.target.value);
              setPage(1);
            }}
          />
        </div>
        <div className="field">
          <label htmlFor="readers-list">{words.list}</label>
          <select
            id="readers-list"
            value={list}
            onChange={(event) => {
              setList(event.target.value);
              setPage(1);
            }}
          >
            <option value="">{words.anyList}</option>
            {newsletters.map((each) => (
              <option key={each.name} value={each.name}>
                {each.name}
              </option>
            ))}
          </select>
        </div>
      </div>
      {failed === null ? null : <Problem message={failed} />}
      <p role="status">{done}</p>
      {table}
      <p className="muted">{words.forgetNote}</p>
    </section>
  );
}
