import { Link } from "@tanstack/react-router";

type Props = { id?: string | null; title: string };

/** A page by its title: a link to the editor when it is a post, plain text otherwise. */
export function PageName({ id, title }: Props) {
  if (id == null) {
    return <>{title}</>;
  }
  return (
    <Link to="/posts/$id" params={{ id }}>
      {title}
    </Link>
  );
}
