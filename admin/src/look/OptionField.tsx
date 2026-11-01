import type { TemplateOption, TemplateValue } from "../api/templates";
import { en } from "../strings/en";
import { wordsOf } from "./lookChange";

type Props = {
  option: TemplateOption;
  value: TemplateValue;
  onChange: (value: TemplateValue) => void;
};

/** One option of a template, with the engine's sentence of help, as the field its kind needs. */
export function OptionField({ option, value, onChange }: Props) {
  const id = `look-${option.key}`;
  const label = wordsOf(option.key);
  const hint = (
    <span className="hint" id={`${id}-hint`}>
      {option.help}
    </span>
  );
  const takes = option.takes;
  if (takes.kind === "switch") {
    return (
      <div className="field">
        <label className="choice">
          <input
            type="checkbox"
            checked={value === true}
            aria-describedby={`${id}-hint`}
            onChange={(event) => onChange(event.target.checked)}
          />
          <span>{label}</span>
        </label>
        {hint}
      </div>
    );
  }
  if (takes.kind === "picks") {
    const picked = Array.isArray(value) ? value : [];
    const toggle = (word: string, on: boolean) =>
      onChange(on ? [...picked, word] : picked.filter((each) => each !== word));
    return (
      <fieldset className="field" aria-describedby={`${id}-hint`}>
        <legend>{label}</legend>
        {hint}
        {takes.allowed.map((word) => (
          <label className="choice" key={word}>
            <input
              type="checkbox"
              checked={picked.includes(word)}
              onChange={(event) => toggle(word, event.target.checked)}
            />
            <span>{word}</span>
          </label>
        ))}
      </fieldset>
    );
  }
  let input = (
    <select
      id={id}
      aria-describedby={`${id}-hint`}
      value={String(value)}
      onChange={(event) => onChange(event.target.value)}
    >
      {(takes.kind === "choice" ? takes.allowed : []).map((word) => (
        <option key={word}>{word}</option>
      ))}
    </select>
  );
  if (takes.kind === "count") {
    input = (
      <input
        id={id}
        type="number"
        required
        min={takes.least}
        max={takes.most}
        aria-describedby={`${id}-hint`}
        value={String(value)}
        onChange={(event) => {
          const typed = event.target.value;
          onChange(typed === "" ? "" : Number(typed));
        }}
      />
    );
  } else if (takes.kind === "lines") {
    input = (
      <textarea
        id={id}
        rows={Math.min(takes.most, 4)}
        aria-describedby={`${id}-hint`}
        value={Array.isArray(value) ? value.join("\n") : ""}
        onChange={(event) => onChange(event.target.value.split("\n"))}
      />
    );
  }
  return (
    <div className="field">
      <label htmlFor={id}>{label}</label>
      {hint}
      {takes.kind === "lines" ? (
        <span className="hint">{en.look.linesHint(takes.most)}</span>
      ) : null}
      {input}
    </div>
  );
}
