import type { PostNumbers } from "../api/searchConsole";
import { count } from "../strings/dates";
import { en } from "../strings/en";
import { searchConsoleWords } from "../strings/searchConsoleWords";
import { PageName } from "./PageName";

type Props = { posts: PostNumbers[] };

const POSITION = new Intl.NumberFormat(en.locale, {
  minimumFractionDigits: 1,
  maximumFractionDigits: 1,
});

const SHARE = new Intl.NumberFormat(en.locale, {
  style: "percent",
  minimumFractionDigits: 1,
  maximumFractionDigits: 1,
});

/** Every published post's clicks, times shown, share clicked and position, each linked to its editor. */
export function PostMonth({ posts }: Props) {
  const words = searchConsoleWords.month;
  return (
    <section aria-labelledby="month-title">
      <h2 id="month-title">{words.title}</h2>
      <dl className="terms">
        {words.terms.map(({ term, meaning }) => (
          <div key={term}>
            <dt>{term}</dt>
            <dd>{meaning}</dd>
          </div>
        ))}
      </dl>
      <div className="scroll">
        <table aria-labelledby="month-title">
          <caption className="muted">{words.caption}</caption>
          <thead>
            <tr>
              <th scope="col">{words.post}</th>
              <th scope="col">{words.clicks}</th>
              <th scope="col">{words.shown}</th>
              <th scope="col">{words.clicked}</th>
              <th scope="col">{words.position}</th>
            </tr>
          </thead>
          <tbody>
            {posts.map((post) => (
              <tr key={post.id}>
                <th scope="row">
                  <PageName id={post.id} title={post.title} />
                </th>
                <td data-label={words.clicks}>{count(Math.round(post.clicks))}</td>
                <td data-label={words.shown}>{count(Math.round(post.impressions))}</td>
                <td data-label={words.clicked}>{SHARE.format(post.ctr)}</td>
                <td data-label={words.position}>
                  {post.position == null ? words.notShown : POSITION.format(post.position)}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </section>
  );
}
