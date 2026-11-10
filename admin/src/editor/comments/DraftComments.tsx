import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useState } from "react";
import { failureMessage, isData, type Reply } from "../../api/call";
import {
  addDraftComment,
  listDraftComments,
  type NewDraftComment,
  resolveThread,
} from "../../api/draftComments";
import { Problem } from "../../components/Problem";
import { draftCommentWords as words } from "../../strings/draftCommentWords";
import type { UserMessage } from "../../strings/en";
import { NewComment } from "./NewComment";
import { partNames } from "./parts";
import { type Send, ThreadItem } from "./ThreadItem";
import { type Thread, threadsOf } from "./threads";

type Props = { id: string; markdown: string };

type ListProps = { label: string; threads: Thread[]; parts: string[] } & Handlers;
type Handlers = {
  onAdd: Send<NewDraftComment>;
  onResolve: Send<{ first: number; resolved: boolean }>;
};

function Threads({ label, threads, parts, onAdd, onResolve }: ListProps) {
  return (
    <ol className="list-none p-0" aria-label={label}>
      {threads.map((thread) => (
        <ThreadItem
          key={thread.first.id}
          thread={thread}
          parts={parts}
          onAdd={onAdd}
          onResolve={onResolve}
        />
      ))}
    </ol>
  );
}

/** The threads once shown: open ones, then done ones behind a button, then a new comment. */
function Shown({ threads, parts, ...handlers }: Omit<ListProps, "label">) {
  const [doneShown, setDoneShown] = useState(false);
  const open = threads.filter((thread) => !thread.first.resolved);
  const done = threads.filter((thread) => thread.first.resolved);
  return (
    <>
      {open.length === 0 ? null : (
        <Threads label={words.openList} threads={open} parts={parts} {...handlers} />
      )}
      {done.length === 0 || doneShown ? null : (
        <button type="button" className="btn" onClick={() => setDoneShown(true)}>
          {words.showDone(done.length)}
        </button>
      )}
      {done.length > 0 && doneShown ? (
        <Threads label={words.doneList} threads={done} parts={parts} {...handlers} />
      ) : null}
      <NewComment parts={parts} onAdd={handlers.onAdd} />
    </>
  );
}

/** The notes staff leave each other on this post, under its text. Readers never see them. */
export function DraftComments({ id, markdown }: Props) {
  const [shown, setShown] = useState(false);
  const queries = useQueryClient();
  const key = ["draft-comments", id];
  const reply = useQuery({ queryKey: key, queryFn: () => listDraftComments(id) }).data;
  // Each change is saved, then the list is read again, so it shows what the server kept.
  const saved = async <T,>(sending: Promise<Reply<T>>): Promise<UserMessage | null> => {
    const answer = await sending;
    if (!isData(answer)) {
      return failureMessage(answer);
    }
    await queries.invalidateQueries({ queryKey: key });
    return null;
  };
  const onAdd: Send<NewDraftComment> = (comment) => saved(addDraftComment(id, comment));
  const onResolve: Send<{ first: number; resolved: boolean }> = ({ first, resolved }) =>
    saved(resolveThread(id, first, resolved));
  const threads = isData(reply) ? threadsOf(reply.data) : null;
  const open = threads?.filter((thread) => !thread.first.resolved).length ?? 0;
  let body = null;
  if (threads !== null) {
    body = (
      <Shown threads={threads} parts={partNames(markdown)} onAdd={onAdd} onResolve={onResolve} />
    );
  } else if (reply !== undefined && !isData(reply)) {
    body = <Problem message={failureMessage(reply)} />;
  }
  return (
    <section className="field" aria-labelledby="draft-comments-title">
      <h2 id="draft-comments-title">{words.title}</h2>
      <p className="hint">{words.hint}</p>
      {threads === null ? null : (
        <p className="hint">{open === 0 ? words.noneOpen : words.open(open)}</p>
      )}
      {shown ? (
        body
      ) : (
        <button type="button" className="btn" onClick={() => setShown(true)}>
          {words.show}
        </button>
      )}
    </section>
  );
}
