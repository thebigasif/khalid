import { Link, useNavigate } from "@tanstack/react-router";
import { type FormEvent, useEffect, useRef, useState } from "react";
import { usePostSearch } from "../api/postSearch";
import { isPage } from "../api/rows";
import { shellWords } from "../strings/shellWords";
import { Icon } from "./Icon";
import { NAV } from "./navigation";

const SCREENS = NAV.flatMap((group) => group.items);

type Props = {
  /** Closes the search; `true` gives the focus back to the button that opened it. */
  close: (giveBack: boolean) => void;
};

/**
 * The open search: one field, the screens whose names hold the words, and
 * the posts whose titles do, the latest posts while nothing is typed.
 * Enter opens the first one; Escape or a click elsewhere closes it.
 */
export function CommandPanel({ close }: Props) {
  const [words, setWords] = useState("");
  const panel = useRef<HTMLDialogElement>(null);
  const field = useRef<HTMLInputElement>(null);
  const navigate = useNavigate();
  const found = usePostSearch(words);
  const wanted = words.trim().toLowerCase();
  const screens =
    wanted === "" ? [] : SCREENS.filter((item) => item.label.toLowerCase().includes(wanted));
  const posts = isPage(found) ? found.rows : [];

  useEffect(() => {
    field.current?.focus();
    const onKey = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        close(true);
      }
    };
    const onPress = (event: MouseEvent) => {
      if (event.target instanceof Node && !panel.current?.contains(event.target)) {
        close(false);
      }
    };
    document.addEventListener("keydown", onKey);
    document.addEventListener("mousedown", onPress);
    return () => {
      document.removeEventListener("keydown", onKey);
      document.removeEventListener("mousedown", onPress);
    };
  }, [close]);

  const openFirst = async (event: FormEvent) => {
    event.preventDefault();
    const [screen] = screens;
    const [post] = posts;
    if (screen !== undefined) {
      await navigate({ to: screen.to });
    } else if (post !== undefined) {
      await navigate({ to: "/posts/$id", params: { id: post.id } });
    }
    close(false);
  };

  return (
    <dialog ref={panel} open className="command" aria-label={shellWords.search.label}>
      <form onSubmit={openFirst}>
        <label className="sr-only" htmlFor="command-words">
          {shellWords.search.label}
        </label>
        <input
          ref={field}
          id="command-words"
          type="search"
          autoComplete="off"
          value={words}
          onChange={(event) => setWords(event.target.value)}
        />
      </form>
      {screens.length > 0 ? (
        <>
          <p className="command-label" id="found-screens">
            {shellWords.search.screens}
          </p>
          <ul className="command-list" aria-labelledby="found-screens">
            {screens.map((item) => (
              <li key={item.to}>
                <Link to={item.to} onClick={() => close(false)}>
                  <Icon name={item.icon} />
                  {item.label}
                </Link>
              </li>
            ))}
          </ul>
        </>
      ) : null}
      {posts.length > 0 ? (
        <>
          <p className="command-label" id="found-posts">
            {wanted === "" ? shellWords.search.latest : shellWords.search.posts}
          </p>
          <ul className="command-list" aria-labelledby="found-posts">
            {posts.map((post) => (
              <li key={post.id}>
                <Link to="/posts/$id" params={{ id: post.id }} onClick={() => close(false)}>
                  <Icon name="posts" />
                  {post.title}
                </Link>
              </li>
            ))}
          </ul>
        </>
      ) : null}
      {found !== undefined && screens.length === 0 && posts.length === 0 ? (
        <p className="command-note">{shellWords.search.nothing}</p>
      ) : null}
    </dialog>
  );
}
