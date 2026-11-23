import { type FormEvent, useState } from "react";
import type { SiteComments } from "../api/site";
import { en } from "../strings/en";

type Props = { saved: SiteComments; onSave: (comments: SiteComments) => Promise<void> };

/** `[comments]` in Settings: open, closed or off for every post, and the email to staff. */
export function CommentsSetting({ saved, onSave }: Props) {
  const words = en.site.comments;
  const [comments, setComments] = useState(saved);
  const [busy, setBusy] = useState(false);

  const submit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setBusy(true);
    await onSave(comments);
    setBusy(false);
  };

  return (
    <form className="max-w-160" onSubmit={submit}>
      <h2>{words.title}</h2>
      <p>{words.lede}</p>
      <fieldset>
        <legend className="sr-only">{words.title}</legend>
        {words.states.map((state) => (
          <div className="field" key={state.value}>
            <label>
              <input
                type="radio"
                name="comments-state"
                value={state.value}
                checked={comments.state === state.value}
                aria-describedby={`comments-${state.value}-hint`}
                onChange={() => setComments({ ...comments, state: state.value })}
              />{" "}
              {state.label}
            </label>
            <span className="hint" id={`comments-${state.value}-hint`}>
              {state.hint}
            </span>
          </div>
        ))}
      </fieldset>
      <div className="field">
        <label>
          <input
            type="checkbox"
            checked={comments.notify}
            onChange={(event) => setComments({ ...comments, notify: event.target.checked })}
          />{" "}
          {words.notify}
        </label>
      </div>
      <button type="submit" className="btn primary" disabled={busy}>
        {words.save}
      </button>
    </form>
  );
}
