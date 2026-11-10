import { useQuery } from "@tanstack/react-query";
import { failureMessage, isData } from "../api/call";
import { listTags } from "../api/terms";
import { Problem } from "../components/Problem";
import { termWords } from "../strings/termWords";
import { TagTable } from "../terms/TagTable";

/** The query key of the site's tags. */
export const TAGS = ["tags"];

/** The site's tags in every language, with how many posts have each. */
export function Tags() {
  const words = termWords;
  const listed = useQuery({ queryKey: TAGS, queryFn: listTags }).data;

  let tags = <p role="status">{words.tags.loading}</p>;
  if (listed !== undefined && !isData(listed)) {
    tags = <Problem message={failureMessage(listed)} />;
  } else if (isData(listed) && listed.data.length === 0) {
    tags = <p role="status">{words.tags.none}</p>;
  } else if (isData(listed)) {
    tags = <TagTable tags={listed.data} />;
  }
  return (
    <>
      <h1>{words.title}</h1>
      <p className="lede">{words.lede}</p>
      <h2 id="tags-title">{words.tags.title}</h2>
      <p className="muted">{words.tags.lede}</p>
      {tags}
    </>
  );
}
