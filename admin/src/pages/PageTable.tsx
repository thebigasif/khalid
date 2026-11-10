import type { AdminPage } from "../api/pages";
import { languageName } from "../strings/languages";
import { pagesWords } from "../strings/pagesWords";

type Props = { pages: AdminPage[] };

/** Every page, one row per language. */
export function PageTable({ pages }: Props) {
  const words = pagesWords.list;
  return (
    <div className="scroll">
      <table aria-labelledby="pages-title">
        <thead>
          <tr>
            <th scope="col">{words.name}</th>
            <th scope="col">{words.slug}</th>
            <th scope="col">{words.language}</th>
          </tr>
        </thead>
        <tbody>
          {pages.map((page) => (
            <tr key={page.id}>
              <td>{page.title}</td>
              <td data-label={words.slug}>
                <code>{page.slug}</code>
              </td>
              <td data-label={words.language}>{languageName(page.language)}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
