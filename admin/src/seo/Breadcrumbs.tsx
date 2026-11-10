import { useState } from "react";
import type { SectionProps } from "../routes/SeoSettings";
import { seoWords } from "../strings/seoWords";
import { Check, SectionForm, TextField, useTexts } from "./Fields";
import { cleaned, flagOf, partOf, textOf } from "./parts";

const SEPARATOR_BY_DEFAULT = "/";
const TAXONOMIES = ["category", "tag", "none"] as const;
type Taxonomy = (typeof TAXONOMIES)[number];

function taxonomyOf(written: string): Taxonomy {
  return TAXONOMIES.find((each) => each === written) ?? "category";
}

/** How the trail above each page reads, from `[seo.breadcrumbs]`. */
export function Breadcrumbs({ site, onSave }: SectionProps) {
  const words = seoWords.breadcrumbs;
  const crumbs = partOf(site.seo, "breadcrumbs");
  const [taxonomy, setTaxonomy] = useState(taxonomyOf(textOf(crumbs, "taxonomy")));
  const [bold, setBold] = useState(flagOf(crumbs, "bold_last", false));
  const { texts, set } = useTexts({
    separator: textOf(crumbs, "separator"),
    home: textOf(crumbs, "home"),
    archive: textOf(crumbs, "archive_prefix"),
    search: textOf(crumbs, "search_prefix"),
  });

  // Only what differs from the default is written, so site.toml stays short.
  const save = () => {
    const separator = texts.separator.trim();
    const part = {
      ...crumbs,
      separator: separator === SEPARATOR_BY_DEFAULT ? undefined : separator,
      home: texts.home,
      archive_prefix: texts.archive,
      search_prefix: texts.search,
      bold_last: bold ? true : undefined,
      taxonomy: taxonomy === "category" ? undefined : taxonomy,
    };
    return onSave({ breadcrumbs: cleaned(part) });
  };

  return (
    <SectionForm lede={words.lede} onSave={save}>
      <div className="pair">
        <TextField
          id="seo-crumb-separator"
          label={words.separator}
          hint={words.separatorHint}
          value={texts.separator}
          onChange={set("separator")}
        />
        <TextField
          id="seo-crumb-home"
          label={words.home}
          hint={words.homeHint}
          value={texts.home}
          onChange={set("home")}
        />
      </div>
      <TextField
        id="seo-crumb-archive"
        label={words.archivePrefix}
        hint={words.archivePrefixHint}
        value={texts.archive}
        onChange={set("archive")}
      />
      <TextField
        id="seo-crumb-search"
        label={words.searchPrefix}
        hint={words.searchPrefixHint}
        value={texts.search}
        onChange={set("search")}
      />
      <fieldset>
        <legend>{words.through}</legend>
        {TAXONOMIES.map((each) => (
          <div className="choice" key={each}>
            <input
              id={`seo-crumb-${each}`}
              type="radio"
              name="seo-crumb-taxonomy"
              checked={taxonomy === each}
              onChange={() => setTaxonomy(each)}
            />
            <label htmlFor={`seo-crumb-${each}`}>{words[each]}</label>
          </div>
        ))}
      </fieldset>
      <Check
        id="seo-crumb-bold"
        label={words.bold}
        hint={words.boldHint}
        checked={bold}
        onChange={setBold}
      />
    </SectionForm>
  );
}
