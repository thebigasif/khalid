import { useState } from "react";
import type { SectionProps } from "../routes/SeoSettings";
import { seoWords } from "../strings/seoWords";
import { Check, SectionForm } from "./Fields";
import { cleaned, flagOf, partOf, type SeoTable, textOf } from "./parts";

/** The names of the locations `[[seo.locations]]` lists, in the order of site.toml. */
function locationNames(seo: SeoTable | undefined): string[] {
  const listed = seo?.locations;
  if (!Array.isArray(listed)) {
    return [];
  }
  return listed
    .map((each) => (typeof each === "object" && each !== null ? textOf(each, "name") : ""))
    .filter((name) => name !== "");
}

/** What a site with locations gets, from `[seo.local]`. */
export function LocalBusiness({ site, onSave }: SectionProps) {
  const words = seoWords.local;
  const local = partOf(site.seo, "local");
  const names = locationNames(site.seo);
  const [page, setPage] = useState(flagOf(local, "page", true));
  const [kml, setKml] = useState(flagOf(local, "kml", true));

  // Only what differs from the default is written, so site.toml stays short.
  const save = () => {
    const part = { ...local, page: page ? undefined : false, kml: kml ? undefined : false };
    return onSave({ local: cleaned(part) });
  };

  return (
    <SectionForm lede={words.lede} onSave={save}>
      <p>{names.length === 0 ? words.none : words.listed(names)}</p>
      <Check
        id="seo-local-page"
        label={words.page}
        hint={words.pageHint}
        checked={page}
        onChange={setPage}
      />
      <Check
        id="seo-local-kml"
        label={words.kml}
        hint={words.kmlHint}
        checked={kml}
        onChange={setKml}
      />
    </SectionForm>
  );
}
