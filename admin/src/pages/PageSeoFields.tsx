import { pagesWords } from "../strings/pagesWords";
import type { SeoFields } from "./edits";
import { PageField } from "./PageField";

type Props = {
  fields: SeoFields;
  set: (key: keyof SeoFields) => (value: string) => void;
};

/** A page's search and sharing settings, folded away under the text. */
export function PageSeoFields({ fields, set }: Props) {
  const words = pagesWords.seo;
  const robots = pagesWords.fields.robots;
  const text = (name: Exclude<keyof SeoFields, "robots">, required = false) => (
    <PageField
      name={name}
      form="edit"
      value={fields[name]}
      onChange={set(name)}
      required={required}
    />
  );
  return (
    <details>
      <summary>{words.summary}</summary>
      <p className="muted">{words.lede}</p>
      <div className="field">
        <label htmlFor="edit-page-robots">{robots.label}</label>
        <span className="hint" id="edit-page-robots-hint">
          {robots.hint}
        </span>
        <select
          id="edit-page-robots"
          aria-describedby="edit-page-robots-hint"
          value={fields.robots}
          onChange={(event) => set("robots")(event.target.value)}
        >
          {words.robots.map((option) => (
            <option key={option.value} value={option.value}>
              {option.label}
            </option>
          ))}
        </select>
      </div>
      {text("canonical")}
      <div className="pair">
        {text("imageSrc")}
        {text("imageAlt", fields.imageSrc.trim() !== "")}
        {text("facebookTitle")}
        {text("facebookDescription")}
        {text("xTitle")}
        {text("xDescription")}
      </div>
    </details>
  );
}
