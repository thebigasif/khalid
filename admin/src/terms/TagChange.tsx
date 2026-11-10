import { type FormEvent, useEffect, useRef, useState } from "react";
import { failureMessage, isData } from "../api/call";
import { type AdminTag, removeTag, renameTag } from "../api/terms";
import { Problem } from "../components/Problem";
import type { UserMessage } from "../strings/en";
import { languageName } from "../strings/languages";
import { termWords } from "../strings/termWords";

/** A tag and what the person chose to do with it. */
export type TagPick = { tag: AdminTag; action: "rename" | "remove" };

type Props = {
  pick: TagPick;
  /** Called with what to tell the person once the change is saved. */
  onDone: (message: string) => Promise<void>;
  onBack: () => void;
};

/** Renames or removes one tag, after saying how many posts change. */
export function TagChange({ pick, onDone, onBack }: Props) {
  const { tag, action } = pick;
  const words = action === "rename" ? termWords.renameTag : termWords.removeTag;
  const heading = useRef<HTMLHeadingElement>(null);
  const [name, setName] = useState(tag.name);
  const [busy, setBusy] = useState(false);
  const [failed, setFailed] = useState<UserMessage | null>(null);

  // The change replaces the list, so the focus moves to what it asks.
  useEffect(() => {
    heading.current?.focus();
  }, []);

  const send = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setBusy(true);
    const wanted = name.trim();
    const reply = action === "rename" ? await renameTag(tag, wanted) : await removeTag(tag);
    setBusy(false);
    if (!isData(reply)) {
      setFailed(failureMessage(reply));
      return;
    }
    await onDone(
      action === "rename"
        ? termWords.renameTag.done(tag.name, reply.data?.[0]?.name ?? wanted)
        : termWords.removeTag.done(tag.name),
    );
  };

  return (
    <section className="focus" aria-labelledby="tag-change-title">
      <h2 id="tag-change-title" ref={heading} tabIndex={-1}>
        {words.title(tag.name)}
      </h2>
      <p>
        {action === "rename"
          ? termWords.renameTag.lede(tag.posts, languageName(tag.language))
          : termWords.removeTag.lede(tag.posts)}
      </p>
      {failed === null ? null : <Problem message={failed} />}
      <form onSubmit={send}>
        {action === "rename" ? (
          <div className="field">
            <label htmlFor="tag-name">{termWords.renameTag.label}</label>
            <span className="hint" id="tag-name-hint">
              {termWords.renameTag.hint}
            </span>
            <input
              id="tag-name"
              value={name}
              required
              aria-describedby="tag-name-hint"
              onChange={(event) => setName(event.target.value)}
            />
          </div>
        ) : null}
        <span className="row">
          <button type="submit" className="btn primary" disabled={busy}>
            {words.submit(tag.posts)}
          </button>
          <button type="button" className="btn" onClick={onBack}>
            {termWords.back}
          </button>
        </span>
      </form>
    </section>
  );
}
