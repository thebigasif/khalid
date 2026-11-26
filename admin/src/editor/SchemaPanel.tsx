import { useCallback, useEffect, useState } from "react";
import type { PostCheck, SuggestedSchema } from "../api/check";
import { checkSchema, type SchemaChecks } from "../api/schemaCheck";
import { en } from "../strings/en";
import { SchemaReports } from "./SchemaReports";
import { SettingField } from "./SettingField";
import { applySchemaChange, type SchemaChange } from "./schemaFix";
import type { SettingsDraft, TextSetting } from "./settings";
import { richResultsTest, schemaTypes } from "./snippet";

type Props = {
  /** The saved post's check, or null before the blog has checked it. */
  check: PostCheck | null;
  /** Whether the post is live, so Google's test can read its page. */
  live: boolean;
  settings: SettingsDraft;
  onSetting: (key: TextSetting, value: string) => void;
  onBlur: () => void;
};

/** The schema types the post's content suggests, each with its reason and how sure. */
export function SchemaSuggestions({
  suggestions,
  filled,
  onFill,
}: {
  suggestions: SuggestedSchema[];
  /** Whether the writer's own schema field already holds something. */
  filled: boolean;
  /** Puts a suggestion's draft in the schema field. */
  onFill: (draft: string) => void;
}) {
  const words = en.schema.reports;
  if (suggestions.length === 0) {
    return null;
  }
  return (
    <section aria-labelledby="schema-suggested">
      <h3 id="schema-suggested">{words.suggested}</h3>
      <p className="hint">{words.suggestedHint}</p>
      <ul>
        {suggestions.map((suggestion) => (
          <li key={suggestion.kind}>
            <strong>{suggestion.kind}</strong>: {suggestion.reason}{" "}
            <span className="hint">({words.sure(suggestion.confidence)})</span>
            {suggestion.draft == null ? null : (
              <p>
                <button type="button" onClick={() => onFill(suggestion.draft ?? "")}>
                  {filled ? words.replace(suggestion.kind) : words.fill(suggestion.kind)}
                </button>
              </p>
            )}
          </li>
        ))}
      </ul>
    </section>
  );
}

/** The schema the page carries, a test for it, and the writer's own JSON-LD. */
export function SchemaPanel({ check, live, settings, onSetting, onBlur }: Props) {
  const words = en.schema;
  const [reports, setReports] = useState<SchemaChecks | null>(null);
  const [note, setNote] = useState<string | null>(null);
  const recheck = useCallback(async (text: string) => {
    if (text.trim() === "") {
      setReports(null);
      setNote(null);
      return;
    }
    const found = await checkSchema(text);
    setReports(found);
    setNote(found === null ? en.schema.reports.failed : null);
  }, []);
  // The schema as saved is checked once when the panel opens; after that,
  // each time the writer leaves the field or makes a fix.
  // biome-ignore lint/correctness/useExhaustiveDependencies: only the first text is checked here.
  useEffect(() => {
    void recheck(settings.schema);
  }, [recheck]);
  const change = (wanted: SchemaChange) => {
    const fixed = applySchemaChange(settings.schema, wanted);
    if (fixed === null) {
      setNote(words.reports.stale);
      return;
    }
    onSetting("schema", fixed);
    void recheck(fixed);
  };
  return (
    <details>
      <summary>{words.title}</summary>
      <div>
        <p className="hint">{words.lede}</p>
        {check === null ? null : (
          <>
            <SchemaSuggestions
              suggestions={check.schema_suggestions ?? []}
              filled={settings.schema.trim() !== ""}
              onFill={(draft) => {
                onSetting("schema", draft);
                void recheck(draft).then(() => setNote(words.reports.filled));
              }}
            />
            <ul className="types" aria-label={words.types}>
              {schemaTypes(check.schema).map((kind) => (
                <li key={kind}>{kind}</li>
              ))}
            </ul>
            <details>
              <summary>{words.show}</summary>
              <div>
                <pre className="json">{JSON.stringify(check.schema, null, 2)}</pre>
              </div>
            </details>
            {live ? (
              <p>
                <a href={richResultsTest(check.snippet.url)}>{words.test}</a>
              </p>
            ) : null}
          </>
        )}
        <SettingField id="setting-schema" label={words.own} hint={words.ownHint}>
          <textarea
            id="setting-schema"
            className="source"
            aria-describedby="setting-schema-hint"
            value={settings.schema}
            onChange={(event) => onSetting("schema", event.target.value)}
            onBlur={() => {
              onBlur();
              void recheck(settings.schema);
            }}
            rows={6}
            spellCheck={false}
          />
        </SettingField>
        {note === null ? null : (
          <p className="hint" role="status">
            {note}
          </p>
        )}
        {reports === null ? null : <SchemaReports checks={reports} onChange={change} />}
      </div>
    </details>
  );
}
