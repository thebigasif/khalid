import { useQueryClient } from "@tanstack/react-query";
import { type FormEvent, useState } from "react";
import { type AltChange, replaceAlt } from "../api/media";
import { problemMessage } from "../api/posts";
import { isData } from "../api/redirects";
import { Problem } from "../components/Problem";
import { Toast } from "../components/Toast";
import { en, type UserMessage } from "../strings/en";

/** The words a preview was made for, and what it would change. */
type Preview = { find: string; replace: string; changes: AltChange[] };

/**
 * The same words changed in the alt text of every file at once (plan 5.1).
 * The changes are shown first, and saved only when the person says so.
 */
export function AltReplace() {
  const words = en.media.library.replace;
  const queries = useQueryClient();
  const [find, setFind] = useState("");
  const [replace, setReplace] = useState("");
  const [preview, setPreview] = useState<Preview | null>(null);
  const [busy, setBusy] = useState<string | null>(null);
  const [done, setDone] = useState<string | null>(null);
  const [failed, setFailed] = useState<UserMessage | null>(null);

  const send = async (apply: boolean, asked: { find: string; replace: string }) => {
    setBusy(apply ? words.saving : words.checking);
    setDone(null);
    const reply = await replaceAlt(asked.find, asked.replace, apply);
    setBusy(null);
    if (!isData(reply)) {
      setFailed(problemMessage(reply.problem, en.problems.unreachable));
      return;
    }
    setFailed(null);
    if (apply) {
      setPreview(null);
      setDone(words.done(reply.data.length));
      await queries.invalidateQueries({ queryKey: ["media"] });
    } else {
      setPreview({ ...asked, changes: reply.data });
    }
  };

  const show = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    if (busy === null) {
      void send(false, { find, replace });
    }
  };

  const apply = (shown: Preview) => {
    if (busy === null) {
      void send(true, shown);
    }
  };

  // A preview is only good for the words it was made for.
  const typed = (change: () => void) => {
    change();
    setPreview(null);
  };

  return (
    <section aria-labelledby="alt-replace-title">
      <h2 id="alt-replace-title">{words.title}</h2>
      <p className="muted">{words.lede}</p>
      <form onSubmit={show}>
        <div className="field">
          <label htmlFor="alt-find">{words.find}</label>
          <input
            id="alt-find"
            required
            value={find}
            onChange={(event) => typed(() => setFind(event.target.value))}
          />
        </div>
        <div className="field">
          <label htmlFor="alt-replace">{words.replace}</label>
          <span className="hint" id="alt-replace-hint">
            {words.replaceHint}
          </span>
          <input
            id="alt-replace"
            aria-describedby="alt-replace-hint"
            value={replace}
            onChange={(event) => typed(() => setReplace(event.target.value))}
          />
        </div>
        <button type="submit" className="btn">
          {words.show}
        </button>
      </form>
      {failed === null ? null : <Problem message={failed} />}
      {preview === null ? null : <Changes preview={preview} onApply={() => apply(preview)} />}
      <Toast message={busy ?? done} />
    </section>
  );
}

type ChangesProps = { preview: Preview; onApply: () => void };

/** What a replace would change, what that means, and the button that saves it. */
function Changes({ preview, onApply }: ChangesProps) {
  const words = en.media.library.replace;
  const count = preview.changes.length;
  if (count === 0) {
    return <p role="status">{words.none}</p>;
  }
  return (
    <>
      <p role="status">{words.found(count)}</p>
      <div className="scroll">
        <table aria-label={words.table}>
          <thead>
            <tr>
              <th scope="col">{words.file}</th>
              <th scope="col">{words.before}</th>
              <th scope="col">{words.after}</th>
            </tr>
          </thead>
          <tbody>
            {preview.changes.map((change) => (
              <tr key={change.id}>
                <td data-label={words.file}>
                  <code>{change.name}</code>
                </td>
                <td data-label={words.before}>{change.before}</td>
                <td data-label={words.after}>
                  {change.after === "" ? (
                    <span className="muted">{words.empty}</span>
                  ) : (
                    change.after
                  )}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      <p className="muted">{words.consequence}</p>
      <button type="button" className="btn primary" onClick={onApply}>
        {words.apply(count)}
      </button>
    </>
  );
}
