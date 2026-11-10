import type { AdminCategory } from "../api/terms";
import { languageName } from "../strings/languages";
import { termWords } from "../strings/termWords";
import type { CategoryPick } from "./CategoryChange";
import { categoryName } from "./names";

type Props = {
  categories: AdminCategory[];
  language: string;
  onPick: (category: AdminCategory, action: CategoryPick["action"]) => void;
};

/** Every category: its name in `language` first, its other names under it, its posts, and what to do. */
export function CategoryTable({ categories, language, onPick }: Props) {
  const words = termWords.categories;
  return (
    <div className="scroll">
      <table aria-labelledby="categories-title">
        <thead>
          <tr>
            <th scope="col">{words.name}</th>
            <th scope="col">{words.slug}</th>
            <th scope="col">{words.posts}</th>
            <th scope="col">
              <span className="muted">{words.action}</span>
            </th>
          </tr>
        </thead>
        <tbody>
          {categories.map((category) => {
            const name = categoryName(category, language);
            return (
              <tr key={category.id}>
                <td>
                  {name}
                  {Object.entries(category.names)
                    .filter(([code]) => code !== language)
                    .map(([code, other]) => (
                      <span className="hint" key={code}>
                        {words.other(languageName(code), other)}
                      </span>
                    ))}
                </td>
                <td data-label={words.slug}>
                  <code>{category.slug}</code>
                </td>
                <td data-label={words.posts}>{words.count(category.posts)}</td>
                <td>
                  <span className="row">
                    <button
                      type="button"
                      className="btn"
                      aria-label={words.renameLabel(name)}
                      onClick={() => onPick(category, "rename")}
                    >
                      {words.rename}
                    </button>
                    {category.posts === 0 ? (
                      <button
                        type="button"
                        className="btn"
                        aria-label={words.removeLabel(name)}
                        onClick={() => onPick(category, "remove")}
                      >
                        {words.remove}
                      </button>
                    ) : null}
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
