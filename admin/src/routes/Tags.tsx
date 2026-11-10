import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useState } from "react";
import { failureMessage, isData } from "../api/call";
import { listTags } from "../api/terms";
import { Problem } from "../components/Problem";
import { termWords } from "../strings/termWords";
import { TagChange, type TagPick } from "../terms/TagChange";
import { TagTable } from "../terms/TagTable";

/** The query key the screen refreshes after a tag is renamed or removed. */
export const TAGS = ["tags"];

/** The site's tags, each renamed or removed in every post at once. */
export function Tags() {
  const words = termWords;
  const queries = useQueryClient();
  const listed = useQuery({ queryKey: TAGS, queryFn: listTags }).data;
  const [picked, setPicked] = useState<TagPick | null>(null);
  const [done, setDone] = useState<string | null>(null);

  const finish = async (message: string) => {
    setPicked(null);
    setDone(message);
    await queries.invalidateQueries({ queryKey: TAGS });
  };

  if (picked !== null) {
    return (
      <>
        <h1>{words.title}</h1>
        <TagChange pick={picked} onDone={finish} onBack={() => setPicked(null)} />
      </>
    );
  }

  let tags = <p role="status">{words.tags.loading}</p>;
  if (listed !== undefined && !isData(listed)) {
    tags = <Problem message={failureMessage(listed)} />;
  } else if (isData(listed) && listed.data.length === 0) {
    tags = <p role="status">{words.tags.none}</p>;
  } else if (isData(listed)) {
    tags = (
      <TagTable
        tags={listed.data}
        onPick={(pick) => {
          setDone(null);
          setPicked(pick);
        }}
      />
    );
  }
  return (
    <>
      <h1>{words.title}</h1>
      <p className="lede">{words.lede}</p>
      <p role="status">{done}</p>
      <h2 id="tags-title">{words.tags.title}</h2>
      <p className="muted">{words.tags.lede}</p>
      {tags}
    </>
  );
}
