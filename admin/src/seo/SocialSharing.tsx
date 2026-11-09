import type { SectionProps } from "../routes/SeoSettings";
import { seoWords } from "../strings/seoWords";
import { SectionForm, TextField, useTexts } from "./Fields";
import { cleaned, partOf, textOf } from "./parts";

/** The X username and the share image every page without its own uses. */
export function SocialSharing({ site, onSave }: SectionProps) {
  const words = seoWords.social;
  const social = partOf(site.seo, "social");
  const image = partOf(social, "image");
  const { texts, set } = useTexts({
    twitter: textOf(social, "twitter"),
    image: textOf(image, "url"),
    imageAlt: textOf(image, "alt"),
  });
  const save = () => {
    const url = texts.image.trim();
    const shared = url === "" ? undefined : { ...image, url, alt: texts.imageAlt };
    return onSave({ social: cleaned({ ...social, twitter: texts.twitter, image: shared }) });
  };
  return (
    <SectionForm lede={words.lede} onSave={save}>
      <TextField
        id="seo-twitter"
        label={words.twitter}
        hint={words.twitterHint}
        value={texts.twitter}
        onChange={set("twitter")}
      />
      <div className="pair">
        <TextField
          id="seo-share-image"
          type="url"
          label={words.image}
          hint={words.imageHint}
          value={texts.image}
          onChange={set("image")}
        />
        <TextField
          id="seo-share-alt"
          label={words.imageAlt}
          hint={words.imageAltHint}
          value={texts.imageAlt}
          onChange={set("imageAlt")}
        />
      </div>
    </SectionForm>
  );
}
