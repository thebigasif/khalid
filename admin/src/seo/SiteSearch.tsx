import { useState } from "react";
import type { SectionProps } from "../routes/SeoSettings";
import { seoWords } from "../strings/seoWords";
import { Check, SectionForm, TextField } from "./Fields";
import { cleaned, flagOf, numberOf, partOf } from "./parts";

const LENGTH_BY_DEFAULT = 50;

/** Which searches the blog refuses before looking anything up, from `[seo.search_filter]`. */
export function SiteSearch({ site, onSave }: SectionProps) {
  const words = seoWords.search;
  const filter = partOf(site.seo, "search_filter");
  const [length, setLength] = useState(String(numberOf(filter, "max_length", LENGTH_BY_DEFAULT)));
  const [symbols, setSymbols] = useState(flagOf(filter, "symbols", true));
  const [spam, setSpam] = useState(flagOf(filter, "spam", true));

  // Only what differs from the default is written, so site.toml stays short.
  const save = () => {
    const longest = Number(length);
    const counted = Number.isInteger(longest) && longest >= 0 && longest !== LENGTH_BY_DEFAULT;
    const part = {
      ...filter,
      max_length: counted ? longest : undefined,
      symbols: symbols ? undefined : false,
      spam: spam ? undefined : false,
    };
    return onSave({ search_filter: cleaned(part) });
  };

  return (
    <SectionForm lede={words.lede} onSave={save}>
      <TextField
        id="seo-search-length"
        type="number"
        label={words.length}
        hint={words.lengthHint}
        value={length}
        onChange={setLength}
      />
      <Check
        id="seo-search-symbols"
        label={words.symbols}
        hint={words.symbolsHint}
        checked={symbols}
        onChange={setSymbols}
      />
      <Check
        id="seo-search-spam"
        label={words.spam}
        hint={words.spamHint}
        checked={spam}
        onChange={setSpam}
      />
    </SectionForm>
  );
}
