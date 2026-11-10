import { useQueryClient } from "@tanstack/react-query";
import { useNavigate } from "@tanstack/react-router";
import { useState } from "react";
import { deleteMedia, type MediaItem } from "../api/media";
import { problemMessage } from "../api/posts";
import { isData } from "../api/redirects";
import { Problem } from "../components/Problem";
import { en, type UserMessage } from "../strings/en";

type Props = { item: MediaItem };

/**
 * Deletes a file for good, after saying what that means. The blog refuses
 * while a post, a page or the theme still uses the file, and says where.
 */
export function DeleteFile({ item }: Props) {
  const words = en.media.remove;
  const navigate = useNavigate();
  const queries = useQueryClient();
  const [busy, setBusy] = useState(false);
  const [failed, setFailed] = useState<UserMessage | null>(null);

  const remove = async () => {
    if (busy) {
      return;
    }
    setBusy(true);
    const reply = await deleteMedia(item.id, item.updated_at);
    setBusy(false);
    if (!isData(reply)) {
      const message = problemMessage(reply.problem, en.problems.unreachable);
      // The API's title for any 409 speaks of a change; this one is about use.
      const inUse = reply.problem?.rule === "uses";
      setFailed(inUse ? { ...message, what: words.inUse } : message);
      return;
    }
    // Leave first, so the page never asks again for the file that is gone.
    await navigate({ to: "/media", search: {} });
    queries.removeQueries({ queryKey: ["media-item", item.id] });
    await queries.invalidateQueries({ queryKey: ["media"] });
  };

  return (
    <section aria-labelledby="delete-file-title">
      <h2 id="delete-file-title">{words.title}</h2>
      <p className="muted">{words.lede}</p>
      {failed === null ? null : <Problem message={failed} />}
      <button type="button" className="btn" onClick={remove} disabled={busy}>
        {words.submit(item.name)}
      </button>
    </section>
  );
}
