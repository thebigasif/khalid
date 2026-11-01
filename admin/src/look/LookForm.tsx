import { type FormEvent, useState } from "react";
import type { SiteSettings } from "../api/site";
import type { Template } from "../api/templates";
import { en } from "../strings/en";
import { defaultOf, turns, type Values, valuesOf, wordsOf } from "./lookChange";
import { OptionField } from "./OptionField";

type Props = {
  templates: Template[];
  /** The template the form opens on: the one in use, or the first. */
  start: Template;
  site: SiteSettings;
  onSave: (chosen: Template, values: Values) => Promise<void>;
};

/** The words for what a template turns on or off. */
function feature(key: string): string {
  return en.look.reader[key] ?? wordsOf(key);
}

/** One list of what a template turns on or off, under its title. */
function Turns({ title, keys }: { title: string; keys: string[] }) {
  if (keys.length === 0) {
    return null;
  }
  return (
    <>
      <p className="turns-title">{title}</p>
      <ul className="turns">
        {keys.map((key) => (
          <li key={key}>{feature(key)}</li>
        ))}
      </ul>
    </>
  );
}

/** Every template the blog can use, one chosen, and the chosen one's options. */
export function LookForm({ templates, start, site, onSave }: Props) {
  const words = en.look;
  const inUse = site.template.name;
  const [chosen, setChosen] = useState(start);
  const [values, setValues] = useState(valuesOf(start, site.template.options));
  const [busy, setBusy] = useState(false);

  const choose = (template: Template) => {
    setChosen(template);
    setValues(valuesOf(template, template.name === inUse ? site.template.options : {}));
  };

  const submit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setBusy(true);
    await onSave(chosen, values);
    setBusy(false);
  };

  return (
    <form onSubmit={submit}>
      <fieldset>
        <legend>
          <h2>{words.choose}</h2>
        </legend>
        <ul className="looks">
          {templates.map((template) => {
            const { on, off } = turns(template);
            return (
              <li className="look" key={template.name}>
                <label className="look-head">
                  <input
                    type="radio"
                    name="look"
                    checked={template.name === chosen.name}
                    aria-describedby={`look-about-${template.name}`}
                    onChange={() => choose(template)}
                  />
                  <span>{wordsOf(template.name)}</span>
                  {template.name === inUse ? <span className="pill ok">{words.inUse}</span> : null}
                </label>
                <p id={`look-about-${template.name}`}>{template.summary}</p>
                <Turns title={words.turnsOn} keys={on} />
                <Turns title={words.turnsOff} keys={off} />
              </li>
            );
          })}
        </ul>
      </fieldset>
      <section aria-labelledby="look-options">
        <h2 id="look-options">{words.optionsFor(wordsOf(chosen.name))}</h2>
        <p className="hint">{chosen.options.length === 0 ? words.noOptions : words.optionsHint}</p>
        {chosen.options.map((option) => (
          <OptionField
            key={`${chosen.name}-${option.key}`}
            option={option}
            value={values[option.key] ?? defaultOf(option.takes)}
            onChange={(value) => setValues({ ...values, [option.key]: value })}
          />
        ))}
      </section>
      <button type="submit" className="btn primary" disabled={busy}>
        {words.save}
      </button>
    </form>
  );
}
