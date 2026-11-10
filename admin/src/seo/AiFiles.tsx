import { useState } from "react";
import type { SectionProps } from "../routes/SeoSettings";
import { seoWords } from "../strings/seoWords";
import { Check, SectionForm, TextField, useTexts } from "./Fields";
import { cleaned, flagOf, lines, listOf, partOf, textOf } from "./parts";

/** `llms.txt` and the whole-blog schema files, from `[seo.llms_txt]` and `[seo.schema_map]`. */
export function AiFiles({ site, onSave }: SectionProps) {
  const words = seoWords.machines;
  const llms = partOf(site.seo, "llms_txt");
  const map = partOf(site.seo, "schema_map");
  const [writesLlms, setWritesLlms] = useState(!flagOf(llms, "off", false));
  const [writesMap, setWritesMap] = useState(!flagOf(map, "off", false));
  const [robots, setRobots] = useState(flagOf(map, "robots_txt", false));
  const { texts, set } = useTexts({
    intro: textOf(llms, "intro"),
    pages: listOf(llms, "pages").join("\n"),
    exclude: listOf(llms, "exclude").join("\n"),
  });

  // Only what differs from the default is written, so site.toml stays short.
  const save = () => {
    const llmsPart = {
      ...llms,
      off: writesLlms ? undefined : true,
      intro: texts.intro,
      pages: lines(texts.pages),
      exclude: lines(texts.exclude),
    };
    const mapPart = {
      ...map,
      off: writesMap ? undefined : true,
      robots_txt: robots ? true : undefined,
    };
    return onSave({ llms_txt: cleaned(llmsPart), schema_map: cleaned(mapPart) });
  };

  return (
    <SectionForm lede={words.lede} onSave={save}>
      <h3>{words.llmsTxt}</h3>
      <Check
        id="seo-llms"
        label={words.llms}
        hint={words.llmsHint}
        checked={writesLlms}
        onChange={setWritesLlms}
      />
      <TextField
        id="seo-llms-intro"
        rows={3}
        label={words.intro}
        hint={words.introHint}
        value={texts.intro}
        onChange={set("intro")}
      />
      <TextField
        id="seo-llms-pages"
        rows={3}
        label={words.pages}
        hint={words.pagesHint}
        value={texts.pages}
        onChange={set("pages")}
      />
      <TextField
        id="seo-llms-exclude"
        rows={2}
        label={words.exclude}
        hint={words.excludeHint}
        value={texts.exclude}
        onChange={set("exclude")}
      />
      <h3>{words.schema}</h3>
      <Check
        id="seo-schema-map"
        label={words.files}
        hint={words.filesHint}
        checked={writesMap}
        onChange={setWritesMap}
      />
      <Check
        id="seo-schema-robots"
        label={words.robots}
        hint={words.robotsHint}
        checked={robots}
        onChange={setRobots}
      />
    </SectionForm>
  );
}
