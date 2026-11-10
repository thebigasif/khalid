import { useQueryClient } from "@tanstack/react-query";
import { type FormEvent, type ReactNode, useState } from "react";
import { failureMessage, isData } from "../api/call";
import { problemMessage } from "../api/posts";
import { isFailure, isPage, useRows } from "../api/rows";
import { type AdminNewsletter, type SendReadiness, sendPost } from "../api/subscribers";
import { Problem } from "../components/Problem";
import { en, type UserMessage } from "../strings/en";
import { subscriberWords } from "../strings/subscriberWords";

type Props = { newsletters: AdminNewsletter[]; sending: SendReadiness };

/** The labels typed in the box: each once, without spaces, in the order given. */
function labelsOf(typed: string): string[] {
  const labels = typed.split(",").map((label) => label.trim().toLowerCase());
  return [...new Set(labels.filter((label) => label !== ""))];
}

/** The screen's one focus: a published post sent to one newsletter, or a labelled part of it. */
export function SendPost({ newsletters, sending }: Props) {
  const words = subscriberWords.send;
  const queries = useQueryClient();
  // The list caps at 100, the most recently changed; titles are then sorted for the eye.
  const rows = useRows({ filter: "status:published", order: "updated_at desc", limit: 100 });
  const [post, setPost] = useState("");
  const [list, setList] = useState(newsletters[0]?.name ?? "main");
  const [typed, setTyped] = useState("");
  const [busy, setBusy] = useState(false);
  const [done, setDone] = useState<string | null>(null);
  const [failed, setFailed] = useState<UserMessage | null>(null);
  const posts = isPage(rows)
    ? [...rows.rows].sort((a, b) => a.title.localeCompare(b.title, en.locale))
    : [];
  const labels = labelsOf(typed);
  const readers = newsletters.find((each) => each.name === list)?.confirmed ?? 0;

  const submit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const picked = posts.find((each) => each.id === post);
    if (picked === undefined || busy) {
      return;
    }
    setBusy(true);
    setDone(null);
    const answer = await sendPost(list, picked.id, labels);
    setBusy(false);
    if (!isData(answer)) {
      setFailed(failureMessage(answer));
      return;
    }
    setFailed(null);
    setPost("");
    setDone(words.started(picked.title));
    await queries.invalidateQueries({ queryKey: ["newsletters"] });
  };

  let form: ReactNode = <p role="status">{subscriberWords.loading}</p>;
  if (sending !== "ready") {
    form = <Problem message={words.notReady[sending]} />;
  } else if (isFailure(rows)) {
    form = <Problem message={problemMessage(rows.problem, en.problems.unreachable)} />;
  } else if (isPage(rows) && posts.length === 0) {
    form = <p>{words.noPosts}</p>;
  } else if (isPage(rows)) {
    form = (
      <form onSubmit={submit}>
        <div className="field">
          <label htmlFor="send-post">{words.post}</label>
          <select
            id="send-post"
            value={post}
            required
            onChange={(event) => setPost(event.target.value)}
          >
            <option value="">{words.pick}</option>
            {posts.map((each) => (
              <option key={each.id} value={each.id}>
                {each.title}
              </option>
            ))}
          </select>
        </div>
        <div className="field">
          <label htmlFor="send-list">{words.list}</label>
          <select id="send-list" value={list} onChange={(event) => setList(event.target.value)}>
            {newsletters.map((each) => (
              <option key={each.name} value={each.name}>
                {words.listOption(each.name, each.confirmed)}
              </option>
            ))}
          </select>
        </div>
        <div className="field">
          <label htmlFor="send-labels">{words.labels}</label>
          <span className="hint" id="send-labels-hint">
            {words.labelsHint}
          </span>
          <input
            id="send-labels"
            value={typed}
            aria-describedby="send-labels-hint"
            onChange={(event) => setTyped(event.target.value)}
          />
        </div>
        <button type="submit" className="btn primary" disabled={busy}>
          {labels.length === 0 ? words.submit(readers) : words.submitLabelled}
        </button>
      </form>
    );
  }
  return (
    <section className="focus wide" aria-labelledby="send-title">
      <h2 id="send-title">{words.title}</h2>
      <p>{words.lede}</p>
      {failed === null ? null : <Problem message={failed} />}
      <p role="status">{busy ? words.sending : done}</p>
      {form}
    </section>
  );
}
