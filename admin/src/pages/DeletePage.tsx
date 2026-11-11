import { useNavigate } from "@tanstack/react-router";
import { useState } from "react";
import { failureMessage, isData } from "../api/call";
import { type AdminPage, deletePage } from "../api/pages";
import { Problem } from "../components/Problem";
import type { UserMessage } from "../strings/en";
import { pagesWords } from "../strings/pagesWords";

type Props = { page: AdminPage };

/** Moves a page to the trash, after saying what happens to links to it. */
export function DeletePage({ page }: Props) {
  const words = pagesWords.edit.remove;
  const navigate = useNavigate();
  const [failed, setFailed] = useState<UserMessage | null>(null);

  const remove = async () => {
    const reply = await deletePage(page.id, page.updated_at);
    if (isData(reply)) {
      await navigate({ to: "/pages" });
    } else {
      setFailed(failureMessage(reply));
    }
  };

  return (
    <section aria-labelledby="delete-page-title">
      <h2 id="delete-page-title">{words.title}</h2>
      <p className="muted">{words.lede(page.title)}</p>
      {failed === null ? null : <Problem message={failed} />}
      <button type="button" className="btn" onClick={remove}>
        {words.submit(page.title)}
      </button>
    </section>
  );
}
