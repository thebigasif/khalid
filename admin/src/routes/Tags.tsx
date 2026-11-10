import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useState } from "react";
import { failureMessage, isData } from "../api/call";
import { listTags } from "../api/terms";
import { Problem } from "../components/Problem";
import { termWords } from "../strings/termWords";
import { CATEGORIES, Categories } from "../terms/Categories";
import { CategoryChange, type CategoryPick } from "../terms/CategoryChange";
import { TagChange, type TagPick } from "../terms/TagChange";
import { TagTable } from "../terms/TagTable";

/** The query key the screen refreshes after a tag is renamed or removed. */
export const TAGS = ["tags"];

/** The one change open on the screen, which takes the list's place. */
type Picked = { tag: TagPick } | { category: CategoryPick };

/** The site's categories and tags, each changed in every post at once. */
export function Tags() {
  const words = termWords;
  const queries = useQueryClient();
  const listed = useQuery({ queryKey: TAGS, queryFn: listTags }).data;
  const [picked, setPicked] = useState<Picked | null>(null);
  const [done, setDone] = useState<string | null>(null);

  const pick = (next: Picked) => {
    setDone(null);
    setPicked(next);
  };
  const finish = async (message: string) => {
    setPicked(null);
    setDone(message);
    await queries.invalidateQueries({ queryKey: TAGS });
    await queries.invalidateQueries({ queryKey: CATEGORIES });
  };

  if (picked !== null) {
    const back = () => setPicked(null);
    return (
      <>
        <h1>{words.title}</h1>
        {"tag" in picked ? (
          <TagChange pick={picked.tag} onDone={finish} onBack={back} />
        ) : (
          <CategoryChange pick={picked.category} onDone={finish} onBack={back} />
        )}
      </>
    );
  }

  let tags = <p role="status">{words.tags.loading}</p>;
  if (listed !== undefined && !isData(listed)) {
    tags = <Problem message={failureMessage(listed)} />;
  } else if (isData(listed) && listed.data.length === 0) {
    tags = <p role="status">{words.tags.none}</p>;
  } else if (isData(listed)) {
    tags = <TagTable tags={listed.data} onPick={(tag) => pick({ tag })} />;
  }
  return (
    <>
      <h1>{words.title}</h1>
      <p className="lede">{words.lede}</p>
      <p role="status">{done}</p>
      <Categories onPick={(category) => pick({ category })} />
      <h2 id="tags-title">{words.tags.title}</h2>
      <p className="muted">{words.tags.lede}</p>
      {tags}
    </>
  );
}
