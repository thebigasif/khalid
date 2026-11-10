import { type FormEvent, useState } from "react";
import type { Design, FontChoice } from "../api/design";
import { type ColorToken, designWords } from "../strings/designWords";
import { ColorTable, colorOr, painted } from "./ColorTable";
import { faintCount, type Values, valuesOf } from "./designChange";
import { LogoPicker } from "./LogoPicker";

type Props = { design: Design; onSave: (values: Values) => Promise<void> };

/** One font choice: a list of every font, the one in use chosen. */
function FontField(props: {
  id: string;
  label: string;
  hint?: string;
  fonts: FontChoice[];
  value: string;
  onChange: (stack: string) => void;
}) {
  const { id, label, hint, fonts, value, onChange } = props;
  return (
    <div className="field">
      <label htmlFor={id}>{label}</label>
      {hint === undefined ? null : (
        <span className="hint" id={`${id}-hint`}>
          {hint}
        </span>
      )}
      <select
        id={id}
        value={value}
        aria-describedby={hint === undefined ? undefined : `${id}-hint`}
        onChange={(event) => onChange(event.target.value)}
      >
        {fonts.map((font) => (
          <option key={font.stack} value={font.stack}>
            {font.label}
          </option>
        ))}
      </select>
    </div>
  );
}

/** A post as the blog draws it with `tokens`, so a change shows before it is saved. */
function Preview({ tokens }: { tokens: Record<string, string> }) {
  const words = designWords;
  const paint = painted({
    "--preview-background": colorOr(tokens["colors.background"]),
    "--preview-text": colorOr(tokens["colors.text"]),
    "--preview-muted": colorOr(tokens["colors.muted"]),
    "--preview-link": colorOr(tokens["colors.link"]),
    "--preview-border": colorOr(tokens["colors.border"]),
    "--preview-body": tokens["fonts.body"] ?? "",
    "--preview-heading": tokens["fonts.heading"] ?? "",
  });
  return (
    <section aria-labelledby="preview-title">
      <h2 id="preview-title">{words.preview}</h2>
      <div className="design-preview" ref={paint}>
        <p className="preview-date">{words.previewDate}</p>
        <p className="preview-title">{words.previewTitle}</p>
        <p>{words.previewText}</p>
      </div>
    </section>
  );
}

/** The design's colours, fonts, shape and logo, changed in place and saved together. */
export function DesignForm({ design, onSave }: Props) {
  const words = designWords;
  const [values, setValues] = useState(valuesOf(design));
  const [busy, setBusy] = useState(false);
  const tokens = values.tokens;
  const set = (name: string, value: string) =>
    setValues({ ...values, tokens: { ...tokens, [name]: value } });
  const keep = (token: ColorToken, kept: boolean) => {
    const others = values.locked.filter((name) => name !== token);
    setValues({ ...values, locked: kept ? [...others, token] : others });
  };
  const faint = faintCount(tokens);

  const submit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setBusy(true);
    await onSave(values);
    setBusy(false);
  };

  return (
    <form className="design-form" onSubmit={submit}>
      <h2>{words.colors}</h2>
      <p className={faint === 0 ? "passed" : "careful"} role="status">
        <strong>{faint === 0 ? words.allReadable : words.someFaint(faint)}</strong> {words.standard}
      </p>
      <ColorTable values={values} saved={design} onColor={set} onKeep={keep} />
      <div className="design-grid">
        <div>
          <h2>{words.fonts}</h2>
          <FontField
            id="font-body"
            label={words.body}
            hint={words.bodyHint}
            fonts={design.fonts}
            value={tokens["fonts.body"] ?? ""}
            onChange={(stack) => set("fonts.body", stack)}
          />
          <FontField
            id="font-heading"
            label={words.heading}
            fonts={design.fonts}
            value={tokens["fonts.heading"] ?? ""}
            onChange={(stack) => set("fonts.heading", stack)}
          />
          <h2>{words.shape}</h2>
          {(["radius", "measure"] as const).map((key) => (
            <div className="field" key={key}>
              <label htmlFor={`shape-${key}`}>{words[key]}</label>
              <span className="hint" id={`shape-${key}-hint`}>
                {key === "radius" ? words.radiusHint : words.measureHint}
              </span>
              <input
                id={`shape-${key}`}
                value={tokens[`shape.${key}`] ?? ""}
                aria-describedby={`shape-${key}-hint`}
                onChange={(event) => set(`shape.${key}`, event.target.value)}
              />
            </div>
          ))}
          <LogoPicker
            value={tokens["brand.logo"] ?? ""}
            onChange={(logo) => set("brand.logo", logo)}
          />
        </div>
        <Preview tokens={tokens} />
      </div>
      <div className="actions">
        <button type="submit" className="btn primary" disabled={busy}>
          {words.save}
        </button>
        <p className="hint resync">{words.resync}</p>
      </div>
    </form>
  );
}
