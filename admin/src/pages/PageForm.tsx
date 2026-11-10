import { type FormEvent, useState } from "react";
import type { AdminPage } from "../api/pages";
import { pagesWords } from "../strings/pagesWords";
import { fieldsOf, type PageFields } from "./edits";
import { PageField } from "./PageField";

type Props = { page: AdminPage; onSave: (fields: PageFields) => Promise<void> };

/** Everything a page's file says, ready to change. */
export function PageForm({ page, onSave }: Props) {
  const [fields, setFields] = useState(fieldsOf(page));
  const [busy, setBusy] = useState(false);
  const set = (key: keyof PageFields) => (value: string) => setFields({ ...fields, [key]: value });

  const submit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setBusy(true);
    await onSave(fields);
    setBusy(false);
  };

  return (
    <form onSubmit={submit}>
      <PageField name="title" form="edit" value={fields.title} onChange={set("title")} required />
      <PageField
        name="description"
        form="edit"
        value={fields.description}
        onChange={set("description")}
      />
      <PageField name="markdown" form="edit" value={fields.markdown} onChange={set("markdown")} />
      <button type="submit" className="btn primary" disabled={busy}>
        {pagesWords.edit.save}
      </button>
    </form>
  );
}
