import { useState } from "react";
import type { PrivacyProps } from "../routes/Privacy";
import { SectionForm, TextField } from "../seo/Fields";
import { cleaned } from "../seo/parts";
import { privacyWords, TEXT_LANGUAGES } from "../strings/privacyWords";
import type { PrivacyText } from "./parts";

type Language = (typeof TEXT_LANGUAGES)[number];
type Key = keyof typeof privacyWords.text.fields;

const KEYS: Key[] = ["title", "banner", "preferences", "analytics", "marketing", "external_media"];

// Every field but the heading holds a sentence, so it gets a box of two lines.
const LONG: Key[] = ["banner", "preferences", "analytics", "marketing", "external_media"];

/** The owner's banner and category words, one language at a time. */
export function Text({ site, onSave }: PrivacyProps) {
  const words = privacyWords.text;
  const languages = TEXT_LANGUAGES.filter((each) => site.languages.includes(each));
  const shown = languages.length === 0 ? TEXT_LANGUAGES : languages;
  const [language, setLanguage] = useState<Language>(shown[0] ?? "en");
  const [texts, setTexts] = useState<Record<string, PrivacyText>>({ ...site.privacy?.text });
  const now = texts[language] ?? {};
  const set = (key: Key) => (value: string) =>
    setTexts({ ...texts, [language]: { ...now, [key]: value } });

  // Every language is sent together, as the one `text` part of [privacy].
  const save = () => {
    const kept = Object.entries(texts).flatMap(([name, text]) => {
      const part = cleaned(text);
      return part === null ? [] : [[name, part] as const];
    });
    return onSave({ privacy: { text: kept.length === 0 ? null : Object.fromEntries(kept) } });
  };

  return (
    <SectionForm lede={words.lede} onSave={save}>
      <div className="field">
        <label htmlFor="privacy-language">{words.language}</label>
        <span className="hint" id="privacy-language-hint">
          {words.languageHint}
        </span>
        <select
          id="privacy-language"
          aria-describedby="privacy-language-hint"
          value={language}
          onChange={(event) => {
            const picked = TEXT_LANGUAGES.find((each) => each === event.target.value);
            if (picked !== undefined) {
              setLanguage(picked);
            }
          }}
        >
          {shown.map((each) => (
            <option key={each} value={each}>
              {words.languages[each]}
            </option>
          ))}
        </select>
      </div>
      {KEYS.map((key) => (
        <TextField
          key={`${language} ${key}`}
          id={`privacy-text-${key}`}
          label={words.fields[key]}
          hint={words.fieldHint}
          rows={LONG.includes(key) ? 2 : undefined}
          value={now[key] ?? ""}
          onChange={set(key)}
        />
      ))}
    </SectionForm>
  );
}
