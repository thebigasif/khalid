import type { AdminTag } from "../api/terms";
import { languageName } from "../strings/languages";
import { termWords } from "../strings/termWords";

type Props = { tags: AdminTag[] };

/** Every tag, one row per language, with how many posts have it. */
export function TagTable({ tags }: Props) {
  const words = termWords.tags;
  return (
    <div className="scroll">
      <table aria-labelledby="tags-title">
        <thead>
          <tr>
            <th scope="col">{words.name}</th>
            <th scope="col">{words.language}</th>
            <th scope="col">{words.posts}</th>
          </tr>
        </thead>
        <tbody>
          {tags.map((tag) => (
            <tr key={tag.id}>
              <td>{tag.name}</td>
              <td data-label={words.language}>{languageName(tag.language)}</td>
              <td data-label={words.posts}>{words.count(tag.posts)}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
