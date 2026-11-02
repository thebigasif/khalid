import { useQuery } from "@tanstack/react-query";
import { Link } from "@tanstack/react-router";
import { useState } from "react";
import { type AdminPost, problemMessage, readPost, savePost } from "../api/posts";
import type { PostRow } from "../api/rows";
import { en } from "../strings/en";
import { seoToolWords } from "../strings/seoToolWords";
import { changes, lacks, type Missing, type SeoFields, seoEdit, seoFields } from "./bulkSeo";

type Props = { row: PostRow; missing: Missing | null };

/** One post of the bulk SEO editor: its SEO title, description and focus keyword. */
export function BulkSeoRow({ row, missing }: Props) {
  const read = useQuery({ queryKey: ["bulk-seo", row.id], queryFn: () => readPost(row.id) }).data;
  if (read === undefined || !("post" in read)) {
    return null;
  }
  // The copy as first read decides the filter, so a row stays put while it is filled in.
  return lacks(seoFields(read.post), missing) ? <Fields first={read.post} /> : null;
}

function Fields({ first }: { first: AdminPost }) {
  const words = seoToolWords.bulk;
  const [post, setPost] = useState(first);
  const [now, setNow] = useState<SeoFields>(seoFields(first));
  const [said, setSaid] = useState<string | null>(null);
  const edit = seoEdit(post, seoFields(post), now);
  const set = (field: keyof SeoFields) => (value: string) => {
    setNow({ ...now, [field]: value });
    setSaid(null);
  };

  const save = async () => {
    const answer = await savePost(post.id, edit);
    if ("post" in answer) {
      setPost(answer.post);
      setNow(seoFields(answer.post));
      setSaid(words.saved);
    } else {
      setSaid(problemMessage(answer.problem, en.problems.unreachable).what);
    }
  };

  // A keyword is measured in words by the linter, so it gets no character count.
  const field = (name: keyof SeoFields, label: string, most: number | null) => (
    <td data-label={label}>
      <textarea
        rows={name === "description" ? 5 : 2}
        aria-label={words.labelFor(label, post.title)}
        value={now[name]}
        onChange={(event) => set(name)(event.target.value)}
      />
      {most !== null && (
        <span className={now[name].length > most ? "hint over" : "hint"}>
          {words.length(now[name].length, most)}
        </span>
      )}
    </td>
  );

  return (
    <tr>
      <th scope="row">
        <Link to="/posts/$id" params={{ id: post.id }}>
          {post.title}
        </Link>
      </th>
      {field("title", words.seoTitle, 60)}
      {field("description", words.description, 160)}
      {field("keyword", words.keyword, null)}
      <td>
        <button
          type="button"
          className="btn"
          disabled={!changes(edit)}
          aria-label={words.saveFor(post.title)}
          onClick={save}
        >
          {words.save}
        </button>
        <span role="status" className="hint">
          {said}
        </span>
      </td>
    </tr>
  );
}
