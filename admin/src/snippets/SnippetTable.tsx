import type { Snippet } from "../api/snippets";
import { day } from "../strings/dates";
import { languageName } from "../strings/languages";
import { snippetWords } from "../strings/snippetWords";
import type { SnippetPick } from "./SnippetChange";

type Props = { snippets: Snippet[]; onPick: (pick: SnippetPick) => void };

/** Every snippet, one row per language, with when it last changed and what to do with it. */
export function SnippetTable({ snippets, onPick }: Props) {
  const words = snippetWords.list;
  return (
    <div className="scroll">
      <table aria-labelledby="snippets-title">
        <thead>
          <tr>
            <th scope="col">{words.name}</th>
            <th scope="col">{words.language}</th>
            <th scope="col">{words.changed}</th>
            <th scope="col">
              <span className="muted">{words.action}</span>
            </th>
          </tr>
        </thead>
        <tbody>
          {snippets.map((snippet) => {
            const language = languageName(snippet.language);
            return (
              <tr key={snippet.id}>
                <td>{snippet.title}</td>
                <td data-label={words.language}>{language}</td>
                <td data-label={words.changed}>{day(snippet.updated_at)}</td>
                <td>
                  <span className="row">
                    <button
                      type="button"
                      className="btn"
                      aria-label={words.editLabel(snippet.title, language)}
                      onClick={() => onPick({ snippet, action: "edit" })}
                    >
                      {words.edit}
                    </button>
                    <button
                      type="button"
                      className="btn"
                      aria-label={words.removeLabel(snippet.title, language)}
                      onClick={() => onPick({ snippet, action: "remove" })}
                    >
                      {words.remove}
                    </button>
                  </span>
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}
