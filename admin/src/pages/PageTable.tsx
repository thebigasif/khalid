import { Link } from "@tanstack/react-router";
import type { AdminPage } from "../api/pages";
import { languageName } from "../strings/languages";
import { pagesWords } from "../strings/pagesWords";

type Props = { pages: AdminPage[] };

/** Every page, one row per language, each opening its own screen. */
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
            <th scope="col">
              <span className="muted">{words.action}</span>
            </th>
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
              <td>
                <Link
                  to="/pages/$id"
                  params={{ id: page.id }}
                  className="btn"
                  aria-label={words.editLabel(page.title, languageName(page.language))}
                >
                  {words.edit}
                </Link>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
