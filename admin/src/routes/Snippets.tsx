import { useQueryClient } from "@tanstack/react-query";
import { useState } from "react";
import { failureMessage, isData } from "../api/call";
import { useSnippets } from "../api/snippets";
import { Problem } from "../components/Problem";
import { SnippetChange, type SnippetPick } from "../snippets/SnippetChange";
import { SnippetTable } from "../snippets/SnippetTable";
import { en } from "../strings/en";
import { snippetWords } from "../strings/snippetWords";

/** The site's snippets, each edited or deleted for every post that places it. */
export function Snippets() {
  const words = snippetWords;
  const queries = useQueryClient();
  const listed = useSnippets();
  const [picked, setPicked] = useState<SnippetPick | null>(null);
  const [done, setDone] = useState<string | null>(null);

  const pick = (next: SnippetPick) => {
    setDone(null);
    setPicked(next);
  };
  const finish = async (message: string) => {
    setPicked(null);
    setDone(message);
    // The editor's snippet list reads the same query, so it sees the change too.
    await queries.invalidateQueries({ queryKey: ["snippets"] });
  };

  if (picked !== null) {
    return (
      <>
        <h1>{words.title}</h1>
        <SnippetChange pick={picked} onDone={finish} onBack={() => setPicked(null)} />
      </>
    );
  }

  let list = <p role="status">{words.list.loading}</p>;
  if (listed !== undefined && !isData(listed)) {
    list = <Problem message={failureMessage(listed)} />;
  } else if (isData(listed) && listed.data.length === 0) {
    list = <p role="status">{en.snippets.none}</p>;
  } else if (isData(listed)) {
    list = <SnippetTable snippets={listed.data} onPick={pick} />;
  }
  return (
    <>
      <h1>{words.title}</h1>
      <p className="lede">{words.lede}</p>
      <p role="status">{done}</p>
      <h2 id="snippets-title">{words.list.title}</h2>
      {list}
    </>
  );
}
