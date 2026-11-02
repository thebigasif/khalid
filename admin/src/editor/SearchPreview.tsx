import { type ChangeEvent, useState } from "react";
import type { Snippet } from "../api/check";
import { calendarDay } from "../strings/dates";
import { en } from "../strings/en";
import { seoEditorWords } from "../strings/seoEditorWords";
import { type Device, fitted, SERP_LINES } from "./pixels";
import { SettingField } from "./SettingField";
import type { SettingsDraft, TextSetting } from "./settings";
import { boldParts, shownAddress } from "./snippet";

type Props = {
  /** The result the saved post makes, or null before the blog has checked it. */
  snippet: Snippet | null;
  settings: SettingsDraft;
  onSetting: (key: TextSetting, value: string) => void;
  onBlur: () => void;
};

const DEVICES: Device[] = ["desktop", "mobile"];

/** `text` with the focus keyword in bold, as Google marks the words searched for. */
function Marked({ text, keyword }: { text: string; keyword: string }) {
  return (
    <>
      {boldParts(text, keyword).map((piece) =>
        piece.bold ? <b key={piece.at}>{piece.text}</b> : <span key={piece.at}>{piece.text}</span>,
      )}
    </>
  );
}

/** The post as a Google result on desktop or mobile, with the title and keyword that shape it. */
export function SearchPreview({ snippet, settings, onSetting, onBlur }: Props) {
  const words = en.search;
  const [device, setDevice] = useState<Device>("desktop");
  const bind = (key: "seoTitle" | "focusKeyword") => ({
    id: `setting-${key}`,
    value: settings[key],
    "aria-describedby": `setting-${key}-hint`,
    onChange: (event: ChangeEvent<HTMLInputElement>) => onSetting(key, event.target.value),
    onBlur,
  });
  const lengths = snippet && words.lengths(snippet.title.length, snippet.description.length);
  const lines = SERP_LINES[device];
  const title = snippet && fitted(snippet.title, lines.title.width, lines.title.size);
  const description =
    snippet && fitted(snippet.description, lines.description.width, lines.description.size);
  const date = settings.date === "" ? null : calendarDay(settings.date);
  return (
    <details open>
      <summary>{words.title}</summary>
      <div>
        <p className="hint">{words.lede}</p>
        <fieldset className="switch spaced" aria-label={seoEditorWords.device}>
          {DEVICES.map((each) => (
            <button
              key={each}
              type="button"
              aria-pressed={device === each}
              onClick={() => setDevice(each)}
            >
              {seoEditorWords[each]}
            </button>
          ))}
        </fieldset>
        {snippet === null || title === null || description === null ? null : (
          <>
            <section className={`serp ${device}`} aria-label={words.result}>
              <p className="url">{shownAddress(snippet.url)}</p>
              <p className="head">
                <Marked text={title.shown} keyword={settings.focusKeyword} />
              </p>
              <p className="desc">
                {date === null ? null : <span className="muted">{`${date} · `}</span>}
                {snippet.description === "" ? (
                  words.none
                ) : (
                  <Marked text={description.shown} keyword={settings.focusKeyword} />
                )}
              </p>
            </section>
            <p className="hint">{lengths}</p>
            {title.cut ? <p className="hint">{seoEditorWords.titleCut}</p> : null}
            {description.cut ? <p className="hint">{seoEditorWords.descriptionCut}</p> : null}
          </>
        )}
        <SettingField id="setting-seoTitle" label={words.seoTitle} hint={words.seoTitleHint}>
          <input {...bind("seoTitle")} />
        </SettingField>
        <p className="hint">{words.description}</p>
        <SettingField id="setting-focusKeyword" label={words.focus} hint={words.focusHint}>
          <input {...bind("focusKeyword")} />
        </SettingField>
      </div>
    </details>
  );
}
