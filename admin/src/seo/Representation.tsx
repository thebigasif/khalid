import { useState } from "react";
import type { SectionProps } from "../routes/SeoSettings";
import { seoWords } from "../strings/seoWords";
import { SectionForm, TextField, useTexts } from "./Fields";
import { cleaned, lines, listOf, partOf, textOf } from "./parts";

const KINDS = ["organization", "person"] as const;
type Kind = (typeof KINDS)[number];
type Key = "name" | "logo" | "logoAlt" | "profiles" | "email" | "telephone" | "founded";
type Shape = { type?: "url" | "email" | "tel" | "date"; rows?: number };

/** Who is behind the blog, for the schema graph: a company or a person, and their profiles. */
export function Representation({ site, onSave }: SectionProps) {
  const words = seoWords.representation;
  const publisher = partOf(site.seo, "publisher");
  const logo = partOf(publisher, "logo");
  const written: Kind = textOf(publisher, "type") === "person" ? "person" : "organization";
  const [kind, setKind] = useState(written);
  const { texts, set } = useTexts<Key>({
    name: textOf(publisher, "name"),
    logo: textOf(logo, "url"),
    logoAlt: textOf(logo, "alt"),
    profiles: listOf(publisher, "same_as").join("\n"),
    email: textOf(publisher, "email"),
    telephone: textOf(publisher, "telephone"),
    founded: textOf(publisher, "founded"),
  });
  const field = (key: Key, shape: Shape = {}) => (
    <TextField
      id={`seo-${key}`}
      label={words[key]}
      hint={words[`${key}Hint` as const]}
      value={texts[key]}
      onChange={set(key)}
      {...shape}
    />
  );

  const save = () => {
    const url = texts.logo.trim();
    return onSave({
      publisher: cleaned({
        ...publisher,
        type: kind === "person" ? "person" : undefined,
        name: texts.name,
        logo: url === "" ? undefined : { ...logo, url, alt: texts.logoAlt },
        same_as: lines(texts.profiles),
        email: texts.email,
        telephone: texts.telephone,
        founded: kind === "person" ? undefined : texts.founded,
      }),
    });
  };

  return (
    <SectionForm lede={words.lede} onSave={save}>
      <fieldset>
        <legend>{words.kind}</legend>
        {KINDS.map((each) => (
          <div className="choice" key={each}>
            <input
              id={`seo-kind-${each}`}
              type="radio"
              name="seo-kind"
              checked={kind === each}
              onChange={() => setKind(each)}
            />
            <label htmlFor={`seo-kind-${each}`}>{words[each]}</label>
          </div>
        ))}
      </fieldset>
      {field("name")}
      <div className="pair">
        {field("logo", { type: "url" })}
        {field("logoAlt")}
      </div>
      {field("profiles", { rows: 4 })}
      <div className="pair">
        {field("email", { type: "email" })}
        {field("telephone", { type: "tel" })}
      </div>
      {kind === "person" ? null : field("founded", { type: "date" })}
    </SectionForm>
  );
}
