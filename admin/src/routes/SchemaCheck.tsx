import { type FormEvent, useState } from "react";
import { problemMessage } from "../api/posts";
import { type Checked, checkMarkup } from "../api/schemaCheck";
import { PageHeader } from "../components/PageHeader";
import { Problem } from "../components/Problem";
import { SchemaReports } from "../editor/SchemaReports";
import { SettingField } from "../editor/SettingField";
import { applySchemaChange, type SchemaChange } from "../editor/schemaFix";
import { seoToolWords } from "../strings/seoToolWords";

type Source = "address" | "code";

/** The schema of any page address, or of pasted code, in two reports. */
export function SchemaCheck() {
  const words = seoToolWords.schemaCheck;
  const [source, setSource] = useState<Source>("address");
  const [address, setAddress] = useState("");
  const [code, setCode] = useState("");
  const [busy, setBusy] = useState(false);
  const [checked, setChecked] = useState<Checked | null>(null);

  const run = async (text: string) => {
    setBusy(true);
    setChecked(
      await checkMarkup(source === "address" ? { url: address.trim() } : { json_ld: text }),
    );
    setBusy(false);
  };
  const submit = (event: FormEvent) => {
    event.preventDefault();
    void run(code);
  };
  const change = (wanted: SchemaChange) => {
    const fixed = applySchemaChange(code, wanted);
    if (fixed !== null) {
      setCode(fixed);
      void run(fixed);
    }
  };

  return (
    <>
      <PageHeader title={words.title} lede={words.lede} />
      <form onSubmit={submit}>
        <fieldset>
          <legend>{words.source}</legend>
          {(["address", "code"] as const).map((kind) => (
            <label key={kind}>
              <input
                type="radio"
                name="schema-source"
                checked={source === kind}
                onChange={() => {
                  setSource(kind);
                  setChecked(null);
                }}
              />{" "}
              {kind === "address" ? words.address : words.code}
            </label>
          ))}
        </fieldset>
        {source === "address" ? (
          <SettingField id="schema-address" label={words.addressLabel} hint={words.addressHint}>
            <input
              id="schema-address"
              type="url"
              aria-describedby="schema-address-hint"
              value={address}
              onChange={(event) => setAddress(event.target.value)}
              required
            />
          </SettingField>
        ) : (
          <SettingField id="schema-code" label={words.codeLabel} hint={words.codeHint}>
            <textarea
              id="schema-code"
              className="source"
              aria-describedby="schema-code-hint"
              value={code}
              onChange={(event) => setCode(event.target.value)}
              rows={12}
              spellCheck={false}
              required
            />
          </SettingField>
        )}
        <button type="submit" className="btn primary" disabled={busy}>
          {busy ? words.checking : words.check}
        </button>
      </form>
      {checked === null ? null : "problem" in checked ? (
        <Problem message={problemMessage(checked.problem, words.failed)} />
      ) : (
        <>
          <h2>{words.results}</h2>
          {checked.checks.types.length === 0 ? null : (
            <p role="status">{words.found(checked.checks.types.join(", "))}</p>
          )}
          <SchemaReports checks={checked.checks} onChange={source === "code" ? change : null} />
        </>
      )}
    </>
  );
}
