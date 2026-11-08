import type { PrivacyProps } from "../routes/Privacy";
import { SectionForm, TextField, useTexts } from "../seo/Fields";
import { cleaned } from "../seo/parts";
import { privacyWords } from "../strings/privacyWords";
import { contactMissing } from "./parts";

/** Who runs the blog, for the privacy page, with a warning while the law's minimum is missing. */
export function Contact({ site, onSave }: PrivacyProps) {
  const words = privacyWords.contact;
  const contact = site.privacy?.contact ?? {};
  const { texts, set } = useTexts({
    name: contact.name ?? "",
    email: contact.email ?? "",
    address: contact.address ?? "",
  });
  const save = () => onSave({ privacy: { contact: cleaned({ ...contact, ...texts }) } });

  return (
    <>
      {contactMissing(site.privacy) ? (
        <p className="careful">
          <strong>{words.missing}</strong> {words.missingWhy}
        </p>
      ) : null}
      <SectionForm lede={words.lede} onSave={save}>
        <TextField
          id="privacy-contact-name"
          label={words.name}
          hint={words.nameHint}
          value={texts.name}
          onChange={set("name")}
        />
        <TextField
          id="privacy-contact-email"
          type="email"
          label={words.email}
          hint={words.emailHint}
          value={texts.email}
          onChange={set("email")}
        />
        <TextField
          id="privacy-contact-address"
          rows={3}
          label={words.address}
          hint={words.addressHint}
          value={texts.address}
          onChange={set("address")}
        />
      </SectionForm>
    </>
  );
}
