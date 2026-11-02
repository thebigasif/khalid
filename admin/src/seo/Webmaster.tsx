import type { SectionProps } from "../routes/SeoSettings";
import { seoWords } from "../strings/seoWords";
import { SectionForm, TextField, useTexts } from "./Fields";
import { cleaned, partOf, textOf } from "./parts";

const ENGINES = ["google", "bing", "yandex", "baidu", "pinterest"] as const;

/** The codes that prove to each search engine's owner tools that the blog is yours. */
export function Webmaster({ site, onSave }: SectionProps) {
  const words = seoWords.webmaster;
  const verification = partOf(site.seo, "verification");
  const { texts, set } = useTexts({
    google: textOf(verification, "google"),
    bing: textOf(verification, "bing"),
    yandex: textOf(verification, "yandex"),
    baidu: textOf(verification, "baidu"),
    pinterest: textOf(verification, "pinterest"),
  });
  // The verification files are kept as site.toml has them.
  const save = () => onSave({ verification: cleaned({ ...verification, ...texts }) });
  return (
    <SectionForm lede={words.lede} onSave={save}>
      {ENGINES.map((engine) => (
        <TextField
          key={engine}
          id={`seo-verify-${engine}`}
          label={words.engines[engine]}
          hint={words.codeHint(words.engines[engine])}
          value={texts[engine]}
          onChange={set(engine)}
        />
      ))}
    </SectionForm>
  );
}
