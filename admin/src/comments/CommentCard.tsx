import type { AdminComment } from "../api/comments";
import { day } from "../strings/dates";
import { en } from "../strings/en";
import { spamWords } from "../strings/spamWords";
import { postName } from "./postName";
import { ReplyForm } from "./ReplyForm";

type Props = {
  comment: AdminComment;
  /** Whether this is the next comment to decide on, drawn as the screen's focus. */
  first: boolean;
  onApprove: (comment: AdminComment) => void;
  onDelete: (comment: AdminComment) => void;
  onReply: (comment: AdminComment, name: string, text: string) => Promise<boolean>;
  onSpam: (comment: AdminComment) => void;
  onNotSpam: (comment: AdminComment) => void;
};

type SpamProps = Pick<Props, "comment" | "onSpam" | "onNotSpam">;

/** Marks a reader's comment as spam, or, under Spam, moves it back to the waiting ones. */
function SpamButton({ comment, onSpam, onNotSpam }: SpamProps) {
  const name = comment.author_name;
  if (comment.status === "spam") {
    return (
      <button
        type="button"
        className="btn"
        aria-label={spamWords.notSpamLabel(name)}
        onClick={() => onNotSpam(comment)}
      >
        {spamWords.notSpam}
      </button>
    );
  }
  return (
    <button
      type="button"
      className="btn"
      aria-label={spamWords.spamLabel(name)}
      onClick={() => onSpam(comment)}
    >
      {spamWords.spam}
    </button>
  );
}

/** One comment: who wrote it on which post, what it says, and what to do with it. */
export function CommentCard(props: Props) {
  const { comment, first, onApprove, onDelete, onReply, onSpam, onNotSpam } = props;
  const words = en.comments;
  const name = comment.author_name;
  const byStaff = comment.author_kind === "staff";
  return (
    <li className={first ? "focus wide" : "card"}>
      <p className="flex flex-wrap items-baseline gap-2">
        <strong>{name}</strong>
        {byStaff ? <span className="pill quiet">{words.staff}</span> : null}
        <span className="muted">{words.on(postName(comment.post), day(comment.created_at))}</span>
      </p>
      <p className="said">{comment.text}</p>
      <div className="flex flex-wrap items-center gap-3">
        {comment.status === "approved" ? null : (
          <button
            type="button"
            className={first ? "btn primary" : "btn"}
            aria-label={words.approveLabel(name)}
            onClick={() => onApprove(comment)}
          >
            {words.approve}
          </button>
        )}
        {byStaff || comment.status === "trash" ? null : (
          <SpamButton comment={comment} onSpam={onSpam} onNotSpam={onNotSpam} />
        )}
        <button
          type="button"
          className="btn"
          aria-label={words.removeLabel(name)}
          onClick={() => onDelete(comment)}
        >
          {words.remove}
        </button>
      </div>
      {comment.status === "trash" ? null : (
        <ReplyForm id={comment.id} onSend={(author, text) => onReply(comment, author, text)} />
      )}
    </li>
  );
}
