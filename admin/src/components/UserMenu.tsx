import { useEffect, useRef, useState } from "react";
import { mountPath } from "../api/mount";
import { readKey } from "../auth/session";
import { en } from "../strings/en";
import { shellWords } from "../strings/shellWords";
import { Icon } from "./Icon";

type Props = { signOut: () => void };

/** The account button at the right of the top bar: how you signed in, the blog, and signing out. */
export function UserMenu({ signOut }: Props) {
  const [open, setOpen] = useState(false);
  const box = useRef<HTMLDivElement>(null);
  const button = useRef<HTMLButtonElement>(null);

  // The panel closes on Escape, giving the focus back, and on a click anywhere else.
  useEffect(() => {
    if (!open) {
      return;
    }
    const onKey = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        setOpen(false);
        button.current?.focus();
      }
    };
    const onPress = (event: MouseEvent) => {
      if (event.target instanceof Node && !box.current?.contains(event.target)) {
        setOpen(false);
      }
    };
    document.addEventListener("keydown", onKey);
    document.addEventListener("mousedown", onPress);
    return () => {
      document.removeEventListener("keydown", onKey);
      document.removeEventListener("mousedown", onPress);
    };
  }, [open]);

  return (
    <div className="user" ref={box}>
      <button
        ref={button}
        type="button"
        className="btn ghost user-button"
        aria-label={shellWords.accountMenu}
        aria-expanded={open}
        aria-controls={open ? "user-menu" : undefined}
        onClick={() => setOpen(!open)}
      >
        <span className="avatar">
          <Icon name="person" />
        </span>
        <Icon name="chevron" />
      </button>
      {open ? (
        <div id="user-menu" className="pop">
          <p className="pop-note">{readKey() === null ? shellWords.byEmail : shellWords.byToken}</p>
          <a className="pop-item" href={mountPath()}>
            <Icon name="external" />
            {en.viewBlog}
          </a>
          <button type="button" className="pop-item" onClick={signOut}>
            <Icon name="signOut" />
            {en.signOut}
          </button>
        </div>
      ) : null}
    </div>
  );
}
