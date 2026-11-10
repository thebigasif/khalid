import { type FormEvent, useEffect, useRef, useState } from "react";
import { failureMessage, isData } from "../api/call";
import { deleteSnippet, editSnippet, type Snippet } from "../api/snippets";
import { Problem } from "../components/Problem";
import type { UserMessage } from "../strings/en";
import { languageName } from "../strings/languages";
import { snippetWords } from "../strings/snippetWords";

/** A snippet and what the person chose to do with it. */
export type SnippetPick = { snippet: Snippet; action: "edit" | "remove" };

type Props = {
  pick: SnippetPick;
  /** Called with what to tell the person once the change is saved. */
  onDone: (message: string) => Promise<void>;
  onBack: () => void;
};

/** Edits or deletes one snippet, after saying which posts the change reaches. */
export function SnippetChange({ pick, onDone, onBack }: Props) {
  const { snippet, action } = pick;
  const words = action === "edit" ? snippetWords.edit : snippetWords.remove;
  const heading = useRef<HTMLHeadingElement>(null);
  const [title, setTitle] = useState(snippet.title);
  const [markdown, setMarkdown] = useState(snippet.markdown);
  const [busy, setBusy] = useState(false);
  const [failed, setFailed] = useState<UserMessage | null>(null);

  // The change replaces the list, so the focus moves to what it asks.
  useEffect(() => {
    heading.current?.focus();
  }, []);

  const send = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setBusy(true);
    const name = title.trim();
    const reply =
      action === "edit"
        ? await editSnippet(snippet, { title: name, markdown })
        : await deleteSnippet(snippet);
    setBusy(false);
    if (!isData(reply)) {
      setFailed(failureMessage(reply));
      return;
    }
    await onDone(
      action === "edit" ? snippetWords.edit.done(name) : snippetWords.remove.done(snippet.title),
    );
  };

  return (
    <section className="focus" aria-labelledby="snippet-change-title">
      <h2 id="snippet-change-title" ref={heading} tabIndex={-1}>
        {words.title(snippet.title)}
      </h2>
      <p>
        {action === "edit"
          ? snippetWords.edit.lede(languageName(snippet.language))
          : snippetWords.remove.lede}
      </p>
      {failed === null ? null : <Problem message={failed} />}
      <form onSubmit={send}>
        {action === "edit" ? (
          <>
            <div className="field">
              <label htmlFor="snippet-title">{snippetWords.edit.name}</label>
              <span className="hint" id="snippet-title-hint">
                {snippetWords.edit.nameHint(snippet.slug)}
              </span>
              <input
                id="snippet-title"
                value={title}
                required
                aria-describedby="snippet-title-hint"
                onChange={(event) => setTitle(event.target.value)}
              />
            </div>
            <div className="field">
              <label htmlFor="snippet-blocks">{snippetWords.edit.blocks}</label>
              <span className="hint" id="snippet-blocks-hint">
                {snippetWords.edit.blocksHint}
              </span>
              <textarea
                id="snippet-blocks"
                rows={10}
                value={markdown}
                aria-describedby="snippet-blocks-hint"
                onChange={(event) => setMarkdown(event.target.value)}
              />
            </div>
          </>
        ) : null}
        <span className="row">
          <button type="submit" className="btn primary" disabled={busy}>
            {words.submit}
          </button>
          <button type="button" className="btn" onClick={onBack}>
            {snippetWords.back}
          </button>
        </span>
      </form>
    </section>
  );
}
