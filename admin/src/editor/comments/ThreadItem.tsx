import { useState } from "react";
import type { DraftComment, NewDraftComment } from "../../api/draftComments";
import { Problem } from "../../components/Problem";
import { moment } from "../../strings/dates";
import { draftCommentWords as words } from "../../strings/draftCommentWords";
import type { UserMessage } from "../../strings/en";
import { aboutLine, type Thread } from "./threads";

/** Sends a change and gives back the problem, or `null` once it is saved. */
export type Send<T> = (change: T) => Promise<UserMessage | null>;

type Props = {
  thread: Thread;
  parts: string[];
  onAdd: Send<NewDraftComment>;
  onResolve: Send<{ first: number; resolved: boolean }>;
};

/** Who wrote one comment and when, then what it says. */
function Said({ comment }: { comment: DraftComment }) {
  return (
    <>
      <p className="hint">{words.byline(comment.author, moment(comment.created_at))}</p>
      <p className="whitespace-pre-wrap">{comment.text}</p>
    </>
  );
}

/** One thread: what it is about, its comments, a box to answer it, and a way to close it. */
export function ThreadItem({ thread, parts, onAdd, onResolve }: Props) {
  const { first, answers } = thread;
  const [text, setText] = useState("");
  const [busy, setBusy] = useState(false);
  const [problem, setProblem] = useState<UserMessage | null>(null);
  const run = async (send: () => Promise<UserMessage | null>) => {
    setBusy(true);
    const found = await send();
    setBusy(false);
    setProblem(found);
    return found;
  };
  const answer = async () => {
    if ((await run(() => onAdd({ text, parent: first.id }))) === null) {
      setText("");
    }
  };
  const resolve = (resolved: boolean) => void run(() => onResolve({ first: first.id, resolved }));
  const box = `answer-${first.id}`;
  return (
    <li className="card">
      <p className="hint">{aboutLine(first.block, parts)}</p>
      <Said comment={first} />
      {answers.length === 0 ? null : (
        <ol className="list-none pl-4" aria-label={words.answers}>
          {answers.map((said) => (
            <li key={said.id}>
              <Said comment={said} />
            </li>
          ))}
        </ol>
      )}
      {first.resolved ? (
        <button type="button" className="btn" disabled={busy} onClick={() => resolve(false)}>
          {words.reopen}
        </button>
      ) : (
        <>
          <label htmlFor={box}>{words.answer}</label>
          <textarea
            id={box}
            rows={2}
            value={text}
            onChange={(event) => setText(event.target.value)}
          />
          <div className="flex flex-wrap gap-3">
            <button
              type="button"
              className="btn"
              disabled={busy || text.trim() === ""}
              onClick={() => void answer()}
            >
              {words.sendAnswer}
            </button>
            <button type="button" className="btn" disabled={busy} onClick={() => resolve(true)}>
              {words.done}
            </button>
          </div>
          <p className="hint">{words.doneHint}</p>
        </>
      )}
      {problem === null ? null : <Problem message={problem} />}
    </li>
  );
}
