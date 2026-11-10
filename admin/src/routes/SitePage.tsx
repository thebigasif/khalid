import { useQuery, useQueryClient } from "@tanstack/react-query";
import { getRouteApi, Link } from "@tanstack/react-router";
import { useState } from "react";
import { failureMessage, isData } from "../api/call";
import { type AdminPage, editPage, readPage } from "../api/pages";
import { Problem } from "../components/Problem";
import { DeletePage } from "../pages/DeletePage";
import { changes, type PageFields } from "../pages/edits";
import { PageForm } from "../pages/PageForm";
import type { UserMessage } from "../strings/en";
import { languageName } from "../strings/languages";
import { pagesWords } from "../strings/pagesWords";
import { PAGES } from "./Pages";

const route = getRouteApi("/signed-in/pages/$id");

/** One page's screen: what its file says, saved field by field, and deleting it. */
export function SitePage() {
  const words = pagesWords.edit;
  const { id } = route.useParams();
  const queries = useQueryClient();
  const read = useQuery({ queryKey: [...PAGES, id], queryFn: () => readPage(id) }).data;
  const [said, setSaid] = useState<string | null>(null);
  const [failed, setFailed] = useState<UserMessage | null>(null);

  const save = async (page: AdminPage, fields: PageFields) => {
    const edit = changes(page, fields);
    if (edit === null) {
      setSaid(words.unchanged);
      return;
    }
    const reply = await editPage(page.id, edit);
    setSaid(isData(reply) ? words.saved : null);
    setFailed(isData(reply) ? null : failureMessage(reply));
    await queries.invalidateQueries({ queryKey: PAGES });
  };

  const page = isData(read) ? read.data[0] : undefined;
  let shown = <p role="status">{words.loading}</p>;
  if (read !== undefined && !isData(read)) {
    shown = <Problem message={failureMessage(read)} />;
  } else if (page !== undefined) {
    shown = (
      <>
        <p className="lede">{words.lede(page.slug, languageName(page.language))}</p>
        {failed === null ? null : <Problem message={failed} />}
        <p role="status">{said}</p>
        <PageForm page={page} onSave={(fields) => save(page, fields)} />
        <DeletePage page={page} />
      </>
    );
  }
  return (
    <>
      <p>
        <Link to="/pages">{words.back}</Link>
      </p>
      <h1>{page?.title ?? words.title}</h1>
      {shown}
    </>
  );
}
