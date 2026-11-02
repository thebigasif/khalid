import { useEffect, useRef, useState } from "react";
import { shellWords } from "../strings/shellWords";
import { CommandPanel } from "./CommandPanel";
import { Icon } from "./Icon";

/** Whether this computer names its command key "⌘" rather than "Ctrl". */
const APPLE = /Mac|iPhone|iPad/.test(navigator.userAgent);

/** Whether `event` is Cmd K or Ctrl K pressed outside the editor's text, which has its own. */
function isSearchKey(event: KeyboardEvent): boolean {
  const command = (event.metaKey || event.ctrlKey) && !event.altKey && !event.shiftKey;
  const inText = event.target instanceof Element && event.target.closest("[contenteditable]");
  return command && (event.key === "k" || event.key === "K") && !inText && !event.defaultPrevented;
}

/** The search button in the top bar; it, Cmd K or Ctrl K open the search for posts and screens. */
export function CommandSearch() {
  const [open, setOpen] = useState(false);
  const button = useRef<HTMLButtonElement>(null);

  useEffect(() => {
    const onKey = (event: KeyboardEvent) => {
      if (isSearchKey(event)) {
        event.preventDefault();
        setOpen(true);
      }
    };
    document.addEventListener("keydown", onKey);
    return () => document.removeEventListener("keydown", onKey);
  }, []);

  const close = (giveBack: boolean) => {
    setOpen(false);
    if (giveBack) {
      button.current?.focus();
    }
  };
  return (
    <>
      <button
        ref={button}
        type="button"
        className="search-button"
        aria-label={shellWords.search.label}
        aria-expanded={open}
        onClick={() => setOpen(true)}
      >
        <Icon name="search" />
        <span className="search-words">{shellWords.search.open}</span>
        <kbd aria-hidden="true">{APPLE ? "⌘K" : "Ctrl K"}</kbd>
      </button>
      {open ? <CommandPanel close={close} /> : null}
    </>
  );
}
