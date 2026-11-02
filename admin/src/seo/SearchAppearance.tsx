import { type FormEvent, useState } from "react";
import { shownAddress } from "../editor/snippet";
import type { SectionProps } from "../routes/SeoSettings";
import { seoWords } from "../strings/seoWords";
import { cleaned, type Part, partOf, textOf } from "./parts";
import {
  DEFAULT_SEPARATOR,
  defaultTemplate,
  filled,
  PAGE_TYPES,
  type PageType,
  type Sample,
  SEPARATORS,
  VARIABLES,
} from "./titles";

type Field = "title" | "description";
type Template = Record<Field, string>;
// Each page type's two templates, by its name in `[seo.titles]`.
type Templates = Record<string, Template>;

const EMPTY: Template = { title: "", description: "" };

function templatesOf(titles: Part): Templates {
  const each = (type: PageType) => {
    const table = partOf(titles, type);
    return { title: textOf(table, "title"), description: textOf(table, "description") };
  };
  return Object.fromEntries(PAGE_TYPES.map((type) => [type, each(type)]));
}

type FieldProps = {
  type: PageType;
  field: Field;
  value: string;
  onChange: (value: string) => void;
};

/** One template, the default it falls back to, and buttons that add each variable. */
function TemplateField({ type, field, value, onChange }: FieldProps) {
  const words = seoWords.appearance;
  const id = `seo-${field}`;
  const label = field === "title" ? words.titleField : words.descriptionField;
  const add = (variable: string) => {
    const text = value.trimEnd();
    onChange(text === "" ? `{${variable}}` : `${text} {${variable}}`);
  };
  return (
    <div className="field">
      <label htmlFor={id}>{label}</label>
      <span className="hint" id={`${id}-hint`}>
        {words.defaultIs(defaultTemplate(type, field))}
      </span>
      <input
        id={id}
        aria-describedby={`${id}-hint`}
        value={value}
        onChange={(event) => onChange(event.target.value)}
      />
      <div className="vars" role="group" aria-label={words.addTo(label.toLowerCase())}>
        {VARIABLES.map((variable) => (
          <button key={variable} type="button" className="btn" onClick={() => add(variable)}>
            {seoWords.variables[variable]}
          </button>
        ))}
      </div>
    </div>
  );
}

/** Titles and descriptions per kind of page, with the separator and a live search result. */
export function SearchAppearance({ site, onSave }: SectionProps) {
  const words = seoWords.appearance;
  const titles = partOf(site.seo, "titles");
  const [separator, setSeparator] = useState(textOf(titles, "separator"));
  const [tagline, setTagline] = useState(textOf(titles, "tagline"));
  const [type, setType] = useState<PageType>("post");
  const [templates, setTemplates] = useState(() => templatesOf(titles));
  const [busy, setBusy] = useState(false);
  const shown = templates[type] ?? EMPTY;
  const set = (field: Field) => (value: string) =>
    setTemplates({ ...templates, [type]: { ...shown, [field]: value } });
  const pick = (value: string) => setType(PAGE_TYPES.find((each) => each === value) ?? "post");
  const mark = separator.trim() === "" ? DEFAULT_SEPARATOR : separator.trim();
  const listed = SEPARATORS.some((each) => each === mark);
  const marks: string[] = listed ? [...SEPARATORS] : [...SEPARATORS, mark];
  const sample: Sample = {
    title: seoWords.samples[type],
    site: site.name,
    sep: mark,
    tagline: tagline.trim(),
    excerpt: type === "home" ? "" : seoWords.sampleExcerpt,
    category: seoWords.samples.category,
    author: seoWords.samples.author,
  };
  const result = (field: Field) =>
    filled(shown[field] || defaultTemplate(type, field), sample, mark);

  const submit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setBusy(true);
    // Settings this form does not show stay as site.toml has them.
    await onSave({ titles: cleaned({ ...titles, separator, tagline, ...templates }) });
    setBusy(false);
  };

  return (
    <form onSubmit={submit}>
      <p className="hint">{words.lede}</p>
      <fieldset>
        <legend>{words.separator}</legend>
        <span className="hint">{words.separatorHint}</span>
        <div className="seps">
          {marks.map((each) => (
            <label key={each}>
              <input
                type="radio"
                name="separator"
                value={each}
                checked={each === mark}
                onChange={() => setSeparator(each)}
              />
              <span>{each}</span>
            </label>
          ))}
        </div>
      </fieldset>
      <div className="field">
        <label htmlFor="seo-tagline">{words.tagline}</label>
        <span className="hint" id="seo-tagline-hint">
          {words.taglineHint}
        </span>
        <input
          id="seo-tagline"
          aria-describedby="seo-tagline-hint"
          value={tagline}
          onChange={(event) => setTagline(event.target.value)}
        />
      </div>
      <div className="field">
        <label htmlFor="seo-type">{words.type}</label>
        <span className="hint" id="seo-type-hint">
          {words.typeHint}
        </span>
        <select
          id="seo-type"
          aria-describedby="seo-type-hint"
          value={type}
          onChange={(event) => pick(event.target.value)}
        >
          {PAGE_TYPES.map((each) => (
            <option key={each} value={each}>
              {seoWords.types[each]}
            </option>
          ))}
        </select>
      </div>
      <TemplateField type={type} field="title" value={shown.title} onChange={set("title")} />
      <TemplateField
        type={type}
        field="description"
        value={shown.description}
        onChange={set("description")}
      />
      <section className="serp" aria-label={words.preview}>
        <p className="url">{shownAddress(`${site.address}${site.mount}`)}</p>
        <p className="head">{result("title")}</p>
        <p className="desc">{result("description")}</p>
      </section>
      <button type="submit" className="btn primary" disabled={busy}>
        {seoWords.save}
      </button>
    </form>
  );
}
