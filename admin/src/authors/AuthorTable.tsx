import { Link } from "@tanstack/react-router";
import type { AdminAuthor } from "../api/authors";
import { en } from "../strings/en";
import { languageName } from "../strings/languages";
import { AuthorAvatar } from "./AuthorAvatar";
import { PostCount } from "./PostCount";

type Props = { authors: AdminAuthor[] };

/** Every author, one row per language, with their photo and how many posts they wrote. */
export function AuthorTable({ authors }: Props) {
  const words = en.authors.list;
  return (
    <div className="scroll">
      <table aria-labelledby="authors-title">
        <thead>
          <tr>
            <th scope="col">{words.name}</th>
            <th scope="col">{words.slug}</th>
            <th scope="col">{words.language}</th>
            <th scope="col">{words.role}</th>
            <th scope="col">{words.posts}</th>
            <th scope="col">
              <span className="muted">{words.action}</span>
            </th>
          </tr>
        </thead>
        <tbody>
          {authors.map((author) => (
            <tr key={author.id}>
              <td>
                <span className="author-cell">
                  <AuthorAvatar author={author} />
                  {author.name}
                </span>
              </td>
              <td data-label={words.slug}>
                <code>{author.slug}</code>
              </td>
              <td data-label={words.language}>{languageName(author.language)}</td>
              <td data-label={words.role}>{author.role ?? words.noRole}</td>
              <td data-label={words.posts}>
                <PostCount slug={author.slug} />
              </td>
              <td>
                <Link
                  to="/authors/$id"
                  params={{ id: author.id }}
                  className="btn"
                  aria-label={words.editLabel(author.name, languageName(author.language))}
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
