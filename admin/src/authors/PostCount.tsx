import { isPage, useRows } from "../api/rows";
import { en } from "../strings/en";

type Props = { slug: string };

/** How many published posts name the author `slug`, in every language. */
export function PostCount({ slug }: Props) {
  const rows = useRows({ filter: `author:${slug}+status:published`, limit: 1 });
  if (!isPage(rows)) {
    return <span className="muted">{en.authors.list.countUnknown}</span>;
  }
  return <>{en.authors.list.count(rows.pagination.total)}</>;
}
