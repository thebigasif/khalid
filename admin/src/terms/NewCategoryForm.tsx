import { type FormEvent, useState } from "react";
import type { NewCategory } from "../api/terms";
import { languageName } from "../strings/languages";
import { termWords } from "../strings/termWords";
import { filledNames } from "./names";

type Props = {
  /** The languages to ask a name in, the site's first language first. */
  languages: string[];
  /** Adds the category; true when it was added, so the form empties. */
  onAdd: (category: NewCategory) => Promise<boolean>;
};

/** The short name and the name in each language of a new category. */
export function NewCategoryForm({ languages, onAdd }: Props) {
  const words = termWords.addCategory;
  const [slug, setSlug] = useState("");
  const [names, setNames] = useState<Record<string, string>>({});
  const [busy, setBusy] = useState(false);

  const submit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setBusy(true);
    const added = await onAdd({ slug: slug.trim(), names: filledNames(names) });
    setBusy(false);
    if (added) {
      setSlug("");
      setNames({});
    }
  };

  return (
    <form onSubmit={submit}>
      <div className="field">
        <label htmlFor="category-slug">{words.slug}</label>
        <span className="hint" id="category-slug-hint">
          {words.slugHint}
        </span>
        <input
          id="category-slug"
          value={slug}
          required
          pattern="[a-z0-9]+(-[a-z0-9]+)*"
          aria-describedby="category-slug-hint"
          onChange={(event) => setSlug(event.target.value)}
        />
      </div>
      <div className="pair">
        {languages.map((code, index) => (
          <div className="field" key={code}>
            <label htmlFor={`category-name-${code}`}>{words.name(languageName(code))}</label>
            <input
              id={`category-name-${code}`}
              value={names[code] ?? ""}
              required={index === 0}
              onChange={(event) => setNames({ ...names, [code]: event.target.value })}
            />
          </div>
        ))}
      </div>
      <button type="submit" className="btn primary" disabled={busy}>
        {words.submit}
      </button>
    </form>
  );
}
