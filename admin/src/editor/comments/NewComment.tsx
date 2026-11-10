import { type FormEvent, useState } from "react";
import type { NewDraftComment } from "../../api/draftComments";
import { Problem } from "../../components/Problem";
import { draftCommentWords as words } from "../../strings/draftCommentWords";
import type { UserMessage } from "../../strings/en";
import type { Send } from "./ThreadItem";

type Props = { parts: string[]; onAdd: Send<NewDraftComment> };

/** Starts a thread about the whole post or one part of its text. */
export function NewComment({ parts, onAdd }: Props) {
  // "0" stands for the whole post; any other value is a part's number.
  const [about, setAbout] = useState("0");
  const [text, setText] = useState("");
  const [busy, setBusy] = useState(false);
  const [problem, setProblem] = useState<UserMessage | null>(null);
  const choices = parts.map((name, n) => ({
    value: String(n + 1),
    label: words.part(n + 1, name),
  }));
  const submit = async (event: FormEvent) => {
    event.preventDefault();
    const block = Number(about);
    setBusy(true);
    const found = await onAdd(block === 0 ? { text } : { text, block });
    setBusy(false);
    setProblem(found);
    if (found === null) {
      setText("");
    }
  };
  return (
    <form aria-labelledby="new-comment-title" onSubmit={(event) => void submit(event)}>
      <h3 id="new-comment-title">{words.newTitle}</h3>
      <label htmlFor="comment-about">{words.about}</label>
      <select id="comment-about" value={about} onChange={(event) => setAbout(event.target.value)}>
        <option value="0">{words.whole}</option>
        {choices.map((choice) => (
          <option key={choice.value} value={choice.value}>
            {choice.label}
          </option>
        ))}
      </select>
      <label htmlFor="comment-text">{words.comment}</label>
      <textarea
        id="comment-text"
        rows={3}
        maxLength={5000}
        aria-describedby="comment-hint"
        value={text}
        onChange={(event) => setText(event.target.value)}
      />
      <p className="hint" id="comment-hint">
        {words.commentHint}
      </p>
      <button type="submit" className="btn primary" disabled={busy || text.trim() === ""}>
        {words.add}
      </button>
      {problem === null ? null : <Problem message={problem} />}
    </form>
  );
}
