import { useState } from "react";
import { failureMessage, isData } from "../../api/call";
import { Problem } from "../../components/Problem";
import type { UserMessage } from "../../strings/en";
import { type Offered, type Suggestion, type SuggestionRequest, suggest } from "./api";
import { aiWords } from "./words";

/** What a picked option does: fill a field, copy it, or add the block. */
export type Pick =
  | { kind: "use"; onUse: (text: string) => void }
  | { kind: "copy" }
  | { kind: "block"; onAdd: (markdown: string) => void };

type Props = {
  /** The words on the button. */
  label: string;
  /** What a screen reader says for it. */
  name: string;
  /** The request, read when the button is pressed, so it sends the latest words. */
  ask: () => SuggestionRequest;
  pick: Pick;
};

type State =
  | { kind: "idle" }
  | { kind: "asking" }
  | { kind: "shown"; offered: Offered }
  | { kind: "failed"; message: UserMessage }
  | { kind: "done"; note: string };

/**
 * An AI button and the options it brings back, each with its length
 * checked. Nothing in the post changes until the writer picks one.
 */
export function SuggestBox({ label, name, ask, pick }: Props) {
  const [state, setState] = useState<State>({ kind: "idle" });
  const open = async () => {
    setState({ kind: "asking" });
    const reply = await suggest(ask());
    setState(
      isData(reply)
        ? { kind: "shown", offered: reply.data }
        : { kind: "failed", message: failureMessage(reply) },
    );
  };
  const chosen = async (text: string) => {
    if (pick.kind === "use") {
      pick.onUse(text);
      setState({ kind: "idle" });
      return;
    }
    if (pick.kind === "block") {
      pick.onAdd(text);
      setState({ kind: "done", note: aiWords.added });
      return;
    }
    await navigator.clipboard.writeText(text).catch(() => undefined);
    setState({ kind: "done", note: aiWords.copied });
  };
  return (
    <div className="ai-suggest">
      <button
        type="button"
        className="btn ai-button"
        aria-label={name}
        aria-expanded={state.kind === "shown"}
        disabled={state.kind === "asking"}
        onClick={() => void open()}
      >
        <span className="ai-badge" aria-hidden="true">
          {aiWords.badge}
        </span>
        {label}
      </button>
      {/* A polite live region, not a second role="status": the editor's save line is the status. */}
      <p className="hint" aria-live="polite">
        {state.kind === "asking" ? aiWords.asking : null}
        {state.kind === "done" ? state.note : null}
      </p>
      {state.kind === "failed" ? <Problem message={state.message} /> : null}
      {state.kind === "shown" ? (
        <Options
          offered={state.offered}
          pick={pick}
          onPick={chosen}
          onClose={() => setState({ kind: "idle" })}
        />
      ) : null}
    </div>
  );
}

type OptionsProps = {
  offered: Offered;
  pick: Pick;
  onPick: (text: string) => Promise<void>;
  onClose: () => void;
};

const INTROS = { use: aiWords.intro, copy: aiWords.fixIntro, block: aiWords.takeawaysIntro };

/** Why an option does not fit, or its length and, for a field, that it fits. */
function lengthNote(option: Suggestion, kind: Pick["kind"]): string {
  if (option.note != null) {
    return option.note;
  }
  const count = aiWords.characters(option.characters);
  return kind === "use" ? `${count}, ${aiWords.fits}` : `${count}.`;
}

/** The options, fitting ones first, each with its length and a button to pick it. */
function Options({ offered, pick, onPick, onClose }: OptionsProps) {
  const whole = pick.kind === "block" ? (offered.markdown ?? null) : null;
  return (
    <section className="ai-options" aria-label={offered.label}>
      <p className="ai-label">{offered.label}</p>
      <p className="hint">{INTROS[pick.kind]}</p>
      <ul>
        {offered.suggestions.map((option) => (
          <li key={option.text}>
            <p>{option.text}</p>
            <p className={option.fits ? "hint" : "hint ai-misfit"}>
              {lengthNote(option, pick.kind)}
            </p>
            {whole === null ? (
              <button type="button" className="btn" onClick={() => void onPick(option.text)}>
                {pick.kind === "use" ? aiWords.use : aiWords.copy}
              </button>
            ) : null}
          </li>
        ))}
      </ul>
      <div className="ai-actions">
        {whole === null ? null : (
          <button type="button" className="btn primary" onClick={() => void onPick(whole)}>
            {aiWords.add}
          </button>
        )}
        <button type="button" className="btn" onClick={onClose}>
          {aiWords.close}
        </button>
      </div>
    </section>
  );
}
