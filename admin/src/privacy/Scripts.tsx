import { useState } from "react";
import { EmptyState } from "../components/EmptyState";
import type { PrivacyProps } from "../routes/Privacy";
import { SectionForm, TextField } from "../seo/Fields";
import { CATEGORIES, privacyWords } from "../strings/privacyWords";
import { type CodeEntry, categoryOf, given } from "./parts";

/** An entry as saved: the code as read, and no empty name or link. */
function saved(entry: CodeEntry): CodeEntry {
  const { vendor, policy, ...rest } = entry;
  const named = given(vendor ?? "");
  const linked = given(policy ?? "");
  return {
    ...rest,
    ...(named === undefined ? {} : { vendor: named }),
    ...(linked === undefined ? {} : { policy: linked }),
  };
}

/** Each of the site's own scripts with the category it waits for, who runs it, and their policy. */
export function Scripts({ site, onSave }: PrivacyProps) {
  const words = privacyWords.scripts;
  const [entries, setEntries] = useState<CodeEntry[]>(site.code ?? []);
  const staticNote =
    site.mode === "static" ? (
      <p className="careful">
        <strong>{words.staticTitle}</strong> {words.staticWhy}
      </p>
    ) : null;
  if (entries.length === 0) {
    return (
      <>
        {staticNote}
        <EmptyState icon="braces" title={words.noScripts}>
          <p className="hint">{words.noScriptsWhy}</p>
          <p className="hint">{words.noScriptsFix}</p>
        </EmptyState>
      </>
    );
  }

  const change = (index: number, next: CodeEntry) =>
    setEntries(entries.map((entry, at) => (at === index ? { ...entry, ...next } : entry)));

  return (
    <>
      {staticNote}
      <SectionForm lede={words.lede} onSave={() => onSave({ code: entries.map(saved) })}>
        <ol className="scripts" aria-label={words.list}>
          {entries.map((entry, index) => {
            // The list is never reordered here, so its place names each script.
            const id = `privacy-script-${index}`;
            const category = categoryOf(entry);
            return (
              <li key={id} className="card" aria-label={words.script(index + 1)}>
                <pre className="json">
                  <code>{[entry.head, entry.body].filter(Boolean).join("\n")}</code>
                </pre>
                {category === null ? <p className="careful">{words.untagged}</p> : null}
                <div className="field">
                  <label htmlFor={`${id}-category`}>{words.category}</label>
                  <select
                    id={`${id}-category`}
                    value={category ?? "marketing"}
                    onChange={(event) => {
                      const picked = CATEGORIES.find((each) => each === event.target.value);
                      if (picked !== undefined) {
                        change(index, { category: picked });
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
                  onChange={(vendor) => change(index, { vendor })}
                />
                <TextField
                  id={`${id}-policy`}
                  type="url"
                  label={words.policy}
                  hint={words.policyHint}
                  value={entry.policy ?? ""}
                  onChange={(policy) => change(index, { policy })}
                />
              </li>
            );
          })}
        </ol>
      </SectionForm>
    </>
  );
}
