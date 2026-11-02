import { useRouterState } from "@tanstack/react-router";
import type { ReactNode, RefObject } from "react";
import { shellWords } from "../strings/shellWords";
import { Icon } from "./Icon";
import { whereIs } from "./navigation";

type Props = {
  /** Whether the side menu is open over the page on a small screen. */
  open: boolean;
  /** Opens the side menu on a small screen. */
  onMenu: () => void;
  /** The menu button, which takes the focus back when the menu closes. */
  opener: RefObject<HTMLButtonElement | null>;
  /** The actions at the right end of the bar. */
  children: ReactNode;
};

/** The bar above every screen: the menu button on small screens, where you are, then the actions. */
export function TopBar({ open, onMenu, opener, children }: Props) {
  const path = useRouterState({ select: (state) => state.location.pathname });
  const where = whereIs(path);
  return (
    <header className="top">
      <button
        ref={opener}
        type="button"
        className="btn ghost menu-button"
        aria-label={shellWords.openMenu}
        aria-expanded={open}
        aria-controls="side-menu"
        onClick={onMenu}
      >
        <Icon name="menu" />
      </button>
      {where === null ? null : (
        <p className="crumb">
          <span className="crumb-group">{where.group.label}</span>
          <span className="crumb-line" aria-hidden="true">
            /
          </span>
          <span>{where.item.label}</span>
        </p>
      )}
      <div className="top-actions">{children}</div>
    </header>
  );
}
