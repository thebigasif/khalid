import { type FormEvent, useState } from "react";
import type { NewPage } from "../api/pages";
import { pagesWords } from "../strings/pagesWords";
import { PageField } from "./PageField";

type Props = { onAdd: (page: NewPage) => Promise<boolean> };

const EMPTY = { title: "", slug: "", language: "", description: "", markdown: "" };

/** The screen's focus: a new page's title, address, language, description and text. */
export function NewPageForm({ onAdd }: Props) {
  const [values, setValues] = useState(EMPTY);
  const [busy, setBusy] = useState(false);
  const set = (key: keyof typeof EMPTY) => (value: string) =>
    setValues({ ...values, [key]: value });

  const submit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setBusy(true);
    const language = values.language.trim();
    const description = values.description.trim();
    const added = await onAdd({
      title: values.title.trim(),
      slug: values.slug.trim(),
      markdown: values.markdown,
      language: language === "" ? null : language,
      description: description === "" ? null : description,
    });
    setBusy(false);
    if (added) {
      setValues(EMPTY);
    }
  };

  return (
    <form onSubmit={submit}>
      <div className="pair">
        <PageField name="title" form="new" value={values.title} onChange={set("title")} required />
        <PageField name="slug" form="new" value={values.slug} onChange={set("slug")} required />
        <PageField name="language" form="new" value={values.language} onChange={set("language")} />
        <PageField
          name="description"
          form="new"
          value={values.description}
          onChange={set("description")}
        />
      </div>
      <PageField name="markdown" form="new" value={values.markdown} onChange={set("markdown")} />
      <p className="muted">{pagesWords.add.live}</p>
      <button type="submit" className="btn primary" disabled={busy}>
        {pagesWords.add.submit}
      </button>
    </form>
  );
}
