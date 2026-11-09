import { useState } from "react";
import { requestKey } from "../api/posts";
import type { SectionProps } from "../routes/SeoSettings";
import { seoWords } from "../strings/seoWords";
import { Check, SectionForm, TextField, useTexts } from "./Fields";
import { cleaned, numberOf, partOf, textOf } from "./parts";

const LIMIT_BY_DEFAULT = 20;

/** What every feed carries, and whether changed posts are sent to IndexNow. */
export function Feeds({ site, onSave }: SectionProps) {
  const words = seoWords.feeds;
  const feeds = partOf(site.seo, "feeds");
  const indexnow = partOf(site.seo, "indexnow");
  const [excerpt, setExcerpt] = useState(textOf(feeds, "content") === "excerpt");
  const [sending, setSending] = useState(textOf(indexnow, "key") !== "");
  const { texts, set } = useTexts({
    limit: String(numberOf(feeds, "limit", LIMIT_BY_DEFAULT)),
    before: textOf(feeds, "before_item"),
    after: textOf(feeds, "after_item"),
    source: textOf(feeds, "source_line"),
    key: textOf(indexnow, "key"),
  });
  // A new key is made only when the switch first turns on, so an existing one is kept.
  const turnIndexNow = (on: boolean) => {
    setSending(on);
    if (on && texts.key.trim() === "") {
      set("key")(requestKey());
    }
  };

  const save = () => {
    const limit = Number(texts.limit);
    const counted = Number.isInteger(limit) && limit > 0 && limit !== LIMIT_BY_DEFAULT;
    const part = {
      ...feeds,
      content: excerpt ? "excerpt" : undefined,
      limit: counted ? limit : undefined,
      before_item: texts.before,
      after_item: texts.after,
      source_line: texts.source,
    };
    const key = texts.key.trim();
    const sent = sending && key !== "" ? { ...indexnow, key } : null;
    return onSave({ feeds: cleaned(part), indexnow: sent });
  };

  return (
    <SectionForm lede={words.lede} onSave={save}>
      <fieldset>
        <legend>{words.content}</legend>
        {[false, true].map((short) => (
          <div className="choice" key={String(short)}>
            <input
              id={`seo-feed-${short ? "excerpt" : "full"}`}
              type="radio"
              name="seo-feed-content"
              checked={excerpt === short}
              onChange={() => setExcerpt(short)}
            />
            <label htmlFor={`seo-feed-${short ? "excerpt" : "full"}`}>
              {short ? words.excerpt : words.full}
            </label>
          </div>
        ))}
      </fieldset>
      <TextField
        id="seo-feed-limit"
        type="number"
        label={words.limit}
        hint={words.limitHint}
        value={texts.limit}
        onChange={set("limit")}
      />
      <TextField
        id="seo-feed-source"
        label={words.source}
        hint={words.sourceHint}
        value={texts.source}
        onChange={set("source")}
      />
      <div className="pair">
        {(["before", "after"] as const).map((key) => (
          <TextField
            key={key}
            id={`seo-feed-${key}`}
            rows={2}
            label={words[key]}
            hint={words.aroundHint}
            value={texts[key]}
            onChange={set(key)}
          />
        ))}
      </div>
      <h3>{words.indexnow}</h3>
      <Check
        id="seo-indexnow"
        label={words.indexnowOn}
        hint={words.indexnowOnHint}
        checked={sending}
        onChange={turnIndexNow}
      />
      {sending ? (
        <>
          <TextField
            id="seo-indexnow-key"
            label={words.key}
            hint={words.keyHint}
            value={texts.key}
            onChange={set("key")}
          />
          <button type="button" className="btn" onClick={() => set("key")(requestKey())}>
            {words.makeKey}
          </button>
        </>
      ) : null}
    </SectionForm>
  );
}
