import { useQuery, useQueryClient } from "@tanstack/react-query";
import { type FormEvent, useState } from "react";
import { categoryName, useCategories } from "../api/categories";
import { type AdminPost, problemMessage, readPost, savePost } from "../api/posts";
import { Problem } from "../components/Problem";
import { en, type UserMessage } from "../strings/en";
import { quickEditWords as words } from "../strings/quickEditWords";
import { type Basics, basicsOf, editFrom } from "./quickEdit";

type Props = {
  id: string;
  /** The authors of the posts on the page, which the author field offers. */
  authors: string[];
  /** Called with what to say once the post is saved, or closed unchanged. */
  onClose: (said: string | null) => void;
};

/** The quick edit of one post, read fresh so it starts from the copy saved now. */
export function QuickEdit({ id, authors, onClose }: Props) {
  const post = useQuery({ queryKey: ["post", id], queryFn: () => readPost(id) }).data;
  if (post === undefined) {
    return <p role="status">{words.reading}</p>;
  }
  if (!("post" in post)) {
    return <Problem message={problemMessage(post.problem, en.problems.unreachable)} />;
  }
  return <QuickForm post={post.post} authors={authors} onClose={onClose} />;
}

type TextProps = {
  name: keyof Basics;
  label: string;
  hint?: string;
  basics: Basics;
  set: (basics: Basics) => void;
};

/** A labelled text field of quick edit, with its hint when it has one. */
function Text({ name, label, hint, basics, set }: TextProps) {
  const hintId = hint === undefined ? undefined : `quick-${name}-hint`;
  return (
    <div className="field">
      <label htmlFor={`quick-${name}`}>{label}</label>
      {hint === undefined ? null : (
        <span className="hint" id={hintId}>
          {hint}
        </span>
      )}
      <input
        id={`quick-${name}`}
        aria-describedby={hintId}
        required={name === "title"}
        value={basics[name]}
        onChange={(event) => set({ ...basics, [name]: event.target.value })}
      />
    </div>
  );
}

/** The form itself: title, description, tags, category and author, saved in one edit. */
function QuickForm({ post, authors, onClose }: { post: AdminPost } & Omit<Props, "id">) {
  const queries = useQueryClient();
  const categories = useCategories();
  const [basics, setBasics] = useState(basicsOf(post));
  const [working, setWorking] = useState(false);
  const [problem, setProblem] = useState<UserMessage | null>(null);
  const writers = [...new Set([...authors, ...(post.author ? [post.author] : [])])];
  const filed = categories.map((c): [string, string] => [c.slug, categoryName(c)]);
  if (basics.category !== "" && !filed.some(([slug]) => slug === basics.category)) {
    filed.push([basics.category, basics.category]);
  }

  const submit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const edit = editFrom(post, basics);
    if (edit === null) {
      onClose(words.nothing);
      return;
    }
    setWorking(true);
    const answer = await savePost(post.id, edit);
    setWorking(false);
    if (!("post" in answer)) {
      setProblem(problemMessage(answer.problem, en.problems.unreachable));
      return;
    }
    await queries.invalidateQueries({ queryKey: ["rows"] });
    await queries.invalidateQueries({ queryKey: ["post", post.id] });
    onClose(words.saved(answer.post.title));
  };

  return (
    <form className="quick-edit" aria-label={words.label(post.title)} onSubmit={submit}>
      <p className="muted">{words.intro}</p>
      {problem === null ? null : <Problem message={problem} />}
      <Text name="title" label={words.title} basics={basics} set={setBasics} />
      <Text
        name="description"
        label={words.description}
        hint={words.descriptionHint}
        basics={basics}
        set={setBasics}
      />
      <Text name="tags" label={words.tags} hint={words.tagsHint} basics={basics} set={setBasics} />
      <div className="field">
        <label htmlFor="quick-category">{words.category}</label>
        <span className="hint" id="quick-category-hint">
          {words.categoryHint}
        </span>
        <select
          id="quick-category"
          aria-describedby="quick-category-hint"
          value={basics.category}
          onChange={(event) => setBasics({ ...basics, category: event.target.value })}
        >
          {filed.map(([slug, name]) => (
            <option key={slug} value={slug}>
              {name}
            </option>
          ))}
          <option value="">{words.noCategory}</option>
        </select>
      </div>
      <div className="field">
        <label htmlFor="quick-author">{words.author}</label>
        <select
          id="quick-author"
          value={basics.author}
          onChange={(event) => setBasics({ ...basics, author: event.target.value })}
        >
          {writers.map((name) => (
            <option key={name} value={name}>
              {name}
            </option>
          ))}
          <option value="">{words.noAuthor}</option>
        </select>
      </div>
      <div className="actions">
        <button type="submit" className="btn primary" disabled={working}>
          {working ? words.saving : words.save}
        </button>
        <button type="button" className="btn ghost" onClick={() => onClose(null)}>
          {words.close}
        </button>
      </div>
    </form>
  );
}
