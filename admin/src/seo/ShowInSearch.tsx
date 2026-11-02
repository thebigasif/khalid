import { useState } from "react";
import type { SectionProps } from "../routes/SeoSettings";
import { seoWords } from "../strings/seoWords";
import { Check, SectionForm, TextField } from "./Fields";
import { cleaned, numberOf, partOf, textOf } from "./parts";

const ARCHIVES = ["categories", "tags", "authors", "dates", "search"] as const;
type Archive = (typeof ARCHIVES)[number];

// What `[seo.index]` gives each archive when site.toml says nothing.
const SHOWN_BY_DEFAULT: Record<Archive, boolean> = {
  categories: true,
  tags: true,
  authors: true,
  dates: false,
  search: false,
};
const THIN_BY_DEFAULT = 3;

/** Which archives search engines may list, thin archives, and the whole blog's switch. */
export function ShowInSearch({ site, onSave }: SectionProps) {
  const words = seoWords.index;
  const index = partOf(site.seo, "index");
  const shownOf = (archive: Archive) => {
    const written = textOf(index, archive);
    return written === "" ? SHOWN_BY_DEFAULT[archive] : written === "index";
  };
  const [shown, setShown] = useState(() => ({
    categories: shownOf("categories"),
    tags: shownOf("tags"),
    authors: shownOf("authors"),
    dates: shownOf("dates"),
    search: shownOf("search"),
  }));
  const fewest = numberOf(index, "thin_archives_below", THIN_BY_DEFAULT);
  const [thin, setThin] = useState(String(fewest));
  const [hidden, setHidden] = useState(textOf(index, "site") === "noindex");

  // Only what differs from the default is written, so site.toml stays short.
  const save = () => {
    const part = { ...index };
    for (const archive of ARCHIVES) {
      const on = shown[archive];
      part[archive] = on === SHOWN_BY_DEFAULT[archive] ? undefined : on ? "index" : "noindex";
    }
    const below = Number(thin);
    const counted = Number.isInteger(below) && below >= 0 && below !== THIN_BY_DEFAULT;
    part.thin_archives_below = counted ? below : undefined;
    part.site = hidden ? "noindex" : undefined;
    return onSave({ index: cleaned(part) });
  };

  return (
    <SectionForm lede={words.lede} onSave={save}>
      <fieldset>
        <legend>{words.archives}</legend>
        {ARCHIVES.map((archive) => (
          <Check
            key={archive}
            id={`seo-index-${archive}`}
            label={words.archive[archive]}
            checked={shown[archive]}
            onChange={(on) => setShown({ ...shown, [archive]: on })}
          />
        ))}
      </fieldset>
      <TextField
        id="seo-thin"
        type="number"
        label={words.thin}
        hint={words.thinHint}
        value={thin}
        onChange={setThin}
      />
      <div className="careful">
        <Check
          id="seo-site-hidden"
          label={words.site}
          hint={words.siteHint}
          checked={hidden}
          onChange={setHidden}
        />
      </div>
    </SectionForm>
  );
}
