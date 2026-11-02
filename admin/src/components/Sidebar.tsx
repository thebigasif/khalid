import { Link } from "@tanstack/react-router";
import type { RefObject } from "react";
import { mountPath } from "../api/mount";
import { blogName } from "../auth/blog";
import { en } from "../strings/en";
import { shellWords } from "../strings/shellWords";
import { BlogMark } from "./BlogMark";
import { Icon } from "./Icon";
import { NAV } from "./navigation";

type Props = {
  /** Whether a small screen shows the menu over the page; a wide screen always shows it. */
  open: boolean;
  /** Hides the menu again on a small screen, after a choice or from its close button. */
  close: () => void;
  /** The close button, which takes the focus when the menu opens. */
  closer: RefObject<HTMLButtonElement | null>;
};

/** The side menu: the blog's mark and name, then every screen in its group, each with an icon. */
export function Sidebar({ open, close, closer }: Props) {
  return (
    <nav id="side-menu" className="side" data-open={open} aria-label={en.nav.label}>
      <div className="side-head">
        <a className="side-brand" href={mountPath()}>
          <BlogMark look="side" />
          <span>{blogName() ?? en.brand}</span>
        </a>
        <button
          ref={closer}
          type="button"
          className="btn ghost side-close"
          aria-label={shellWords.closeMenu}
          onClick={close}
        >
          <Icon name="close" />
        </button>
      </div>
      {NAV.map((group) => (
        <div key={group.label} className="side-group">
          <p className="side-label" id={`nav-${group.label}`}>
            {group.label}
          </p>
          <ul aria-labelledby={`nav-${group.label}`}>
            {group.items.map((item) => (
              <li key={item.to}>
                <Link
                  to={item.to}
                  activeOptions={{ exact: item.to === "/", includeSearch: false }}
                  onClick={close}
                >
                  <Icon name={item.icon} />
                  {item.label}
                </Link>
              </li>
            ))}
          </ul>
        </div>
      ))}
    </nav>
  );
}
