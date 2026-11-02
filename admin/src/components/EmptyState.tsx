import type { ReactNode } from "react";
import { Icon, type IconName } from "./Icon";

type Props = {
  icon: IconName;
  /** What is empty, in one short sentence. */
  title: string;
  /** The one clear next step: a sentence, a link or a button. */
  children: ReactNode;
};

/** What a list shows when it has nothing yet: never a blank panel, always one next step. */
export function EmptyState({ icon, title, children }: Props) {
  return (
    <div className="empty">
      <span className="empty-icon">
        <Icon name={icon} />
      </span>
      <p className="empty-title">{title}</p>
      {children}
    </div>
  );
}
