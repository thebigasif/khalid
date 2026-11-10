import { useState } from "react";
import type { MenuLink, MenuName } from "../api/site";
import { navWords as words } from "../strings/navWords";
import { madeLink, swapped } from "./menuLinks";

/** A link's name in the list and on its buttons, with its language when it has one. */
function named(link: MenuLink): string {
  return link.language === undefined ? link.label : `${link.label} (${link.language})`;
}

/**
 * One menu on the Navigation screen: its links in order, each with buttons
 * to move or remove it, and fields to add one. The screen saves both menus.
 */
export function MenuEditor({
  menu,
  links,
  languages,
  setLinks,
}: {
  menu: MenuName;
  links: readonly MenuLink[];
  languages: readonly string[];
  setLinks: (links: MenuLink[]) => void;
}) {
  const card = words.menus[menu];
  const [label, setLabel] = useState("");
  const [href, setHref] = useState("");
  const [language, setLanguage] = useState("");
  const [problem, setProblem] = useState<string | null>(null);
  const id = (part: string) => `${menu}-${part}`;

  const add = () => {
    const made = madeLink(label, href, language);
    if ("problem" in made) {
      setProblem(made.problem);
      return;
    }
    setProblem(null);
    setLinks([...links, made.link]);
    setLabel("");
    setHref("");
  };

  return (
    <section aria-labelledby={id("title")}>
      <h2 id={id("title")}>{card.title}</h2>
      <p className="hint">{card.lede}</p>
      {links.length === 0 ? (
        <p>{card.none}</p>
      ) : (
        <ol className="home-sections">
          {links.map((link, at) => (
            <li key={`${named(link)} ${link.href}`}>
              <span className="what">
                <strong>{named(link)}</strong>
                <span className="hint">{link.href}</span>
              </span>
              <span className="row">
                {at > 0 ? (
                  <button
                    className="btn"
                    type="button"
                    aria-label={words.upNamed(named(link))}
                    onClick={() => setLinks(swapped(links, at, at - 1))}
                  >
                    {words.up}
                  </button>
                ) : null}
                {at < links.length - 1 ? (
                  <button
                    className="btn"
                    type="button"
                    aria-label={words.downNamed(named(link))}
                    onClick={() => setLinks(swapped(links, at, at + 1))}
                  >
                    {words.down}
                  </button>
                ) : null}
                <button
                  className="btn"
                  type="button"
                  aria-label={words.removeNamed(named(link))}
                  onClick={() => setLinks(links.filter((_, other) => other !== at))}
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
          <label htmlFor={id("label")}>{words.label.label}</label>
          <span className="hint" id={id("label-hint")}>
            {words.label.hint}
          </span>
          <input
            id={id("label")}
            aria-describedby={id("label-hint")}
            value={label}
            onChange={(event) => setLabel(event.target.value)}
          />
        </div>
        <div className="field">
          <label htmlFor={id("href")}>{words.href.label}</label>
          <span className="hint" id={id("href-hint")}>
            {words.href.hint}
          </span>
          <input
            id={id("href")}
            aria-describedby={id("href-hint")}
            value={href}
            onChange={(event) => setHref(event.target.value)}
          />
        </div>
        {languages.length > 1 ? (
          <div className="field">
            <label htmlFor={id("language")}>{words.language.label}</label>
            <select
              id={id("language")}
              value={language}
              onChange={(event) => setLanguage(event.target.value)}
            >
              <option value="">{words.language.every}</option>
              {languages.map((code) => (
                <option key={code} value={code}>
                  {words.language.only(code)}
                </option>
              ))}
            </select>
          </div>
        ) : null}
        {problem === null ? null : <p role="alert">{problem}</p>}
        <button type="button" className="btn" aria-label={words.addTo(menu)} onClick={add}>
          {words.add}
        </button>
      </div>
    </section>
  );
}
