import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useState } from "react";
import { failureMessage, isData } from "../api/call";
import { createPage, listPages, type NewPage } from "../api/pages";
import { Problem } from "../components/Problem";
import { NewPageForm } from "../pages/NewPageForm";
import { PageTable } from "../pages/PageTable";
import type { UserMessage } from "../strings/en";
import { pagesWords } from "../strings/pagesWords";

/** The query key the screen refreshes after a page is added, changed or deleted. */
export const PAGES = ["pages"];

/** The site's pages, such as About, and adding one. */
export function Pages() {
  const words = pagesWords;
  const queries = useQueryClient();
  const listed = useQuery({ queryKey: PAGES, queryFn: listPages }).data;
  const [done, setDone] = useState<string | null>(null);
  const [failed, setFailed] = useState<UserMessage | null>(null);

  const add = async (page: NewPage) => {
    const reply = await createPage(page);
    if (!isData(reply)) {
      setDone(null);
      setFailed(failureMessage(reply));
      return false;
    }
    setFailed(null);
    setDone(words.add.added(page.title));
    await queries.invalidateQueries({ queryKey: PAGES });
    return true;
  };

  let list = <p role="status">{words.list.loading}</p>;
  if (listed !== undefined && !isData(listed)) {
    list = <Problem message={failureMessage(listed)} />;
  } else if (isData(listed) && listed.data.length === 0) {
    list = <p role="status">{words.list.none}</p>;
  } else if (isData(listed)) {
    list = <PageTable pages={listed.data} />;
  }
  return (
    <>
      <h1>{words.title}</h1>
      <p className="lede">{words.lede}</p>
      <section className="focus wide" aria-labelledby="add-page-heading">
        <h2 id="add-page-heading">{words.add.title}</h2>
        {failed === null ? null : <Problem message={failed} />}
        <p role="status">{done}</p>
        <NewPageForm onAdd={add} />
      </section>
      <h2 id="pages-title">{words.list.title}</h2>
      {list}
    </>
  );
}
