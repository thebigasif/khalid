import { Link } from "@tanstack/react-router";
import type { DragEvent } from "react";
import type { CalendarPost } from "../api/calendar";
import { en } from "../strings/en";

/** What a dragged draft carries: its identity, under Khalid's own type. */
export const DRAGGED = "application/x-khalid-post";

const TIME = new Intl.DateTimeFormat(en.locale, { hour: "2-digit", minute: "2-digit" });

/**
 * One post on a day: where it stands, its title linking to the editor, and
 * the time it goes live. A draft can be dragged onto another day.
 */
export function PostChip({ post }: { post: CalendarPost }) {
  const draft = post.status === "draft";
  const carry = (event: DragEvent) => {
    event.dataTransfer.setData(DRAGGED, post.id);
    event.dataTransfer.effectAllowed = "move";
  };
  return (
    <span className={`chip chip-${post.status}`}>
      <span className="sr-only">{en.status[post.status]}: </span>
      <Link
        to="/posts/$id"
        params={{ id: post.id }}
        draggable={draft}
        onDragStart={draft ? carry : undefined}
      >
        {post.title}
      </Link>
      {post.at === null || post.at === undefined ? null : (
        <time dateTime={post.at}>{TIME.format(new Date(post.at))}</time>
      )}
    </span>
  );
}
