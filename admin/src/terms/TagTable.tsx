import type { AdminTag } from "../api/terms";
import { languageName } from "../strings/languages";
import { termWords } from "../strings/termWords";
import type { TagPick } from "./TagChange";

type Props = { tags: AdminTag[]; onPick: (pick: TagPick) => void };

/** Every tag, one row per language, with how many posts have it and what to do with it. */
export function TagTable({ tags, onPick }: Props) {
  const words = termWords.tags;
  return (
    <div className="scroll">
      <table aria-labelledby="tags-title">
        <thead>
          <tr>
            <th scope="col">{words.name}</th>
            <th scope="col">{words.language}</th>
            <th scope="col">{words.posts}</th>
            <th scope="col">
              <span className="muted">{words.action}</span>
            </th>
          </tr>
        </thead>
        <tbody>
          {tags.map((tag) => {
            const language = languageName(tag.language);
            return (
              <tr key={tag.id}>
                <td>{tag.name}</td>
                <td data-label={words.language}>{language}</td>
                <td data-label={words.posts}>{words.count(tag.posts)}</td>
                <td>
                  <span className="row">
                    <button
                      type="button"
                      className="btn"
                      aria-label={words.renameLabel(tag.name, language)}
                      onClick={() => onPick({ tag, action: "rename" })}
                    >
                      {words.rename}
                    </button>
                    <button
                      type="button"
                      className="btn"
                      aria-label={words.removeLabel(tag.name, language)}
                      onClick={() => onPick({ tag, action: "remove" })}
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
