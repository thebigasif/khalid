import type { AdminAuthor } from "../api/authors";

type Props = { author: AdminAuthor };

/**
 * The author's photo, round, when it is an address the admin can show; a
 * file beside the author's file shows only on the blog, so it gets the
 * initial instead. The name beside it says who it is, so it has no alt.
 */
export function AuthorAvatar({ author }: Props) {
  const photo = author.photo ?? "";
  if (photo.startsWith("/") || photo.startsWith("https://")) {
    return <img className="author-avatar" src={photo} alt="" width={36} height={36} />;
  }
  return (
    <span className="author-avatar author-initial" aria-hidden="true">
      {author.name.slice(0, 1).toUpperCase()}
    </span>
  );
}
