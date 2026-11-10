import { useState } from "react";
import type { PrivacyProps } from "../routes/Privacy";
import { Check, SectionForm, TextField, useTexts } from "../seo/Fields";
import { cleaned } from "../seo/parts";
import { privacyWords } from "../strings/privacyWords";
import { imprintMissing } from "./parts";

/** The imprint page German law asks for: on or off, and its own lines beyond the contact. */
export function Imprint({ site, onSave }: PrivacyProps) {
  const words = privacyWords.imprint;
  const imprint = site.privacy?.imprint;
  const [on, setOn] = useState(imprint !== undefined);
  const { texts, set } = useTexts({
    phone: imprint?.phone ?? "",
    register: imprint?.register ?? "",
    vat_id: imprint?.vat_id ?? "",
    responsible: imprint?.responsible ?? "",
  });
  // An empty `[privacy.imprint]` still adds the page, so on with no lines saves `{}`.
  const save = () =>
    onSave({ privacy: { imprint: on ? cleaned({ ...imprint, ...texts }) ?? {} : null } });

  return (
    <>
      {on && imprintMissing(site.privacy) ? (
        <p className="careful">
          <strong>{words.missing}</strong> {words.missingWhy}
        </p>
      ) : null}
      <SectionForm lede={words.lede} onSave={save}>
        <Check
          id="privacy-imprint"
          label={words.show}
          hint={words.showHint}
          checked={on}
          onChange={setOn}
        />
        {on ? (
          <>
            <TextField
              id="privacy-imprint-phone"
              type="tel"
              label={words.phone}
              hint={words.phoneHint}
              value={texts.phone}
              onChange={set("phone")}
            />
            <TextField
              id="privacy-imprint-register"
              label={words.register}
              hint={words.registerHint}
              value={texts.register}
              onChange={set("register")}
            />
            <TextField
              id="privacy-imprint-vat"
              label={words.vatId}
              hint={words.vatIdHint}
              value={texts.vat_id}
              onChange={set("vat_id")}
            />
            <TextField
              id="privacy-imprint-responsible"
              rows={2}
              label={words.responsible}
              hint={words.responsibleHint}
              value={texts.responsible}
              onChange={set("responsible")}
            />
          </>
        ) : null}
      </SectionForm>
    </>
  );
}
