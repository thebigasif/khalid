import { type ChangeEvent, type FormEvent, type ReactNode, useState } from "react";
import { seoWords } from "../strings/seoWords";

type TextProps = {
  id: string;
  label: string;
  hint: string;
  value: string;
  onChange: (value: string) => void;
  type?: "text" | "url" | "email" | "tel" | "date" | "number";
  /** Lines to show, for a field that takes one entry per line. */
  rows?: number;
};

/** One labelled field with the hint that says what it does. */
export function TextField({ id, label, hint, value, onChange, type = "text", rows }: TextProps) {
  const bind = {
    id,
    value,
    "aria-describedby": `${id}-hint`,
    onChange: (event: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) =>
      onChange(event.target.value),
  };
  return (
    <div className="field">
      <label htmlFor={id}>{label}</label>
      <span className="hint" id={`${id}-hint`}>
        {hint}
      </span>
      {rows === undefined ? <input {...bind} type={type} /> : <textarea {...bind} rows={rows} />}
    </div>
  );
}

type CheckProps = {
  id: string;
  label: string;
  hint?: string;
  checked: boolean;
  onChange: (on: boolean) => void;
  /** Shown but not changeable, when another setting decides it. */
  disabled?: boolean;
};

/** One switch, with its hint under the words when it needs one. */
export function Check({ id, label, hint, checked, onChange, disabled }: CheckProps) {
  return (
    <div className="choice">
      <input
        id={id}
        type="checkbox"
        checked={checked}
        disabled={disabled}
        aria-describedby={hint === undefined ? undefined : `${id}-hint`}
        onChange={(event) => onChange(event.target.checked)}
      />
      <div>
        <label htmlFor={id}>{label}</label>
        {hint === undefined ? null : (
          <span className="hint" id={`${id}-hint`}>
            {hint}
          </span>
        )}
      </div>
    </div>
  );
}

type FormProps = { lede: string; onSave: () => Promise<void>; children: ReactNode };

/** One part's form: what it is for, its fields, and its Save button. */
export function SectionForm({ lede, onSave, children }: FormProps) {
  const [busy, setBusy] = useState(false);
  const submit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setBusy(true);
    await onSave();
    setBusy(false);
  };
  return (
    <form onSubmit={submit}>
      <p className="hint">{lede}</p>
      {children}
      <button type="submit" className="btn primary" disabled={busy}>
        {seoWords.save}
      </button>
    </form>
  );
}

/** Text fields kept by name, and a setter for one of them. */
export function useTexts<K extends string>(start: Record<K, string>) {
  const [texts, setTexts] = useState(start);
  const set = (key: K) => (value: string) => setTexts({ ...texts, [key]: value });
  return { texts, set };
}
