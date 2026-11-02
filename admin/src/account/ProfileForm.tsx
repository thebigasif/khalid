import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useState } from "react";
import { type AdminAuthor, editAuthor, readAuthor } from "../api/authors";
import { failureMessage, isData } from "../api/call";
import { AuthorForm } from "../authors/AuthorForm";
import { type AuthorFields, changes } from "../authors/edits";
import { Problem } from "../components/Problem";
import { AUTHORS } from "../routes/Authors";
import { en, type UserMessage } from "../strings/en";

type Props = { id: string };

/** The author `id`'s fields, saved field by field as the Authors screen saves them. */
export function ProfileForm({ id }: Props) {
  const queries = useQueryClient();
  const read = useQuery({ queryKey: [...AUTHORS, id], queryFn: () => readAuthor(id) }).data;
  const [said, setSaid] = useState<string | null>(null);
  const [failed, setFailed] = useState<UserMessage | null>(null);

  const save = async (author: AdminAuthor, fields: AuthorFields) => {
    const edit = changes(author, fields);
    if (edit === null) {
      setSaid(en.authors.edit.unchanged);
      return;
    }
    const reply = await editAuthor(author.id, edit);
    setSaid(isData(reply) ? en.authors.edit.saved : null);
    setFailed(isData(reply) ? null : failureMessage(reply));
    await queries.invalidateQueries({ queryKey: AUTHORS });
  };

  if (read === undefined) {
    return <p role="status">{en.account.profile.loading}</p>;
  }
  if (!isData(read)) {
    return <Problem message={failureMessage(read)} />;
  }
  const author = read.data[0];
  if (author === undefined) {
    return <p>{en.account.profile.none}</p>;
  }
  return (
    <>
      {failed === null ? null : <Problem message={failed} />}
      <p role="status">{said}</p>
      <AuthorForm author={author} onSave={(fields) => save(author, fields)} />
    </>
  );
}
