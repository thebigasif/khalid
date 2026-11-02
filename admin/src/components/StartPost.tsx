import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useNavigate } from "@tanstack/react-router";
import { type FormEvent, useState } from "react";
import { readSession } from "../api/account";
import { authorSlug } from "../api/authors";
import { isData } from "../api/call";
import { createPost, problemMessage, requestKey } from "../api/posts";
import { en, type UserMessage } from "../strings/en";
import { Problem } from "./Problem";

/** Starts a draft from a working title and opens it in the editor. */
export function StartPost() {
  const navigate = useNavigate();
  const [title, setTitle] = useState("");
  const [problem, setProblem] = useState<UserMessage | null>(null);
  // One key per attempt, so a double click or a lost answer makes one draft.
  const [key, setKey] = useState(requestKey);
  const queries = useQueryClient();
  const start = useMutation({
    mutationFn: async () => {
      // A new post names the signed-in person's own author profile, when they have one.
      const session = await queries
        .fetchQuery({ queryKey: ["session"], queryFn: readSession })
        .catch(() => undefined);
      const author = isData(session) ? authorSlug(session.data.account.author) : undefined;
      return createPost(title.trim(), key, author);
    },
  });

  const submit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const answer = await start.mutateAsync().catch(() => null);
    if (answer === null) {
      setProblem(en.problems.unreachable);
    } else if ("post" in answer) {
      await navigate({ to: "/posts/$id", params: { id: answer.post.id } });
      return;
    } else {
      setProblem(problemMessage(answer.problem, en.problems.unreachable));
    }
    setKey(requestKey());
  };

  return (
    <>
      {problem === null ? null : <Problem message={problem} />}
      <form className="focus" onSubmit={submit}>
        <h2 className="mt-0">{en.start.title}</h2>
        <p>{en.start.lede}</p>
        <div className="field">
          <label htmlFor="new-title">{en.start.label}</label>
          <input
            id="new-title"
            required
            value={title}
            onChange={(event) => setTitle(event.target.value)}
          />
        </div>
        <button type="submit" className="btn primary" disabled={start.isPending}>
          {start.isPending ? en.start.starting : en.start.submit}
        </button>
      </form>
    </>
  );
}
