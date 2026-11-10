import { type FormEvent, useEffect, useRef, useState } from "react";
import { failureMessage, isData } from "../api/call";
import { type AdminCategory, deleteCategory, renameCategory } from "../api/terms";
import { Problem } from "../components/Problem";
import type { UserMessage } from "../strings/en";
import { languageName } from "../strings/languages";
import { termWords } from "../strings/termWords";
import { categoryName, filledNames } from "./names";

/** A category, what the person chose to do with it, and the languages it is named in. */
export type CategoryPick = {
  category: AdminCategory;
  action: "rename" | "remove";
  /** The site's languages, its first language first. */
  languages: string[];
};

type Props = {
  pick: CategoryPick;
  /** Called with what to tell the person once the change is saved. */
  onDone: (message: string) => Promise<void>;
  onBack: () => void;
};

/** Renames or deletes one category, after saying what changes for its posts. */
export function CategoryChange({ pick, onDone, onBack }: Props) {
  const { category, action } = pick;
  const [first = "en"] = pick.languages;
  const shown = categoryName(category, first);
  const codes = [...new Set([...pick.languages, ...Object.keys(category.names)])];
  const heading = useRef<HTMLHeadingElement>(null);
  const [names, setNames] = useState<Record<string, string>>(category.names);
  const [busy, setBusy] = useState(false);
  const [failed, setFailed] = useState<UserMessage | null>(null);

  // The change replaces the list, so the focus moves to what it asks.
  useEffect(() => {
    heading.current?.focus();
  }, []);

  const send = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setBusy(true);
    const filled = filledNames(names);
    const reply =
      action === "rename" ? await renameCategory(category, filled) : await deleteCategory(category);
    setBusy(false);
    if (!isData(reply)) {
      setFailed(failureMessage(reply));
      return;
    }
    await onDone(
      action === "rename"
        ? termWords.renameCategory.done(filled[first] ?? Object.values(filled)[0] ?? shown)
        : termWords.deleteCategory.done(shown),
    );
  };

  const words = action === "rename" ? termWords.renameCategory : termWords.deleteCategory;
  return (
    <section className="focus" aria-labelledby="category-change-title">
      <h2 id="category-change-title" ref={heading} tabIndex={-1}>
        {words.title(shown)}
      </h2>
      <p>
        {action === "rename"
          ? termWords.renameCategory.lede(category.posts, category.slug)
          : termWords.deleteCategory.lede}
      </p>
      {failed === null ? null : <Problem message={failed} />}
      <form onSubmit={send}>
        {action === "rename" ? (
          <>
            <p className="hint">{termWords.renameCategory.hint}</p>
            <div className="pair">
              {codes.map((code, index) => (
                <div className="field" key={code}>
                  <label htmlFor={`rename-${code}`}>
                    {termWords.addCategory.name(languageName(code))}
                  </label>
                  <input
                    id={`rename-${code}`}
                    value={names[code] ?? ""}
                    required={index === 0}
                    onChange={(event) => setNames({ ...names, [code]: event.target.value })}
                  />
                </div>
              ))}
            </div>
          </>
        ) : null}
        <span className="row">
          <button type="submit" className="btn primary" disabled={busy}>
            {words.submit}
          </button>
          <button type="button" className="btn" onClick={onBack}>
            {termWords.back}
          </button>
        </span>
      </form>
    </section>
  );
}
