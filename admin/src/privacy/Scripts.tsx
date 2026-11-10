import { useState } from "react";
import { EmptyState } from "../components/EmptyState";
import type { PrivacyProps } from "../routes/Privacy";
import { SectionForm, TextField } from "../seo/Fields";
import { CATEGORIES, privacyWords } from "../strings/privacyWords";
import { type CodeEntry, categoryOf, given } from "./parts";

/** One script on the screen, with a key that stays the same when one above it is removed. */
type Row = { key: number; entry: CodeEntry };

/** `code` as written, or `undefined` when blank. Code is kept as typed, never trimmed. */
function code(text: string | undefined): string | undefined {
  return given(text ?? "") === undefined ? undefined : text;
}

/** An entry as saved: no empty code, name or link. */
function saved(entry: CodeEntry): CodeEntry {
  const head = code(entry.head);
  const body = code(entry.body);
  const vendor = given(entry.vendor ?? "");
  const policy = given(entry.policy ?? "");
  return {
    ...(head === undefined ? {} : { head }),
    ...(body === undefined ? {} : { body }),
    ...(entry.category === undefined ? {} : { category: entry.category }),
    ...(vendor === undefined ? {} : { vendor }),
    ...(policy === undefined ? {} : { policy }),
  };
}

/** Whether `entry` holds code at all; one with none is left out when saved. */
function hasCode(entry: CodeEntry): boolean {
  return code(entry.head) !== undefined || code(entry.body) !== undefined;
}

/** Each of the site's own scripts: its code, the category it waits for, who runs it, and their policy. */
export function Scripts({ site, onSave }: PrivacyProps) {
  const words = privacyWords.scripts;
  const [rows, setRows] = useState<Row[]>(() =>
    (site.code ?? []).map((entry, key) => ({ key, entry })),
  );
  const [next, setNext] = useState(rows.length);
  // A new script waits for Marketing, the strictest, until the owner picks.
  const add = () => {
    setRows([...rows, { key: next, entry: { category: "marketing" } }]);
    setNext(next + 1);
  };
  const addButton = (
    <button type="button" className="btn" onClick={add}>
      {words.add}
    </button>
  );
  const staticNote =
    site.mode === "static" ? (
      <p className="careful">
        <strong>{words.staticTitle}</strong> {words.staticWhy}
      </p>
    ) : null;
  if (rows.length === 0 && (site.code ?? []).length === 0) {
    return (
      <>
        {staticNote}
        <EmptyState icon="braces" title={words.noScripts}>
          <p className="hint">{words.noScriptsWhy}</p>
          <p className="hint">{words.noScriptsFix}</p>
          {addButton}
        </EmptyState>
      </>
    );
  }

  const change = (key: number, edit: CodeEntry) => {
    const edited = (row: Row): Row =>
      row.key === key ? { key, entry: { ...row.entry, ...edit } } : row;
    setRows(rows.map(edited));
  };
  const remove = (key: number) => setRows(rows.filter((row) => row.key !== key));
  const entries = rows.map((row) => row.entry).filter(hasCode);

  return (
    <>
      {staticNote}
      <SectionForm lede={words.lede} onSave={() => onSave({ code: entries.map(saved) })}>
        {rows.length === 0 ? <p className="hint">{words.noneLeft}</p> : null}
        <ol className="scripts" aria-label={words.list}>
          {rows.map(({ key, entry }, index) => {
            const id = `privacy-script-${key}`;
            const category = categoryOf(entry);
            return (
              <li key={id} className="card" aria-label={words.script(index + 1)}>
                {category === null ? <p className="careful">{words.untagged}</p> : null}
                <TextField
                  id={`${id}-head`}
                  label={words.head}
                  hint={words.headHint}
                  value={entry.head ?? ""}
                  onChange={(head) => change(key, { head })}
                  rows={4}
                />
                <TextField
                  id={`${id}-body`}
                  label={words.body}
                  hint={words.bodyHint}
                  value={entry.body ?? ""}
                  onChange={(body) => change(key, { body })}
                  rows={4}
                />
                <div className="field">
                  <label htmlFor={`${id}-category`}>{words.category}</label>
                  <select
                    id={`${id}-category`}
                    value={category ?? "marketing"}
                    onChange={(event) => {
                      const picked = CATEGORIES.find((each) => each === event.target.value);
                      if (picked !== undefined) {
                        change(key, { category: picked });
                      }
                    }}
                  >
                    {CATEGORIES.map((each) => (
                      <option key={each} value={each}>
                        {privacyWords.categories[each]}
                      </option>
                    ))}
                  </select>
                </div>
                <TextField
                  id={`${id}-vendor`}
                  label={words.vendor}
                  hint={words.vendorHint}
                  value={entry.vendor ?? ""}
                  onChange={(vendor) => change(key, { vendor })}
                />
                <TextField
                  id={`${id}-policy`}
                  type="url"
                  label={words.policy}
                  hint={words.policyHint}
                  value={entry.policy ?? ""}
                  onChange={(policy) => change(key, { policy })}
                />
                <button type="button" className="btn" onClick={() => remove(key)}>
                  {words.remove(index + 1)}
                </button>
              </li>
            );
          })}
        </ol>
        <p>{addButton}</p>
      </SectionForm>
    </>
  );
}
