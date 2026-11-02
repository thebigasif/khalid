import type { ReactNode } from "react";

type Props = {
  title: string;
  /** One or two sentences on what the screen is for. */
  lede?: string;
  /** The screen's own actions, at the right on a wide screen. */
  children?: ReactNode;
};

/** The top of every screen: its one heading, what it is for, and its actions. */
export function PageHeader({ title, lede, children }: Props) {
  return (
    <div className="page-head">
      <div>
        <h1>{title}</h1>
        {lede === undefined ? null : <p className="lede">{lede}</p>}
      </div>
      {children === undefined ? null : <div className="page-actions">{children}</div>}
    </div>
  );
}
