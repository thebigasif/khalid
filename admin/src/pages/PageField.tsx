import { pagesWords } from "../strings/pagesWords";

type Props = {
  /** Which of the page's fields this is. */
  name: keyof typeof pagesWords.fields;
  /** Tells the new-page form's fields from the edit form's. */
  form: "new" | "edit";
  value: string;
  onChange: (value: string) => void;
  required?: boolean;
};

/** One labelled field of a page, with its hint; the text is several lines. */
export function PageField({ name, form, value, onChange, required = false }: Props) {
  const words = pagesWords.fields[name];
  const id = `${form}-page-${name}`;
  const shared = {
    id,
    value,
    required,
    "aria-describedby": `${id}-hint`,
  };
  return (
    <div className={name === "markdown" ? "field text" : "field"}>
      <label htmlFor={id}>{words.label}</label>
      <span className="hint" id={`${id}-hint`}>
        {words.hint}
      </span>
      {name === "markdown" ? (
        <textarea rows={12} {...shared} onChange={(event) => onChange(event.target.value)} />
      ) : (
        <input {...shared} onChange={(event) => onChange(event.target.value)} />
      )}
    </div>
  );
}
