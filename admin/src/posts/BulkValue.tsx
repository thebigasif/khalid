import type { ReactNode } from "react";
import { type Category, categoryName } from "../api/categories";
import { en } from "../strings/en";
import type { BulkAction } from "./bulk";

/** The kinds of change that take a value, which this field asks for. */
export type ValueKind = Exclude<BulkAction["kind"], "trash">;

type Props = {
  kind: ValueKind;
  value: string;
  onChange: (value: string) => void;
  /** The authors of the posts on the page. */
  authors: string[];
  /** The tags the ticked posts have. */
  tags: string[];
  categories: Category[];
};

/** One labelled field with its hint, whose control is `children`. */
function Field({ label, hint, children }: { label: string; hint: string; children: ReactNode }) {
  return (
    <div className="field">
      <label htmlFor="bulk-value">{label}</label>
      <span className="hint" id="bulk-value-hint">
        {hint}
      </span>
      {children}
    </div>
  );
}

/** A select of `options`, each a value and the words it shows. */
function Choice({
  value,
  onChange,
  options,
}: Pick<Props, "value" | "onChange"> & { options: [string, string][] }) {
  return (
    <select
      id="bulk-value"
      aria-describedby="bulk-value-hint"
      value={value}
      onChange={(event) => onChange(event.target.value)}
    >
      {options.map(([option, shown]) => (
        <option key={option} value={option}>
          {shown}
        </option>
      ))}
    </select>
  );
}

/** The value a change to many posts needs: the author, the tag or the category. */
export function BulkValue({ kind, value, onChange, authors, tags, categories }: Props) {
  const words = en.bulk;
  const same = (names: string[]) => names.map((name): [string, string] => [name, name]);
  switch (kind) {
    case "author":
      return (
        <Field label={words.newAuthor} hint={words.newAuthorHint}>
          <Choice value={value} onChange={onChange} options={same(authors)} />
        </Field>
      );
    case "addTag":
      return (
        <Field label={words.newTag} hint={words.newTagHint}>
          <input
            id="bulk-value"
            aria-describedby="bulk-value-hint"
            required
            value={value}
            onChange={(event) => onChange(event.target.value)}
          />
        </Field>
      );
    case "removeTag":
      return (
        <Field label={words.oldTag} hint={words.oldTagHint}>
          <Choice value={value} onChange={onChange} options={same(tags)} />
        </Field>
      );
    case "category": {
      const named = categories.map((c): [string, string] => [c.slug, categoryName(c)]);
      const options: [string, string][] = [...named, ["", words.noCategory]];
      return (
        <Field label={words.newCategory} hint={words.newCategoryHint}>
          <Choice value={value} onChange={onChange} options={options} />
        </Field>
      );
    }
  }
}
