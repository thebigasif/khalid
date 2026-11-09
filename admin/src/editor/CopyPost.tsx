import { useNavigate } from "@tanstack/react-router";
import { useState } from "react";
import { failureMessage, isData } from "../api/call";
import { duplicatePost } from "../api/seo";
import { Problem } from "../components/Problem";
import type { UserMessage } from "../strings/en";
import { seoEditorWords } from "../strings/seoEditorWords";

/** Copies the post as a new draft, then opens the copy. */
export function CopyPost({ id }: { id: string }) {
  const words = seoEditorWords;
  const navigate = useNavigate();
  const [busy, setBusy] = useState(false);
  const [problem, setProblem] = useState<UserMessage | null>(null);
  const copy = async () => {
    setBusy(true);
    const reply = await duplicatePost(id);
    setBusy(false);
    if (!isData(reply)) {
      setProblem(failureMessage(reply));
      return;
    }
    void navigate({ to: "/posts/$id", params: { id: reply.data.id } });
  };
  return (
    <div className="field">
      <p className="hint" id="copy-hint">
        {words.copyHint}
      </p>
      <button
        type="button"
        className="btn"
        disabled={busy}
        aria-describedby="copy-hint"
        onClick={() => void copy()}
      >
        {words.copy}
      </button>
      {problem === null ? null : <Problem message={problem} />}
    </div>
  );
}
