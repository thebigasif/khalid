import type { AdminCategory } from "../api/terms";
import { languageName } from "../strings/languages";
import { termWords } from "../strings/termWords";
import { categoryName } from "./names";

type Props = { categories: AdminCategory[]; language: string };

/** Every category: its name in `language` first, its other names under it, and its posts. */
export function CategoryTable({ categories, language }: Props) {
  const words = termWords.categories;
  return (
    <div className="scroll">
      <table aria-labelledby="categories-title">
        <thead>
          <tr>
            <th scope="col">{words.name}</th>
            <th scope="col">{words.slug}</th>
            <th scope="col">{words.posts}</th>
          </tr>
        </thead>
        <tbody>
          {categories.map((category) => (
            <tr key={category.id}>
              <td>
                {categoryName(category, language)}
                {Object.entries(category.names)
                  .filter(([code]) => code !== language)
                  .map(([code, name]) => (
                    <span className="hint" key={code}>
                      {words.other(languageName(code), name)}
                    </span>
                  ))}
              </td>
              <td data-label={words.slug}>
                <code>{category.slug}</code>
              </td>
              <td data-label={words.posts}>{words.count(category.posts)}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
