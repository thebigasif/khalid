import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useNavigate } from "@tanstack/react-router";
import { useState } from "react";
import { failureMessage, isData } from "../api/call";
import { type AdminPage, deletePage, listPages } from "../api/pages";
import { addRedirect } from "../api/redirects";
import { Problem } from "../components/Problem";
import type { Farewell } from "../posts/bulk";
import { FarewellChoice, type Target } from "../posts/FarewellChoice";
import { PAGES } from "../routes/Pages";
import type { UserMessage } from "../strings/en";
import { pagesWords } from "../strings/pagesWords";

type Props = { page: AdminPage };

/** The site's other pages with an address, which the page's old address may lead to. */
function otherPages(pages: AdminPage[], leaving: AdminPage): Target[] {
  return pages.flatMap((other) =>
    other.id === leaving.id || other.path === undefined || other.path === null
      ? []
      : [{ title: other.title, path: other.path }],
  );
}

/**
 * Moves a page to the trash, then stores what its old address does: a 410, or a 301 to
 * another page or post, so links to it need no change by hand.
 */
export function DeletePage({ page }: Props) {
  const words = pagesWords.edit.remove;
  const navigate = useNavigate();
  const queries = useQueryClient();
  const listed = useQuery({ queryKey: PAGES, queryFn: listPages }).data;
  const [farewell, setFarewell] = useState<Farewell>({ kind: "gone" });
  const [failed, setFailed] = useState<UserMessage | null>(null);
  const [deleted, setDeleted] = useState(false);
  const from = page.path ?? null;

  const remove = async () => {
    const reply = await deletePage(page.id, page.updated_at);
    if (!isData(reply)) {
      setFailed(failureMessage(reply));
      return;
    }
    await queries.invalidateQueries({ queryKey: PAGES });
    if (from !== null) {
      const stored = await addRedirect(from, farewell.kind === "moved" ? farewell.to : null);
      if (!isData(stored)) {
        setFailed(words.noFarewell(page.title, from, failureMessage(stored).what));
        setDeleted(true);
        return;
      }
    }
    await navigate({ to: "/pages" });
  };

  return (
    <section aria-labelledby="delete-page-title">
      <h2 id="delete-page-title">{words.title}</h2>
      <p className="muted">{words.lede(page.title)}</p>
      {from === null || deleted ? null : (
        <FarewellChoice
          leaving={[]}
          also={isData(listed) ? otherPages(listed.data, page) : []}
          farewell={farewell}
          onChange={setFarewell}
          words={words}
          name="page"
        />
      )}
      {failed === null ? null : <Problem message={failed} />}
      {deleted ? null : (
        <button type="button" className="btn" onClick={remove}>
          {words.submit(page.title)}
        </button>
      )}
    </section>
  );
}
