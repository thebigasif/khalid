import type { Template } from "../api/templates";
import { en } from "../strings/en";
import { type CtaLines, showsCta } from "./cta";

type Props = {
  /** The template chosen in the form, which decides where the call to action shows. */
  chosen: Template;
  lines: CtaLines;
  onChange: (lines: CtaLines) => void;
};

/** The longest line a label, an address or a note may be. */
const MOST = 160;

/** The site's call to action, `[template.cta]`: the button's words, where it goes, and a note. */
export function CtaFields({ chosen, lines, onChange }: Props) {
  const words = en.look.cta;
  // A label without an address, or the other way round, is no button.
  const started = lines.label.trim() !== "" || lines.href.trim() !== "" || lines.note.trim() !== "";
  const field = (key: keyof CtaLines, label: string, hint: string, needed: boolean) => (
    <div className="field">
      <label htmlFor={`look-cta-${key}`}>{label}</label>
      <span className="hint" id={`look-cta-${key}-hint`}>
        {hint}
      </span>
      <input
        id={`look-cta-${key}`}
        type="text"
        maxLength={MOST}
        required={needed}
        aria-describedby={`look-cta-${key}-hint`}
        value={lines[key]}
        onChange={(event) => onChange({ ...lines, [key]: event.target.value })}
      />
    </div>
  );
  return (
    <section aria-labelledby="look-cta">
      <h2 id="look-cta">{words.title}</h2>
      <p className="hint">{words.about}</p>
      <p className="hint">{showsCta(chosen) ? words.shows : words.blockOnly}</p>
      {field("label", words.label, words.labelHint, started)}
      {field("href", words.href, words.hrefHint, started)}
      {field("note", words.note, words.noteHint, false)}
    </section>
  );
}
