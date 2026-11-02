import { useQuery } from "@tanstack/react-query";
import { useState } from "react";
import { isData } from "../api/call";
import { type LinkSuggestion, readLinkSuggestions } from "../api/seo";
import { seoEditorWords } from "../strings/seoEditorWords";

type Props = {
  id: string;
  /** The saved copy's time, so the list follows each save. */
  version: string;
  /** Puts a link to `suggestion` into the text. */
  onAdd: (suggestion: LinkSuggestion) => void;
};

/** Why a suggestion is on the list, in plain words, each reason once. */
function reasons(suggestion: LinkSuggestion): string[] {
  const why = seoEditorWords.why;
  const { shares } = suggestion;
  return [
    shares.pillar ? why.pillar : shares.cluster ? why.cluster : null,
    shares.category ? why.category : null,
    shares.tags.length > 0 ? why.tags(shares.tags.join(", ")) : null,
    shares.keyword ? why.keyword(shares.keyword) : null,
  ].filter((reason) => reason !== null);
}

/** The blog's posts this one should link to, each with why, and a button that adds the link. */
export function LinkSuggestions({ id, version, onAdd }: Props) {
  const words = seoEditorWords;
  const read = useQuery({
    queryKey: ["link-suggestions", id, version],
    queryFn: () => readLinkSuggestions(id),
  }).data;
  const [added, setAdded] = useState<string[]>([]);
  if (!isData(read)) {
    return null;
  }
  const shown = read.data.filter((each) => !added.includes(each.id));
  return (
    <details>
      <summary>{words.links}</summary>
      <div>
        <p className="hint">{words.linksLede}</p>
        <p role="status">{added.length > 0 ? words.added : null}</p>
        {shown.length === 0 ? <p className="hint">{words.noLinks}</p> : null}
        <ul className="rows">
          {shown.map((suggestion) => (
            <li key={suggestion.id}>
              <div>
                <strong>{suggestion.title}</strong>
                <span className="hint">{reasons(suggestion).join(". ")}</span>
              </div>
              <button
                type="button"
                className="btn"
                aria-label={words.addLink(suggestion.title)}
                onClick={() => {
                  onAdd(suggestion);
                  setAdded([...added, suggestion.id]);
                }}
              >
                {words.addLinkShort}
              </button>
            </li>
          ))}
        </ul>
      </div>
    </details>
  );
}
