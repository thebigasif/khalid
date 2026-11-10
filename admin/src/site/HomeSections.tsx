import { useState } from "react";
import type { HomeSection } from "../api/site";
import { namesAList, SECTION_KINDS, homeSections as words } from "../strings/homeSections";

type Kind = HomeSection["kind"];

/** `list` with the items at `from` and `to` swapped. */
function swapped<T>(list: readonly T[], from: number, to: number): T[] {
  const next = [...list];
  const [moved] = next.splice(from, 1);
  if (moved !== undefined) {
    next.splice(to, 0, moved);
  }
  return next;
}

/** The section the "Add a section" fields describe, or null without a slug it needs. */
function made(kind: Kind, slug: string): HomeSection | null {
  if (!namesAList(kind)) {
    return { kind };
  }
  const trimmed = slug.trim();
  return trimmed === "" ? null : { kind, slug: trimmed };
}

/**
 * The home page's sections in Settings: the list top first, each with
 * buttons to move or remove it, a row to add one, and a button that saves
 * the whole list to `site.toml`.
 */
export function HomeSections({
  saved,
  onSave,
}: {
  saved: readonly HomeSection[];
  onSave: (sections: HomeSection[]) => void;
}) {
  const [sections, setSections] = useState<HomeSection[]>([...saved]);
  const [kind, setKind] = useState<Kind>("latest");
  const [slug, setSlug] = useState("");
  const [problem, setProblem] = useState<string | null>(null);

  const add = () => {
    const section = made(kind, slug);
    const label = section === null ? null : words.label(section);
    if (section === null) {
      setProblem(words.slugMissing);
    } else if (sections.some((other) => words.label(other) === label)) {
      setProblem(words.already);
    } else {
      setProblem(null);
      setSections([...sections, section]);
      setSlug("");
    }
  };

  return (
    <section aria-labelledby="home-sections-title">
      <h2 id="home-sections-title">{words.title}</h2>
      <p className="hint">{words.lede}</p>
      {sections.length === 0 ? (
        <p>{words.none}</p>
      ) : (
        <ol className="home-sections">
          {sections.map((section, at) => (
            <li key={words.label(section)}>
              <span className="what">
                <strong>{words.label(section)}</strong>
                <span className="hint">{words.shows[section.kind]}</span>
              </span>
              <span className="row">
                {at > 0 ? (
                  <button
                    className="btn"
                    type="button"
                    aria-label={words.upNamed(section)}
                    onClick={() => setSections(swapped(sections, at, at - 1))}
                  >
                    {words.up}
                  </button>
                ) : null}
                {at < sections.length - 1 ? (
                  <button
                    className="btn"
                    type="button"
                    aria-label={words.downNamed(section)}
                    onClick={() => setSections(swapped(sections, at, at + 1))}
                  >
                    {words.down}
                  </button>
                ) : null}
                <button
                  className="btn"
                  type="button"
                  aria-label={words.removeNamed(section)}
                  onClick={() => setSections(sections.filter((_, other) => other !== at))}
                >
                  {words.remove}
                </button>
              </span>
            </li>
          ))}
        </ol>
      )}
      <div className="home-section-add">
        <div className="field">
          <label htmlFor="home-section-kind">{words.kindLabel}</label>
          <select
            id="home-section-kind"
            value={kind}
            onChange={(event) => setKind(event.target.value as Kind)}
          >
            {SECTION_KINDS.map((choice) => (
              <option key={choice} value={choice}>
                {words.menuName(choice)}
              </option>
            ))}
          </select>
        </div>
        {namesAList(kind) ? (
          <div className="field">
            <label htmlFor="home-section-slug">{words.slugLabel(kind)}</label>
            <span className="hint" id="home-section-slug-hint">
              {words.slugHint(kind)}
            </span>
            <input
              id="home-section-slug"
              aria-describedby="home-section-slug-hint"
              value={slug}
              onChange={(event) => setSlug(event.target.value)}
            />
          </div>
        ) : null}
        {problem === null ? null : <p role="alert">{problem}</p>}
        <button type="button" className="btn" onClick={add}>
          {words.add}
        </button>
      </div>
      <button type="button" className="btn primary" onClick={() => onSave(sections)}>
        {words.save}
      </button>
    </section>
  );
}
